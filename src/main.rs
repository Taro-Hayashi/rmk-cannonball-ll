#![no_std]
#![no_main]

use core::cell::RefCell;

mod keymap;
mod nrf_flex;
mod vial;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::peripherals::{RNG, USBD};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, pac, rng, usb};
use keymap::{COL, ROW};
use nrf_flex::NrfFlex;
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use panic_probe as _;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::build_ble_stack;
use rmk::config::{
    BehaviorConfig, BleBatteryConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig,
    VialConfig,
};
use rmk::debounce::fast_debouncer::FastDebouncer;
use rmk::driver::bitbang_spi::BitBangSpiBus;
use rmk::driver::shared_spi::SharedSpiBus;
use rmk::futures::future::join3;
use rmk::input_device::Runnable;
use rmk::input_device::pmw3610::{Pmw3610, Pmw3610Config};
use rmk::input_device::pointing::{PointingDevice, PointingProcessor, PointingProcessorConfig};
use rmk::input_device::rotary_encoder::RotaryEncoder;
use rmk::keyboard::Keyboard;
use rmk::shift_register::ShiftRegisterMatrix;
use rmk::{HostResources, KeymapData, initialize_keymap_and_storage, run_all, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

const UNLOCK_KEYS: &[(u8, u8)] = &[(0, 0), (0, 1)];

const L2CAP_MTU: usize = 251;
const L2CAP_TXQ: u8 = 3;
const L2CAP_RXQ: u8 = 3;
const PMW3610_FORCE_AWAKE: bool = true;
const PMW3610_POLL_INTERVAL_US: u64 = 250;
const PMW3610_REPORT_HZ: u16 = 800;
const CLEAR_STORAGE_ON_BOOT: bool = cfg!(feature = "reset-storage");
const POINTING_INVERT_X: bool = cfg!(feature = "sensor-rotated-180");
const POINTING_INVERT_Y: bool = !cfg!(feature = "sensor-rotated-180");

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<'d, embassy_nrf::mode::Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_adv()
        .support_peripheral()
        .support_dle_peripheral()
        .support_phy_update_peripheral()
        .support_le_2m_phy()
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU as u16, L2CAP_MTU as u16, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

fn ble_addr() -> [u8; 6] {
    let ficr = pac::FICR;
    let high = u64::from(ficr.deviceid(1).read());
    let addr = high << 32 | u64::from(ficr.deviceid(0).read());
    let addr = addr | 0x0000_c000_0000_0000;
    unwrap!(addr.to_le_bytes()[..6].try_into())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // nRF config with DCDC
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);

    // --- MPSL (Multiprotocol Service Layer) ---
    let mpsl_p = mpsl::Peripherals::new(
        p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31,
    );
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();
    let mpsl = MPSL.init(unwrap!(mpsl::MultiprotocolServiceLayer::with_timeslots(
        mpsl_p,
        Irqs,
        lfclk_cfg,
        SESSION_MEM.init(mpsl::SessionMem::new()),
    )));
    spawner.spawn(mpsl_task(&*mpsl).unwrap());

    // --- SDC (SoftDevice Controller) ---
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23,
        p.PPI_CH24, p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    static RNG_INST: StaticCell<rng::Rng<'static, embassy_nrf::mode::Async>> = StaticCell::new();
    let rng_inst = RNG_INST.init(rng::Rng::new(p.RNG, Irqs));
    let mut rng_gen = ChaCha12Rng::from_rng(&mut *rng_inst).unwrap();
    static SDC_MEM: StaticCell<sdc::Mem<4096>> = StaticCell::new();
    let sdc_mem = SDC_MEM.init(sdc::Mem::<4096>::new());
    let sdc = unwrap!(build_sdc(sdc_p, rng_inst, mpsl, sdc_mem));

    // --- BLE stack ---
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_gen, &mut host_resources).await;

    // USB driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Internal flash (MPSL-aware)
    let flash = Flash::take(mpsl, p.NVMC);

    // --- 74HC595 matrix ---
    let row_pins = [
        Input::new(p.P0_28, Pull::Down),
        Input::new(p.P0_29, Pull::Down),
    ];
    let debouncer = FastDebouncer::new();
    let matrix_sck = Output::new(p.P0_05, Level::High, OutputDrive::Standard);
    let matrix_sdio = NrfFlex(Flex::new(p.P0_04));
    let shared_bus = {
        static SPI_BUS: StaticCell<RefCell<BitBangSpiBus<Output<'static>, NrfFlex<'static>>>> =
            StaticCell::new();
        SPI_BUS.init(RefCell::new(BitBangSpiBus::new(matrix_sck, matrix_sdio)))
    };
    let matrix_spi = SharedSpiBus::new(shared_bus);
    let matrix_cs = Output::new(p.P1_11, Level::High, OutputDrive::Standard);
    let mut matrix = ShiftRegisterMatrix::<_, _, _, _, ROW, COL>::new(
        matrix_spi,
        matrix_cs,
        row_pins,
        debouncer,
    );
    matrix.init().await;

    // --- Rotary encoders ---
    let mut enc_head = RotaryEncoder::new(
        Input::new(p.P0_02, Pull::Up),
        Input::new(p.P0_03, Pull::Up),
        0,
    );
    let mut enc_chest = RotaryEncoder::new(
        Input::new(p.P1_15, Pull::Up),
        Input::new(p.P1_14, Pull::Up),
        1,
    );
    let mut enc_leg = RotaryEncoder::new(
        Input::new(p.P1_13, Pull::Up),
        Input::new(p.P1_12, Pull::Up),
        2,
    );

    // --- PMW3610 trackball ---
    let spi = SharedSpiBus::new(shared_bus);
    let cs = Output::new(p.P0_10, Level::High, OutputDrive::Standard);
    let mot: Option<Input<'static>> = None;
    let sensor_config = Pmw3610Config {
        res_cpi: 800,
        force_awake: PMW3610_FORCE_AWAKE,
        ..Default::default()
    };
    let mut pointing_device = PointingDevice::<Pmw3610<_, _, _>>::with_poll_interval_and_report_hz(
        0,
        spi,
        cs,
        mot,
        sensor_config,
        PMW3610_POLL_INTERVAL_US,
        PMW3610_REPORT_HZ,
    );

    // --- RMK config ---
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 6,
        clear_storage: CLEAR_STORAGE_ON_BOOT,
        ..Default::default()
    };
    let ble_battery_config = BleBatteryConfig::default();
    let rmk_config = RmkConfig {
        device_config: DeviceConfig {
            vid: 0x8884,
            pid: 0x0907,
            manufacturer: "Taro Hayashi",
            product_name: "Cannonball LL",
            serial_number: "vial:f64c2b3c:000001",
        },
        vial_config: VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, UNLOCK_KEYS),
        storage_config,
        ble_battery_config,
        ..Default::default()
    };

    let mut keymap_data = KeymapData::new_with_encoder(
        keymap::get_default_keymap(),
        keymap::get_default_encoder_map(),
    );
    let mut behavior_config = BehaviorConfig::default();
    let per_key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &rmk_config.storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    let mut keyboard = Keyboard::new(&keymap);
    let mut pointing_processor = PointingProcessor::new(
        &keymap,
        PointingProcessorConfig {
            invert_x: POINTING_INVERT_X,
            invert_y: POINTING_INVERT_Y,
            swap_xy: true,
            ..Default::default()
        },
    );

    join3(
        run_all!(
            matrix,
            enc_head,
            enc_chest,
            enc_leg,
            pointing_device,
            pointing_processor
        ),
        keyboard.run(),
        run_rmk(&keymap, driver, &stack, &mut storage, rmk_config),
    )
    .await;
}
