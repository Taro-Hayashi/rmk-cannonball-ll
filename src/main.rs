#![no_std]
#![no_main]

mod keymap;
mod nrf_flex;
mod vial;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::nvmc::Nvmc;
use embassy_nrf::peripherals::USBD;
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, usb};
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
    let p = embassy_nrf::init(Default::default());

    // USB driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Internal flash
    let flash = async_flash_wrapper(Nvmc::new(p.NVMC));

    // --- Direct-pin matrix (no physical buttons) ---
    let direct_pins: [[Option<Input>; COL]; ROW] = [[None, None]];
    let debouncer = DefaultDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, SIZE>::new(direct_pins, debouncer, true);

    // --- Rotary encoder (head only) ---
    let mut enc_head = RotaryEncoder::new(
        Input::new(p.P0_02, Pull::Up),
        Input::new(p.P0_03, Pull::Up),
        0,
    );

    // --- PMW3610 trackball ---
    // SPI: SDIO=P0_04, SCK=P0_05, CS=P0_10(NFC2), MOT=P0_09(NFC1)
    let sck = Output::new(p.P0_05, Level::High, OutputDrive::Standard);
    let sdio = NrfFlex(Flex::new(p.P0_04));
    let spi = BitBangSpiBus::new(sck, sdio);
    let cs = Output::new(p.P0_10, Level::High, OutputDrive::Standard);
    let mot = Input::new(p.P0_09, Pull::Up);
    let sensor_config = Pmw3610Config { res_cpi: 1200, ..Default::default() };
    let mut pointing_device = PointingDevice::<Pmw3610<_, _, _>>::new(0, spi, cs, Some(mot), sensor_config);

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
