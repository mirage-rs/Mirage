//! Universal Asynchronous Receiver/Transmitter driver for Tegra210.
//!
//! # Description
//!
//! There are four UARTs built into Tegra X1 devices.
//! These UARTs support both 16450 and 16550 compatible modes
//! (defaults to 16450).
//! A fifth UART is located in the Audio Processing Engine (APE).
//!
//! Those UARTs are identical and provide serial data synchronization
//! and data conversion for both receiver and transmitter sections.
//!
//! UARTs support device clocks of up to 200 MHz. Each symbol requires
//! 16 clock cycles for proper sampling and processing of the input data.
//! Thus, the maximum baud rate is `200 / 16 = 12.5M`.
//!
//! # Implementation
//!
//! - The bitflag structs [`FifoControl`], [`InterruptIdentification`],
//! [`LineControl`], [`LineStatus`], [`VendorStatus`] are abstractions
//! over possible values in these UART registers.
//!
//! - The [`Registers`] struct along with its factory methods provide
//! abstractions over the UART registers and the possibility to create
//! pointers to each UART mapped at a different address.
//!
//! - The [`Uart`] struct is an abstraction over a UART which holds
//! the corresponding [`Clock`] for enabling the device and the
//! [`Registers`] block to do communication.
//!
//! - [`Uart`] holds pre-defined constants which represent the UARTs
//! A through E and should be preferred over creating instances of
//! the [`Uart`] struct manually.
//!
//! - [`Uart::init`] has to be called for each device before it can
//! be used.
//!
//! - [`Uart::read`] and [`Uart::write`] are the recommended methods
//! for communicating over UART. For writing data, using the methods
//! exposed by the [`Write`] trait are however preferred if you're
//! transmitting strings.
//!
//! - The [`Send`] and [`Sync`] traits are implemented for [`Uart`],
//! instances and its references can be shared safely between thread
//! boundaries.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::uart::Uart;
//!
//! fn main() {
//!     let mut device = &mut Uart::A;
//!
//!     device.init(115_200);
//!     writeln!(&mut device, "Hello, friend!").ok();
//! }
//! ```
//!
//! [`FifoControl`]: enum.FifoControl.html
//! [`InterruptIdentification`]: enum.InterruptIdentification.html
//! [`LineControl`]: enum.LineControl.html
//! [`LineStatus`]: enum.LineStatus.html
//! [`VendorStatus`]: enum.VendorStatus.html
//! [`Registers`]: struct.Registers.html
//! [`Uart`]: struct.Uart.html
//! [`Clock`]: ../clock/struct.Clock.html
//! [`Uart::init`]: struct.Uart.html#method_init
//! [`Uart::read`]: struct.Uart.html#method_read
//! [`Uart::write`]: struct.Uart.html#method_write
//! [`Write`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html
//! [`Send`]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html
//! [`Sync`]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html

use core::{
    fmt::{Error, Write},
    marker::{Send, Sync},
};

use register::mmio::ReadWrite;

use crate::{clock::Clock, timer::usleep};

/// Base address for the UART A registers.
const UART_A_BASE: u32 = 0x7000_6000;
/// Base address for the UART B registers.
const UART_B_BASE: u32 = 0x7000_6040;
/// Base address for the UART C registers.
const UART_C_BASE: u32 = 0x7000_6200;
/// Base address for the UART D registers.
const UART_D_BASE: u32 = 0x7000_6300;
/// Base address for the UART E registers.
const UART_E_BASE: u32 = 0x7000_6400;

