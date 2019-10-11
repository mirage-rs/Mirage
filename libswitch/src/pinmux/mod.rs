//! Pin Multiplexer (Pinmux) configurations for various I/O controllers.

use register::mmio::WriteOnly;

use crate::i2c::I2cDevice;
use crate::uart::UartDevice;

const PINMUX_BASE: u32 = 0x7000_3000;

const PINMUX_PULL_NONE: u32 = (0 << 2);
const PINMUX_PULL_DOWN: u32 = (1 << 2);
const PINMUX_PULL_UP: u32 = (2 << 2);

const PINMUX_TRISTATE: u32 = (1 << 4);
const PINMUX_PARKED: u32 = (1 << 5);
const PINMUX_INPUT: u32 = (1 << 6);
const PINMUX_LOCK: u32 = (1 << 7);
const PINMUX_LPDR: u32 = (1 << 8);
const PINMUX_HSM: u32 = (1 << 9);

/// Configures an UART device.
pub fn configure_uart(device: UartDevice) {
    let value = match device {
        UartDevice::A => 0,
        UartDevice::B => 1,
        UartDevice::C => 2,
        UartDevice::D => 3,
        UartDevice::E => 4,
    };

    let tx_reg = unsafe { &(*((PINMUX_BASE + 0xE4 + 0x10 * value) as *const WriteOnly<u32>)) };
    let rx_reg = unsafe { &(*((PINMUX_BASE + 0xE8 + 0x10 * value) as *const WriteOnly<u32>)) };
    let rts_reg = unsafe { &(*((PINMUX_BASE + 0xEC + 0x10 * value) as *const WriteOnly<u32>)) };
    let cts_reg = unsafe { &(*((PINMUX_BASE + 0xF0 + 0x10 * value) as *const WriteOnly<u32>)) };

    tx_reg.set(0);
    rx_reg.set(PINMUX_INPUT | PINMUX_PULL_UP);
    rts_reg.set(0);
    cts_reg.set(PINMUX_INPUT | PINMUX_PULL_DOWN);
}

/// Configures an I²C device.
pub fn configure_i2c(device: I2cDevice) {
    let value = match device {
        I2cDevice::I1 => 0,
        I2cDevice::I2 => 1,
        I2cDevice::I3 => 2,
        I2cDevice::I4 => 3,
        I2cDevice::I5 => 4,
        I2cDevice::I6 => 5,
    };

    let scl_reg = unsafe { &(*((PINMUX_BASE + 0xBC + 8 * value) as *const WriteOnly<u32>)) };
    let sda_reg = unsafe { &(*((PINMUX_BASE + 0xC0 + 8 * value) as *const WriteOnly<u32>)) };

    scl_reg.set(PINMUX_INPUT);
    sda_reg.set(PINMUX_INPUT);
}
