#![no_std]
#![no_main]

mod keymap;
mod nrf_flex;
mod vial;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::nvmc::Nvmc;
use embassy_nrf::peripherals::USBD;
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, usb};
use embassy_time::Timer;
use keymap::{COL, ROW, SIZE};
use nrf_flex::NrfFlex;
use panic_probe as _;
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::direct_pin::DirectPinMatrix;
use rmk::driver::bitbang_spi::BitBangSpiBus;
use rmk::futures::future::join3;
use rmk::input_device::Runnable;
use rmk::input_device::pmw3610::{Pmw3610, Pmw3610Config};
use rmk::input_device::pointing::PointingDevice;
use rmk::input_device::rotary_encoder::RotaryEncoder;
use rmk::keyboard::Keyboard;
use rmk::storage::async_flash_wrapper;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all, run_rmk};
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    CLOCK_POWER => usb::vbus_detect::InterruptHandler;
});

const UNLOCK_KEYS: &[(u8, u8)] = &[(0, 0), (0, 1)];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Cannonball LL RMK start (minimal debug)");

    let p = embassy_nrf::init(Default::default());

    // USB driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Internal flash
    let flash = async_flash_wrapper(Nvmc::new(p.NVMC));

    // --- Direct-pin matrix (2 keys, temporary) ---
    let direct_pins: [[Option<Input>; COL]; ROW] = [[
        Some(Input::new(p.P0_28, Pull::Up)),
        Some(Input::new(p.P0_29, Pull::Up)),
    ]];
    let debouncer = DefaultDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, SIZE>::new(direct_pins, debouncer, true);

    // --- Rotary encoder (head only, for sanity check) ---
    let mut enc_head = RotaryEncoder::new(
        Input::new(p.P0_02, Pull::Up),
        Input::new(p.P0_03, Pull::Up),
        0,
    );

    // --- PMW3610 trackball ---
    // Disable XIAO Sense microphone (P1_08 = MIC_PWR) before using P0_16 as SDIO
    let _mic_pwr = Output::new(p.P1_08, Level::Low, OutputDrive::Standard);
    Timer::after_millis(1).await;

    // SPI: SCK=P1_10, SDIO=P0_16, CS=P0_10(NFC2)
    let sck = Output::new(p.P1_10, Level::High, OutputDrive::Standard);
    let sdio = NrfFlex(Flex::new(p.P0_16));
    let mut spi = BitBangSpiBus::new(sck, sdio);
    let mut cs = Output::new(p.P0_10, Level::High, OutputDrive::Standard);

    // --- SPI communication test: read Product ID (reg 0x00, expect 0x3E) ---
    info!("Reading PMW3610 Product ID...");
    Timer::after_millis(50).await; // power-up delay
    let pid = spi_read_register(&mut spi, &mut cs, 0x00).await;
    info!("PMW3610 Product ID: {:#04x}", pid);
    if pid == 0x3E {
        info!("PMW3610 SPI OK!");
    } else {
        info!("PMW3610 SPI NG — check wiring/NFC pin config");
    }

    // Proceed with normal operation (polling mode)
    let mot: Option<Input<'static>> = None;
    let sensor_config = Pmw3610Config { res_cpi: 1200, ..Default::default() };
    let mut pointing_device = PointingDevice::<Pmw3610<_, _, _>>::new(0, spi, cs, mot, sensor_config);

    // --- RMK config ---
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 6,
        ..Default::default()
    };
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

    join3(
        run_all!(matrix, enc_head, pointing_device),
        keyboard.run(),
        run_rmk(&keymap, driver, &mut storage, rmk_config),
    )
    .await;
}

/// Manual SPI read for debug: read one register via bit-bang half-duplex SPI
async fn spi_read_register(
    spi: &mut BitBangSpiBus<Output<'_>, NrfFlex<'_>>,
    cs: &mut Output<'_>,
    reg: u8,
) -> u8 {
    use embedded_hal_async::spi::SpiBus;

    cs.set_low();
    Timer::after_micros(1).await;

    // Send read command (bit 7 = 0 for read)
    let cmd = [reg & 0x7F];
    let _ = SpiBus::write(spi, &cmd).await;

    // t_SRAD: wait for data ready (PMW3610 datasheet: ~4us)
    Timer::after_micros(5).await;

    let mut buf = [0u8; 1];
    let _ = SpiBus::read(spi, &mut buf).await;

    cs.set_high();
    Timer::after_micros(2).await;

    buf[0]
}
