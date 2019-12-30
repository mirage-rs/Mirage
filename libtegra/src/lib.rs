//! Low-level hardware access library for the Nintendo Switch.
//!
//! **Note:** This code is written specifically for the Switch.
//! If you decide to use it for other Tegra210 platforms, use
//! at own risk.

#![no_std]
#![feature(const_fn)]
#![feature(optimize_attribute)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate enum_primitive;

extern crate mirage_mmio;

extern crate paste;

pub mod apb_misc;
pub mod button;
pub mod clock;
pub mod cluster;
pub mod display;
pub mod fuse;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod mc;
pub mod pinmux;
pub mod pmc;
pub mod power;
pub mod rtc;
pub mod sdmmc;
pub mod sdram;
pub mod se;
pub mod sysctr0;
pub mod sysreg;
pub mod timer;
pub mod tsec;
pub mod uart;
