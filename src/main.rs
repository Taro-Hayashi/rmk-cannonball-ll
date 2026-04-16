//! Reset-bootloader firmware for Cannonball LL.
//!
//! 1. Erase rmk storage (6 sectors @ 0xA0000) and pointing-device slots
//!    (2 sectors @ 0x9E000) so the next firmware boots with its
//!    compile-time default keymap / settings.
//! 2. Write the Adafruit nRF52 bootloader UF2 magic (0x57) to GPREGRET.
//! 3. System reset — the bootloader re-enters UF2 DFU mode on reboot.

#![no_std]
#![no_main]

use core::sync::atomic::{Ordering, compiler_fence};

use cortex_m::peripheral::SCB;
use cortex_m_rt::entry;
use panic_halt as _;

// nRF52840 NVMC peripheral registers.
const NVMC_READY: *mut u32 = 0x4001_E400 as *mut u32;
const NVMC_CONFIG: *mut u32 = 0x4001_E504 as *mut u32;
const NVMC_ERASEPAGE: *mut u32 = 0x4001_E508 as *mut u32;

// POWER.GPREGRET — retained across soft reset and read by the bootloader.
const POWER_GPREGRET: *mut u32 = 0x4000_051C as *mut u32;

const NVMC_CONFIG_REN: u32 = 0;
const NVMC_CONFIG_EEN: u32 = 2;

// Adafruit nRF52 bootloader: 0x57 = enter UF2 DFU on next boot.
const DFU_MAGIC_UF2_RESET: u32 = 0x57;

const PAGE_SIZE: u32 = 0x1000;
const ERASE_START: u32 = 0x9_E000;
const ERASE_END: u32 = 0xA_6000;

fn wait_ready() {
    unsafe {
        while core::ptr::read_volatile(NVMC_READY) & 1 == 0 {}
    }
}

fn erase_page(addr: u32) {
    unsafe {
        core::ptr::write_volatile(NVMC_CONFIG, NVMC_CONFIG_EEN);
        wait_ready();
        core::ptr::write_volatile(NVMC_ERASEPAGE, addr);
        wait_ready();
        core::ptr::write_volatile(NVMC_CONFIG, NVMC_CONFIG_REN);
        wait_ready();
    }
}

#[entry]
fn main() -> ! {
    let mut addr = ERASE_START;
    while addr < ERASE_END {
        erase_page(addr);
        addr += PAGE_SIZE;
    }

    unsafe {
        core::ptr::write_volatile(POWER_GPREGRET, DFU_MAGIC_UF2_RESET);
    }
    compiler_fence(Ordering::SeqCst);

    SCB::sys_reset();
}
