//! Driver to read inputs from physical buttons on the Switch console.
//!
//! # Implementation
//!
//! - The bitflags struct [`Button`] holds the values to identify physical
//! buttons of the console.
//!
//! - The functions [`read`], [`wait`] and [`wait_for`] can be used to
//! get input, they however differ in functionality.
//!
//! - [`read`] tries to get input immediately and returns the bitmask.
//!
//! - [`wait`] waits until a button was pressed and returns the bitmask.
//!
//! - [`wait_for`] waits for a given duration to read the provided
//! bitmask and returns a [`Result`] with the bitmask or `()` in
//! case the function has timed out.
//!
//! # Example
//!
//! ```
//! use mirage_libtegra::button::*;
//!
//! fn main() {
//!     // Wait for 10 seconds to get the key combination for entering RCM.
//!     let button = wait_for(10, Button::POWER | Button::VOL_UP)
//!         .unwrap_or_else(|_| panic!("Key combination for entering RCM wasn't pressed in time!"));
//! }
//! ```
//!
//! [`Button`]: struct.Button.html
//! [`read`]: fn.read.html
//! [`wait`]: fn.wait.html
//! [`wait_for`]: fn.wait_for.html
//! [`Result`]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html

use crate::{
    gpio::{Gpio, GpioLevel},
    i2c::*,
    timer::get_seconds,
};

bitflags! {
    /// Physical buttons of the Switch console.
    pub struct Button: u32 {
        /// The power button.
        const POWER = 0b1;
        /// The Volume Down button.
        const VOL_DOWN = 0b10;
        /// The Volume Up button.
        const VOL_UP = 0b100;
    }
}

/// Reads a physical button input.
pub fn read() -> Button {
    let mut result = Button::empty();

    if Gpio::BUTTON_VOL_DOWN.read() == GpioLevel::Low {
        result |= Button::VOL_DOWN;
    }

    if Gpio::BUTTON_VOL_UP.read() == GpioLevel::Low {
        result |= Button::VOL_UP;
    }

    if I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, 0x15).unwrap() & 0x4 != 0 {
        result |= Button::POWER;
    }

    result
}

/// Waits for a physical button input.
pub fn wait() -> Button {
    let mut result = Button::empty();
    let mut pwr = false;
    let mut btn = read();

    if btn.contains(Button::POWER) {
        pwr = true;
        btn &= !Button::POWER;
    }

    loop {
        result = read();

        if !result.contains(Button::POWER) && pwr {
            pwr = false;
        } else if pwr {
            result &= !Button::POWER;
        }

        if btn != result {
            break;
        }
    }

    result
}

/// Waits for physical button input equal to the bitmask for a given time.
pub fn wait_for(seconds: u32, mask: Button) -> Result<Button, ()> {
    let timeout = get_seconds() + seconds;

    let mut result;
    while get_seconds() < timeout {
        result = read() & mask;

        if result.contains(mask) {
            return Ok(result);
        }
    }

    Err(())
}