bitflags! {
    /// Representation of the `UART_IIR_FCR_0` register.
    ///
    /// This register is used for FIFO control operations.
    pub struct FifoControl: u32 {
        /// Enable the transmit and receive FIFOs. This bit should be enabled.
        const FCR_EN_FIFO = 1 << 0;
        /// Clears the contents of the receive FIFO and resets its counter logic to 0
        /// (the receive shift register is not cleared or altered).
        /// This bit returns to 0 after clearing the FIFOs.
        const RX_CLR = 1 << 1;
        /// Clears the contents of the transmit FIFO and resets its counter logic to 0
        /// (the transmit shift register is not cleared or altered).
        /// This bit returns to 0 after clearing the FIFOs.
        const TX_CLR = 1 << 2;

        /// DMA:
        /// 0 = DMA_MODE_0.
        /// 1 = DMA_MODE_1.
        const DMA = 1 << 3;

        /// TX_TRIG:
        /// 0 = FIFO_COUNT_GREATER_16.
        /// 1 = FIFO_COUNT_GREATER_8.
        /// 2 = FIFO_COUNT_GREATER_4.
        /// 3 = FIFO_COUNT_GREATER_1.
        const TX_TRIG = 3 << 4;
        const TX_TRIG_FIFO_COUNT_GREATER_16 = 0 << 4;
        const TX_TRIG_FIFO_COUNT_GREATER_8 = 1 << 4;
        const TX_TRIG_FIFO_COUNT_GREATER_4 = 2 << 4;
        const TX_TRIG_FIFO_COUNT_GREATER_1 = 3 << 4;

        /// RX_TRIG:
        /// 0 = FIFO_COUNT_GREATER_16.
        /// 1 = FIFO_COUNT_GREATER_8.
        /// 2 = FIFO_COUNT_GREATER_4.
        /// 3 = FIFO_COUNT_GREATER_1.
        const RX_TRIG = 3 << 6;
        const RX_TRIG_FIFO_COUNT_GREATER_16 = 0 << 6;
        const RX_TRIG_FIFO_COUNT_GREATER_8 = 1 << 6;
        const RX_TRIG_FIFO_COUNT_GREATER_4 = 2 << 6;
        const RX_TRIG_FIFO_COUNT_GREATER_1 = 3 << 6;
    }
}

bitflags! {
    /// Representation of the `UART_IIR_FCR_0` register.
    ///
    /// This register is also used for interrupt identification.
    pub struct InterruptIdentification: u32 {
        /// Interrupt pending if ZERO.
        const IS_STA = 1 << 0;
        /// Encoded interrupt ID.
        const IS_PRI0 = 1 << 1;
        /// Encoded interrupt ID.
        const IS_PRI1 = 1 << 2;
        /// Encoded interrupt ID.
        const IS_PRI2 = 1 << 3;

        /// FIFO Mode Status.
        /// 0 = MODE_16450 (no FIFO).
        /// 1 = MODE_16550 (FIFO).
        const EN_FIFO = 3 << 6;
        const MODE_16450 = 0 << 6;
        const MODE_16550 = 1 << 6;
    }
}

bitflags! {
    /// Representation of the `UART_LCR_0` register.
    ///
    /// This register denotes the UART Line Control Register,
    /// which is used for setting various transfer options.
    pub struct LineControl: u32 {
        /// Word length of 5.
        const WORD_LENGTH_5 = 0;
        /// Word length of 6.
        const WORD_LENGTH_6 = 1;
        /// Word length of 7.
        const WORD_LENGTH_7 = 2;
        /// Word length of 8.
        const WORD_LENGTH_8 = 3;

        /// STOP:
        /// 0 = Transmit 1 stop bit.
        /// 1 = Transmit 2 stop bits (receiver always checks for 1 stop bit).
        const STOP = 1 << 2;
        /// No parity sent.
        const PAR = 1 << 3;
        /// Even parity format.
        /// There will always be an even number of 1s in the parity representation.
        const EVEN = 1 << 4;
        /// Set (force) parity to value in `LCR`.
        const SET_P = 1 << 5;
        /// Set BREAK condition -- Transmitter sends all zeroes to indicate BREAK.
        const SET_B = 1 << 6;
        /// Divisor Latch Access Bit (set to allow programming of the DLH, DLM Divisors).
        const DLAB = 1 << 7;
    }
}

bitflags! {
    /// Representation of the `UART_LSR_0` register.
    ///
    /// This register indicates the UART line status which is useful
    /// for figuring out the state of data transfer progress.
    pub struct LineStatus: u32 {
        /// Receiver Data Ready.
        const RDR = 1 << 0;
        /// Receiver Overrun Error.
        const OVRF = 1 << 1;
        /// Parity Error.
        const PERR = 1 << 2;
        /// Framing Error.
        const FERR = 1 << 3;
        /// BREAK condition detected on line.
        const BRK = 1 << 4;
        /// Transmit Holding Register is Empty -- OK to write data.
        const THRE = 1 << 5;
        /// Transmit Shift Register empty status.
        const TMTY = 1 << 6;
        /// Receive FIFO error.
        const FIFOE = 1 << 7;
        /// Transmitter FIFO full status.
        const TX_FIFO_FULL = 1 << 8;
        /// Receiver FIFO empty status.
        const RX_FIFO_EMPTY = 1 << 9;
    }
}

