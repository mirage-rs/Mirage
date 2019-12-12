//! Mirage Bootstrap
//!
//! This is the first-stage bootloader, responsible for initializing
//! the hardware and chainloading the second-stage bootloader.
//!
//! # Memory
//!
//! Execution starts at `0x40010000`, which is the stack top of the
//! Boot and Power Management Processor of the Tegra X1. It has a
//! length of `0x20000`.
//!
//! The low IRAM is located at `0x40003000`, right where the execution
//! stack ends, with a length of `0x8000`. When chainloading other RCM
//! payloads, this is where they are being loaded.
//!
//! # Tasks
//!
//! Being injected as a baremetal ARM payload through the CVE-2018-6242
//! ("Fusée Gelée") vulnerability, we are still at a very early bootrom
//! stage.
//!
//! That's why there are several things for us to do:
//!
//! * Initialize the hardware
//!
//! * Execute the skipped bootrom part
//!
//! * Find and load stage 2
//!
//! * Clean up the resources

#![no_std]
#![no_main]
#![feature(global_asm)]

// Bootstrap should be executed on the BPMP CPU.
#[cfg(not(any(target_arch = "arm", rustdoc, test)))]
compile_error!("No!");

// Load the first bootstrap stage from Assembly.
global_asm!(include_str!("start.S"));

#[macro_use]
extern crate mirage_libswitch;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    // TODO: Implement a proper panic handler.
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    // TODO: Implement the bootloader.
}
