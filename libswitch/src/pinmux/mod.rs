//! Pin Multiplexer (Pinmux) configurations for various I/O controllers.

use register::mmio::WriteOnly;

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
