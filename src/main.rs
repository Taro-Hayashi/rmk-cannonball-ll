#![no_std]
#![no_main]

mod keymap;
mod vial;

use core::convert::Infallible;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::nvmc::Nvmc;
use embassy_nrf::peripherals::USBD;
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, usb};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use keymap::{COL, ROW, SIZE};
use panic_probe as _;
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::direct_pin::DirectPinMatrix;
use rmk::driver::bitbang_spi::BitBangSpiBus;
use rmk::driver::flex_pin::FlexPin;
use rmk::futures::future::join3;
use rmk::input_device::Runnable;
use rmk::input_device::pmw3610::{Pmw3610, Pmw3610Config};
use rmk::input_device::pointing::PointingDevice;
use rmk::keyboard::Keyboard;
use rmk::storage::async_flash_wrapper;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all, run_rmk};
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    CLOCK_POWER => usb::vbus_detect::InterruptHandler;
});

const UNLOCK_KEYS: &[(u8, u8)] = &[(0, 0), (0, 1)];

/// Newtype wrapper so we can implement FlexPin for embassy_nrf::gpio::Flex
/// (orphan rule prevents implementing foreign traits on foreign types directly)
struct NrfFlex<'d>(Flex<'d>);

impl<'d> ErrorType for NrfFlex<'d> {
    type Error = Infallible;
}

impl<'d> InputPin for NrfFlex<'d> {
    fn is_high(&mut self) -> Result<bool, Infallible> { Ok(self.0.is_high()) }
    fn is_low(&mut self) -> Result<bool, Infallible> { Ok(self.0.is_low()) }
}

impl<'d> OutputPin for NrfFlex<'d> {
    fn set_high(&mut self) -> Result<(), Infallible> { self.0.set_high(); Ok(()) }
    fn set_low(&mut self) -> Result<(), Infallible> { self.0.set_low(); Ok(()) }
}

impl<'d> FlexPin for NrfFlex<'d> {
    fn set_as_input(&mut self) { self.0.set_as_input(Pull::Down); }
    fn set_as_output(&mut self) {
        self.0.set_level(Level::Low);
        self.0.set_as_output(OutputDrive::Standard);
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Cannonball LL RMK start!");

    let p = embassy_nrf::init(Default::default());

    // USB driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Internal flash for key storage
    let flash = async_flash_wrapper(Nvmc::new(p.NVMC));

    // --- Direct-pin matrix (2 keys, temporary until 595 implementation) ---
    let direct_pins: [[Option<Input>; COL]; ROW] = [[
        Some(Input::new(p.P0_28, Pull::Up)),
        Some(Input::new(p.P0_29, Pull::Up)),
    ]];
    let debouncer = DefaultDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, SIZE>::new(direct_pins, debouncer, true);

    // --- PMW3610 trackball ---
    // SCK=P1_13 (D8), SDIO=P1_15 (D10) — standard header pins (non-microphone)
    // CS=P0_10 (NFC2), MOT=P0_09 (NFC1) — enabled by nfc-pins-as-gpio feature
    let sck = Output::new(p.P1_13, Level::High, OutputDrive::Standard);
    let sdio = NrfFlex(Flex::new(p.P1_15));
    let spi = BitBangSpiBus::new(sck, sdio);
    let cs = Output::new(p.P0_10, Level::High, OutputDrive::Standard);
    let mot = Input::new(p.P0_09, Pull::Up);

    let sensor_config = Pmw3610Config {
        res_cpi: 1200,
        ..Default::default()
    };
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

    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
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
        run_all!(matrix, pointing_device),
        keyboard.run(),
        run_rmk(&keymap, driver, &mut storage, rmk_config),
    )
    .await;
}