bitflags! {
    /// Representation of the `UART_VENDOR_STATUS_0_0` register.
    ///
    /// This register is used to acquire status data on the
    /// RX and TX FIFOs.
    pub struct VendorStatus: u32 {
        /// This bit is set to 1 when the TX path is IDLE.
        const UART_TX_IDLE = 1 << 0;
        /// This bit is set to 1 when the RX path is IDLE.
        const UART_RX_IDLE = 1 << 1;

        /// This bit is set to 1 when a read is issued to an empty FIFO and gets
        /// cleared on register read (sticky bit until read).
        /// 0 = NO_UNDERRUN.
        /// 1 = UNDERRUN.
        const RX_UNDERRUN = 1 << 2;

        ///This bit is set to 1 when write data is issued to the TX FIFO when it is already full
        /// and gets cleared on register read (sticky bit until read).
        /// 0 = NO_OVERRUN.
        /// 1 = OVERRUN.
        const TX_OVERRUN = 1 << 3;

        /// The entry in this field reflects the number of current entries in the RX FIFO.
        const RX_FIFO_COUNTER = 63 << 16;
        /// The entry in this field reflects the number of current entries in the TX FIFO.
        const TX_FIFO_COUNTER = 63 << 24;
    }
}

/// Representation of the UART registers.
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    /// The `UART_THR_DLAB_0_0` register.
    pub THR_DLAB: ReadWrite<u32>,
    /// The `UART_IER_DLAB_0_0` register.
    pub IER_DLAB: ReadWrite<u32>,
    /// The `UART_IIR_FCR_0` register.
    pub IIR_FCR: ReadWrite<u32>,
    /// The `UART_LCR_0` register.
    pub LCR: ReadWrite<u32>,
    /// The `UART_MCR_0` register.
    pub MCR: ReadWrite<u32>,
    /// The `UART_LSR_0` register.
    pub LSR: ReadWrite<u32>,
    /// The `UART_MSR_0` register.
    pub MSR: ReadWrite<u32>,
    /// The `UART_SPR_0` register.
    pub SPR: ReadWrite<u32>,
    /// The `UART_IRDA_CSR_0` register.
    pub IRDA_CSR: ReadWrite<u32>,
    /// The `UART_RX_FIFO_CFG_0` register.
    pub RX_FIFO_CFG: ReadWrite<u32>,
    /// The `UART_MIE_0` register.
    pub MIE: ReadWrite<u32>,
    /// The `UART_VENDOR_STATUS_0_0` register.
    pub VENDOR_STATUS: ReadWrite<u32>,
    _unk: [u8; 0xC],
    /// The `UART_ASR_0` register.
    pub ASR: ReadWrite<u32>,
}

impl Registers {
    /// Factory method to create a pointer to the UART A registers.
    #[inline]
    pub fn get_a() -> &'static Self {
        unsafe { &*(UART_A_BASE as *const Registers) }
    }

    /// Factory method to create a pointer to the UART B registers.
    #[inline]
    pub fn get_b() -> &'static Self {
        unsafe { &*(UART_B_BASE as *const Registers) }
    }

    /// Factory method to create a pointer to the UART C registers.
    #[inline]
    pub fn get_c() -> &'static Self {
        unsafe { &*(UART_C_BASE as *const Registers) }
    }

    /// Factory method to create a pointer to the UART D registers.
    #[inline]
    pub fn get_d() -> &'static Self {
        unsafe { &*(UART_D_BASE as *const Registers) }
    }

    /// Factory method to create a pointer to the UART E registers.
    #[inline]
    pub fn get_e() -> &'static Self {
        unsafe { &*(UART_E_BASE as *const Registers) }
    }
}

/// Representation of a UART.
#[derive(Clone, Copy)]
pub struct Uart {
    /// The UART CPU registers used for communication.
    registers: &'static Registers,
    /// The device clock to enable data transfer.
    clock: &'static Clock,
}

// Definitions for known UARTs.
impl Uart {
    /// Representation of the UART A.
    pub const A: Self = Uart {
        registers: Registers::get_a(),
        clock: &Clock::UART_A,
    };

    /// Representation of the UART B.
    pub const B: Self = Uart {
        registers: Registers::get_b(),
        clock: &Clock::UART_B,
    };

    /// Representation of the UART C.
    pub const C: Self = Uart {
        registers: Registers::get_c(),
        clock: &Clock::UART_C,
    };

    /// Representation of the UART D.
    pub const D: Self = Uart {
        registers: Registers::get_d(),
        clock: &Clock::UART_D,
    };

