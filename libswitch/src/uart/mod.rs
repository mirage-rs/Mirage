//! Tegra210 UART driver.

use core::fmt::{Error, Write};
use core::marker::{Send, Sync};

use register::mmio::*;
use register::FieldValue;

use crate::clock::Clock;
use crate::timer::usleep;

const UART_LSR_RDR: u32 = (1 << 0);
const UART_LSR_THRE: u32 = (1 << 5);
const UART_LSR_TMTY: u32 = (1 << 6);

const UART_LCR_WORD_LENGTH_8: u32 = 3;
const UART_LCR_DLAB: u32 = (1 << 7);

const UART_FCR_FCR_EN_FIFO: u32 = (1 << 0);
const UART_FCR_RX_CLR: u32 = (1 << 1);
const UART_FCR_TX_CLR: u32 = (1 << 2);

/// Representation of the UART registers.
#[repr(C)]
pub struct UartRegisters {
    thr_dlab: ReadWrite<u32>,
    ier_dlab: ReadWrite<u32>,
    iir_fcr: ReadWrite<u32>,
    lcr: ReadWrite<u32>,
    mcr: ReadWrite<u32>,
    lsr: ReadWrite<u32>,
    msr: ReadWrite<u32>,
    spr: ReadWrite<u32>,
    irda_csr: ReadWrite<u32>,
    rx_fifo_cfg: ReadWrite<u32>,
    mie: ReadWrite<u32>,
    vendor_status: ReadWrite<u32>,
    unk: [u8; 0xC],
    asr: ReadWrite<u32>,
}

/// Representation of a UART device.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UartDevice {
    pub registers: *const UartRegisters,
    pub clock: &'static Clock,
}

unsafe impl Send for UartDevice {}

unsafe impl Sync for UartDevice {}

impl UartDevice {
    pub const A: Self = UartDevice {
        registers: 0x7000_6000 as *const UartRegisters,
        clock: &Clock::UART_A,
    };

    pub const B: Self = UartDevice {
        registers: 0x7000_6040 as *const UartRegisters,
        clock: &Clock::UART_B,
    };

    pub const C: Self = UartDevice {
        registers: 0x7000_6200 as *const UartRegisters,
        clock: &Clock::UART_C,
    };

    pub const D: Self = UartDevice {
        registers: 0x7000_6300 as *const UartRegisters,
        clock: &Clock::UART_D,
    };

    pub const E: Self = UartDevice {
        registers: 0x7000_6400 as *const UartRegisters,
        clock: &Clock::UART_E,
    };
}

impl UartDevice {
    #[inline]
    fn wait_cycles(&self, baud: u32, amount: u32) {
        usleep((amount * 1_000_000 + 16 * baud - 1) / (16 * baud));
    }

    #[inline]
    fn wait_symbols(&self, baud: u32, amount: u32) {
        usleep((amount * 1_000_000 + baud - 1) / baud);
    }

    #[inline]
    fn wait_idle(&self, status: u32) {
        let lsr_reg = unsafe { &((*self.registers).lsr) };

        while (lsr_reg.get() & value) == 0 {}
    }

    #[inline]
    fn wait_transmit(&self) {
        let lsr_reg = unsafe { &((*self.registers).lsr) };

        while (lsr_reg.get() & UART_LSR_THRE) == 0 {}
    }

    #[inline]
    fn wait_receive(&self) {
        let lsr_reg = unsafe { &((*self.registers).lsr) };

        while (lsr_reg.get() & UART_LSR_RDR) == 0 {}
    }

    /// Initializes the device.
    pub fn init(&self, baud: u32) {
        // Enable device clock.
        self.clock.enable();

        // Wait for TX idle state.
        self.wait_idle(UART_LSR_TMTY);

        let rate = (8 * baud + 408_000_000) / (16 * baud);

        let uart_base = unsafe { &(*self.registers) };

        // Disable interrupts.
        uart_base.ier_dlab.set(0);

        // No hardware flow control.
        uart_base.mcr.set(0);

        // Enable DLAB and set word length to 8.
        uart_base.lcr.set(UART_LCR_DLAB | UART_LCR_WORD_LENGTH_8);

        uart_base.thr_dlab.set(rate);
        uart_base.ier_dlab.set(rate >> 8);

        // Disable DLAB.
        uart_base.lcr.set(uart_base.lcr.get() & !UART_LCR_DLAB);

        uart_base.spr.get(); // Dummy read.
        self.wait_symbols(baud, 3); // Wait for 3 symbols.

        // Enable FIFO.
        uart_base.iir_fcr.set(UART_FCR_FCR_EN_FIFO);
        uart_base.spr.get(); // Dummy read.
        self.wait_cycles(baud, 3); // Wait for 3 baud cycles.

        // Flush FIFO.
        self.wait_idle(UART_LSR_TMTY); // Ensure no data is being written to TX FIFO.
        uart_base
            .iir_fcr
            .set(uart_base.iir_fcr.get() | UART_FCR_RX_CLR | UART_FCR_TX_CLR); // Clear TX and RX FIFOs.
        self.wait_cycles(baud, 32); // Wait for 32 baud cycles.

        // Wait for idle state.
        self.wait_idle(UART_LSR_TMTY);
        self.wait_idle(UART_LSR_RDR);
    }

    /// Sends an `u32`.
    pub fn write_u32(&self, value: u32) {
        let mut digits: [u8; 10] = [0x0; 10];
        let mut value = value;

        for i in digits.iter_mut() {
            *i = ((value % 10) + 0x30) as u8;

            value /= 10;

            if value == 0 {
                break;
            }
        }

        for digit in digits.iter().rev() {
            self.write(&[*digit]);
        }
    }

    /// Sends an `u64`.
    pub fn write_u64(&self, value: u64) {
        let mut digits: [u8; 20] = [0x0; 20];
        let mut value = value;

        for i in digits.iter_mut() {
            *i = ((value % 10) + 0x30) as u8;

            value /= 10;

            if value == 0 {
                break;
            }
        }

        for digit in digits.iter().rev() {
            self.write(&[*digit]);
        }
    }

    /// Sends a buffer of `u8` data.
    pub fn write(&self, data: &[u8]) {
        let thr_reg = unsafe { &((*self.registers).thr_dlab) };

        for byte in data {
            // Wait until it is possible to send data.
            self.wait_transmit();

            // Send data.
            thr_reg.set(u32::from(*byte));
        }
    }

    /// Reads and returns a byte (`u8`).
    pub fn read_byte(&self) -> u8 {
        let thr_reg = unsafe { &((*self.registers).thr_dlab) };

        // Wait until it is possible to receive data.
        self.wait_receive();

        // Read byte.
        thr_reg.get() as u8
    }

    /// Reads bytes into a buffer.
    pub fn read(&self, buffer: &mut [u8]) {
        for i in buffer.iter_mut() {
            *i = self.read_byte();
        }
    }
}

impl Write for UartDevice {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write(s.as_bytes());

        // Wait for everything to be written.
        self.wait_transmit();

        Ok(())
    }
}
