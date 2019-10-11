//! Driver to read inputs from physical buttons on the Switch console.

use crate::gpio::GpioPin;
use crate::i2c::*;
use crate::timer::get_milliseconds;

bitflags! {
    /// Physical buttons of the Switch console.
    pub struct Button: u32 {
        const POWER = 0b1;
        const VOL_DOWN = 0b10;
        const VOL_UP = 0b100;
    }
}

/// Reads a physical button input.
pub fn read() -> Button {
    let mut result = Button::empty();

    if !GpioPin::BUTTON_VOL_DOWN.read() {
        result |= Button::VOL_DOWN;
    }

    if !GpioPin::BUTTON_VOL_UP.read() {
        result |= Button::VOL_UP;
    }

    let mut buffer: [u8; 1] = [0; 1];
    if I2cDevice::I5.read(MAX77620_PWR_I2C_ADDR, 0x15, &buffer) {
        if buffer[0] & 0x4 {
            result |= Button::POWER;
        }
    }

    result
}

/// Waits for a physical button input.
pub fn wait() -> Button {
    let mut result = Button::empty();
    let mut pwr = false;
    let mut btn = read();

    if btn & Button::POWER {
        pwr = true;
        btn &= !Button::POWER;
    }

    loop {
        result = read();

        if !(result & Button::POWER) && pwr {
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
