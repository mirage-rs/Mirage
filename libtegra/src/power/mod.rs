//! Drivers for Nintendo Switch power components.

use crate::i2c::{I2c, Error, Device};

pub mod max77620;

/// Sets a bit in a PMIC register over I²C during CPU shutdown.
#[inline]
pub fn send_pmic_cpu_shutdown_cmd() -> Result<(), Error> {
    // PMIC == Device 4:3C.
    let value = I2c::C5.read_byte(Device::Max77620Pwr, 0x41)?;

    I2c::C5.write_byte(Device::Max77620Pwr, 0x41, value | 4)
}

/// Reads the value of TI charger bit over I²C.
#[inline]
pub fn read_ti_charger_bit_7() -> Result<bool, Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(Device::Bq24193, 0).unwrap();

    Ok((value & 0x80) != 0)
}

/// Clears TI charger bit over I²C.
#[inline]
pub fn clear_ti_charger_bit_7() -> Result<(), Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(Device::Bq24193, 0)?;

    I2c::C1.write_byte(Device::Bq24193, 0, value & 0x7F)
}

/// Sets TI charger bit over I²C.
#[inline]
pub fn set_ti_charger_bit_7() -> Result<(), Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(Device::Bq24193, 0)?;

    I2c::C1.write_byte(Device::Bq24193, 0, value | 0x80)
}
