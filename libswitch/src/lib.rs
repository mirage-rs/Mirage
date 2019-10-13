//! Low-level hardware access library for the Nintendo Switch.
//!
//! **Note:** This code is written specifically for the Switch.
//! If you decide to use it for other Tegra210 platforms, use
//! at own risk.

#![no_std]
#![feature(optimize_attr)]

#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate enum_primitive;

extern crate register;

pub mod button;
pub mod clock;
pub mod fuse;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod pinmux;
pub mod timer;
pub mod tsec;
pub mod uart;