    /// Representation of the UART APE.
    pub const E: Self = Uart {
        registers: Registers::get_e(),
        clock: &Clock::UART_APE,
    };
}

impl Uart {
    /// Waits for a given amount of cycles at a given baud rate.
    #[inline]
    fn wait_cycles(&self, baud: u32, amount: u32) {
        usleep((amount * 1_000_000 + 16 * baud - 1) / (16 * baud));
    }

    /// Waits for a given amount of symbols at a given baud rate.
    #[inline]
    fn wait_symbols(&self, baud: u32, amount: u32) {
        usleep((amount * 1_000_000 + baud - 1) / baud);
    }

    /// Blocks until the line has entered the desired state.
    #[inline]
    pub fn wait_idle(&self, status: VendorStatus) {
        if status.contains(VendorStatus::UART_TX_IDLE) {
            while (self.registers.LSR.get() & LineStatus::TMTY.bits()) == 0 {}
        }

        if status.contains(VendorStatus::UART_RX_IDLE) {
            while (self.registers.LSR.get() & LineStatus::RDR.bits()) == 0 {}
        }
    }

    /// Waits until data have been transmitted.
    #[inline]
    fn wait_transmit(&self) {
        while (self.registers.LSR.get() & LineStatus::THRE.bits()) == 0 {}
    }

    /// Waits until data have been received.
    #[inline]
    fn wait_receive(&self) {
        while (self.registers.LSR.get() & LineStatus::RDR.bits()) == 0 {}
    }

    /// Initializes the UART.
    pub fn init(&self, baud: u32) {
        // Enable device clock.
        self.clock.enable();

        // Wait for TX idle state.
        self.wait_idle(VendorStatus::UART_TX_IDLE);

        // Calculate baud rate and round to nearest.
        let baud_rate = (8 * baud + 408_000_000) / (16 * baud);

        // Disable interrupts.
        self.registers.IER_DLAB.set(0);

        // No hardware flow control.
        self.registers.MCR.set(0);

        // Enable DLAB and set word length to 8.
        self.registers
            .LCR
            .set((LineControl::DLAB | LineControl::WORD_LENGTH_8).bits());

        self.registers.THR_DLAB.set(baud_rate);
        self.registers.IER_DLAB.set(baud_rate >> 8);

        // Disable DLAB.
        self.registers
            .LCR
            .set(self.registers.LCR.get() & !LineControl::DLAB.bits());

        self.registers.SPR.get(); // Dummy read.
        self.wait_symbols(baud, 3); // Wait for 3 symbols.

        // Enable FIFO.
        self.registers.IIR_FCR.set(FifoControl::FCR_EN_FIFO.bits());
        self.registers.SPR.get(); // Dummy read.
        self.wait_cycles(baud, 3); // Wait for 3 baud cycles.

        // Flush FIFO.
        self.wait_idle(VendorStatus::UART_TX_IDLE); // Ensure no data is being written to TX FIFO.
        self.registers
            .IIR_FCR
            .set(self.registers.IIR_FCR.get() | (FifoControl::RX_CLR | FifoControl::TX_CLR).bits()); // Clear TX and RX FIFOs.
        self.wait_cycles(baud, 32); // Wait for 32 baud cycles.

        // Wait for idle state.
        self.wait_idle(VendorStatus::UART_TX_IDLE | VendorStatus::UART_RX_IDLE);
    }

    /// Writes a byte over UART.
    pub fn write_byte(&self, byte: u8) {
        // Wait until it is possible to write data.
        self.wait_transmit();

        // Write the byte.
        self.registers.THR_DLAB.set(u32::from(byte));
    }

    /// Writes a buffer of `u8` data over UART.
    pub fn write(&self, data: &[u8]) {
        for byte in data {
            self.write_byte(*byte);
        }
    }

    /// Reads a byte (`u8`) over UART.
    pub fn read_byte(&self) -> u8 {
        // Wait until it is possible to read data.
        self.wait_receive();

        // Read byte.
        self.registers.THR_DLAB.get() as u8
    }

    /// Reads bytes into a buffer.
    pub fn read(&self, buffer: &mut [u8]) {
        for i in buffer.iter_mut() {
            *i = self.read_byte();
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // Write data.
        self.write(s.as_bytes());

        // Wait for everything to be written.
        self.wait_transmit();

        Ok(())
    }
}

unsafe impl Send for Uart {}

unsafe impl Sync for Uart {}
