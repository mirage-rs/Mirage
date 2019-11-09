//! Inter-Integrated Circuit driver for Tegra210.
//!
//! # Description
//!
//! The I²C controller (I2C) implements an I²C 3.0 specification-compliant
//! I²C master and slave controller. The I²C controller supports multiple
//! masters and slaves. It supports Standard mode (up to 100 Kbits/s),
//! Fast mode (up to 400 Kbits/s), Fast mode plus (Fm+, up to 1 Mbits/s),
//! and High-speed mode (up to 3.4 Mbits/s).
//!
//! Tegra X1 devices have six instances of this controller. All six
//! instances have identical I2C master functionality. There are also
//! three additional I2C instances in the TSEC, CL-DVFS and VI modules.
//!
//! The I²C controller supports DMA for the Master controller over the APB
//! bus. There is no DMA support for the Slave controller. The I²C controller
//! also supports packet mode transfers where the data to be transferred is
//! encapsulated in a predefined packet format as payload and sent to the
//! I²C controller over the APB bus. The header of the packet specifies the
//! type of operation to be performed, the size and other parameters.
//!
//! # Implementation
//!
//! - The addresses of available I2C devices are exposed as constants.
//!
//! - The [`Registers`] struct along with its factory methods provide
//! abstractions over the I2C registers and the possibility to create
//! pointers to each I2C mapped at a different address.
//!
//! - The [`I2c`] represents an I2C and holds the [`Clock`] to enable
//! the device and the corresponding [`Registers`] block to do
//! communication.
//!
//! - [`I2c`] holds pre-defined constants which represent the I2C
//! controllers 1 through 6 and should be preferred over creating
//! instances of the [`I2c`] struct manually.
//!
//!
//! - [`I2c::init`] has to be called for each device before it can
//! be used.
//!
//! - [`I2c::read`] and [`I2c::write`] take buffers as arguments.
//! For write operations, this buffer must contain the byte
//! representation of the number to send in little-endian byte order.
//! For read operations, the buffer wis filled with little-endian-ordered
//! bytes.
//!
//! - The [`Sync`] trait is implemented for [`I2c`], it is considered
//! safe to share references between threads.
//!
//! - [`send_pmic_cpu_shutdown_cmd`], [`read_ti_charger_bit_7`],
//! [`clear_ti_charger_bit_7`] and [`set_ti_charger_bit_7`] are helper
//! functions which wrap common I2C operations.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::i2c::*;
//!
//! fn main() {
//!
//! }
//! ```
//!
//! [`Registers`]: struct.Registers.html
//! [`I2c`]: struct.I2c.html
//! [`Clock`]: ../clock/struct.Clock.html
//! [`I2c::init`]: struct.I2c.html#method.init
//! [`I2c::read`]: struct.I2c.html#method.read
//! [`I2c::write`]: struct.I2c.html#method.write
//! [`Sync`]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html
//! [`send_pmic_cpu_shutdown_cmd`]: fn.send_pmic_cpu_shutdown_cmd.html
//! [`read_ti_charger_bit_7`]: fn.read_ti_charger_bit_7.html
//! [`clear_ti_charger_bit_7`]: fn.clear_ti_charger_bit_7.html
//! [`set_ti_charger_bit_7`]: fn.set_ti_charger_bit_7.html

use core::{convert::TryInto, marker::Sync};

use register::mmio::ReadWrite;

use crate::{clock::Clock, timer::usleep};

/// Base address for the I²C registers 1 through 4.
const I2C_1234_BASE: u32 = 0x7000_C000;

/// Base address for the I²C registers 5 through 6.
const I2C_56_BASE: u32 = 0x7000_D000;

/// The I²C device address for the Maxim 77621 CPU.
pub const MAX77621_CPU_I2C_ADDR: u32 = 0x1B;
/// The I²C device address for the Maxim 77621 GPU.
pub const MAX77621_GPU_I2C_ADDR: u32 = 0x1C;
/// The I²C device address for the Maxim 17050.
pub const MAX17050_I2C_ADDR: u32 = 0x36;
/// The I²C device address for the Maxim 77620 PWR.
pub const MAX77620_PWR_I2C_ADDR: u32 = 0x3C;
/// The I²C device address for the Maxim 77620 RTC.
pub const MAX77620_RTC_I2C_ADDR: u32 = 0x68;
/// The I²C device address for the TI BQ24193.
pub const BQ24193_I2C_ADDR: u32 = 0x6B;

/// Enumeration of possible I²C errors that may occur.
#[derive(Debug)]
pub enum Error {
    /// Returned in case the boundaries of a buffer used for
    /// read and write operations exceed the permitted size.
    BufferBoundariesBlown,
    /// Returned when the transmission over I²C errors.
    TransmissionFailed,
    /// Returned when a querying error for a device occurs.
    QueryFailed,
}

/// Sets a bit in a PMIC register over I²C during CPU shutdown.
#[inline]
pub fn send_pmic_cpu_shutdown_cmd() -> Result<(), Error> {
    // PMIC == Device 4:3C.
    let value = I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, 0x41)?;

    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x41, value | 4)
}

/// Reads the value of TI charger bit over I²C.
#[inline]
pub fn read_ti_charger_bit_7() -> Result<bool, Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(BQ24193_I2C_ADDR, 0).unwrap();

    Ok((value & 0x80) != 0)
}

/// Clears TI charger bit over I²C.
#[inline]
pub fn clear_ti_charger_bit_7() -> Result<(), Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(BQ24193_I2C_ADDR, 0)?;

    I2c::C1.write_byte(BQ24193_I2C_ADDR, 0, value & 0x7F)
}

/// Sets TI charger bit over I²C.
#[inline]
pub fn set_ti_charger_bit_7() -> Result<(), Error> {
    // TI Charger = Device 0:6B.
    let value = I2c::C1.read_byte(BQ24193_I2C_ADDR, 0)?;

    I2c::C1.write_byte(BQ24193_I2C_ADDR, 0, value | 0x80)
}

/// Representation of the I²C registers.
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    pub I2C_CNFG: ReadWrite<u32>,
    pub I2C_CMD_ADDR0: ReadWrite<u32>,
    pub I2C_CMD_ADDR1: ReadWrite<u32>,
    pub I2C_CMD_DATA1: ReadWrite<u32>,
    pub I2C_CMD_DATA2: ReadWrite<u32>,
    _0x14: ReadWrite<u32>,
    _0x18: ReadWrite<u32>,
    pub I2C_STATUS: ReadWrite<u32>,
    pub I2C_SL_CNFG: ReadWrite<u32>,
    pub I2C_SL_RCVD: ReadWrite<u32>,
    pub I2C_SL_STATUS: ReadWrite<u32>,
    pub I2C_SL_ADDR1: ReadWrite<u32>,
    pub I2C_SL_ADDR2: ReadWrite<u32>,
    pub I2C_TLOW_SEXT: ReadWrite<u32>,
    _0x38: ReadWrite<u32>,
    pub I2C_SL_DELAY_COUNT: ReadWrite<u32>,
    pub I2C_SL_INT_MASK: ReadWrite<u32>,
    pub I2C_SL_INT_SOURCE: ReadWrite<u32>,
    pub I2C_SL_INT_SET: ReadWrite<u32>,
    _0x4C: ReadWrite<u32>,
    pub I2C_TX_PACKET_FIFO: ReadWrite<u32>,
    pub I2C_RX_FIFO: ReadWrite<u32>,
    pub PACKET_TRANSFER_STATUS: ReadWrite<u32>,
    pub FIFO_CONTROL: ReadWrite<u32>,
    pub FIFO_STATUS: ReadWrite<u32>,
    pub INTERRUPT_MASK_REGISTER: ReadWrite<u32>,
    pub INTERRUPT_STATUS_REGISTER: ReadWrite<u32>,
    pub I2C_CLK_DIVISOR_REGISTER: ReadWrite<u32>,
    pub I2C_INTERRUPT_SOURCE_REGISTER: ReadWrite<u32>,
    pub I2C_INTERRUPT_SET_REGISTER: ReadWrite<u32>,
    pub I2C_SLV_TX_PACKET_FIFO: ReadWrite<u32>,
    pub I2C_SLV_RX_FIFO: ReadWrite<u32>,
    pub I2C_SLV_PACKET_STATUS: ReadWrite<u32>,
    pub I2C_BUS_CLEAR_CONFIG: ReadWrite<u32>,
    pub I2C_BUS_CLEAR_STATUS: ReadWrite<u32>,
    pub I2C_CONFIG_LOAD: ReadWrite<u32>,
    _0x90: ReadWrite<u32>,
    pub I2C_INTERFACE_TIMING_0: ReadWrite<u32>,
    pub I2C_INTERFACE_TIMING_1: ReadWrite<u32>,
    pub I2C_HS_INTERFACE_TIMING_0: ReadWrite<u32>,
    pub I2C_HS_INTERFACE_TIMING_1: ReadWrite<u32>,
}

impl Registers {
    /// Factory method to create a pointer to the I²C 1 registers.
    #[inline]
    pub const fn get_1() -> &'static Self {
        unsafe { &*((I2C_1234_BASE + 0x000) as *const Registers) }
    }

    /// Factory method to create a pointer to the I²C 2 registers.
    #[inline]
    pub const fn get_2() -> &'static Self {
        unsafe { &*((I2C_1234_BASE + 0x400) as *const Registers) }
    }

    /// Factory method to create a pointer to the I²C 3 registers.
    #[inline]
    pub const fn get_3() -> &'static Self {
        unsafe { &*((I2C_1234_BASE + 0x500) as *const Registers) }
    }

    /// Factory method to create a pointer to the I²C 4 registers.
    #[inline]
    pub const fn get_4() -> &'static Self {
        unsafe { &*((I2C_1234_BASE + 0x700) as *const Registers) }
    }

    /// Factory method to create a pointer to the I²C 5 registers.
    #[inline]
    pub const fn get_5() -> &'static Self {
        unsafe { &*((I2C_56_BASE + 0x000) as *const Registers) }
    }

    /// Factory method to create a pointer to the I²C 6 registers.
    #[inline]
    pub const fn get_6() -> &'static Self {
        unsafe { &*((I2C_56_BASE + 0x100) as *const Registers) }
    }
}

/// Representation of an I²C controller.
#[derive(Clone, Copy)]
pub struct I2c {
    /// The device clock for the controller.
    clock: &'static Clock,
    /// The registers used for communication.
    registers: &'static Registers,
}

// Definitions for known I²C devices.
impl I2c {
    /// Representation of the I²C controller 1.
    pub const C1: Self = I2c {
        clock: &Clock::I2C_1,
        registers: Registers::get_1(),
    };

    /// Representation of the I²C controller 2.
    pub const C2: Self = I2c {
        clock: &Clock::I2C_2,
        registers: Registers::get_2(),
    };

    /// Representation of the I²C controller 3.
    pub const C3: Self = I2c {
        clock: &Clock::I2C_3,
        registers: Registers::get_3(),
    };

    /// Representation of the I²C controller 4.
    pub const C4: Self = I2c {
        clock: &Clock::I2C_4,
        registers: Registers::get_4(),
    };

    /// Representation of the I²C controller 5.
    pub const C5: Self = I2c {
        clock: &Clock::I2C_5,
        registers: Registers::get_5(),
    };

    /// Representation of the I²C controller 6.
    pub const C6: Self = I2c {
        clock: &Clock::I2C_6,
        registers: Registers::get_6(),
    };
}

impl I2c {
    /// Loads the hardware configuration for the I²C.
    fn load_config(&self) {
        // Set MSTR_CONFIG_LOAD, TIMEOUT_CONFIG_LOAD, undocumented bit.
        self.registers.I2C_CONFIG_LOAD.set(0x25);

        // Wait up to 20 microseconds for master config to be loaded.
        for i in 0..20 {
            usleep(i);
            if self.registers.I2C_CONFIG_LOAD.get() & 1 == 0 {
                break;
            }
        }
    }

    /// Transmits the data to the device over I²C.
    fn send(&self, device: u32, data: &[u8]) -> Result<(), Error> {
        // Set device for 7-bit write mode.
        self.registers.I2C_CMD_ADDR0.set(device << 1);

        // Load in data to write.
        let data_source = u32::from_le_bytes(data.try_into().unwrap());
        self.registers.I2C_CMD_DATA1.set(data_source);

        // Set config with LENGTH = data_length, NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        self.registers
            .I2C_CNFG
            .set((((data.len() << 1) - 2) | 0x2800) as u32);

        // Load hardware configuration.
        self.load_config();

        // CONFIG |= SEND.
        self.registers
            .I2C_CNFG
            .set((self.registers.I2C_CNFG.get() & 0xFFFF_FDFF) | 0x200);

        // Wait until not busy.
        while self.registers.I2C_STATUS.get() & 0x100 != 0 {}

        // Determine result from the result of CMD1_STAT == SL1_XFER_SUCCESSFUL.
        if self.registers.I2C_STATUS.get() & 0xF == 0 {
            return Ok(());
        } else {
            return Err(Error::TransmissionFailed);
        }
    }

    /// Receives bytes from the device over I²C and writes them to the buffer.
    fn receive(&self, device: u32, buffer: &mut [u8]) -> Result<(), Error> {
        // Set device for 7-bit read mode.
        self.registers.I2C_CMD_ADDR0.set((device << 1) | 1);

        // Set config with LENGTH = buffer.len(), NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        self.registers
            .I2C_CNFG
            .set((((buffer.len() << 1) - 2) | 0x2840) as u32);

        // Load hardware configuration.
        self.load_config();

        // CONFIG |= SEND.
        self.registers
            .I2C_CNFG
            .set((self.registers.I2C_CNFG.get() & 0xFFFF_FDFF) | 0x200);

        // Wait until not busy.
        while self.registers.I2C_STATUS.get() & 0x100 != 0 {}

        // Ensure success.
        if self.registers.I2C_STATUS.get() & 0xF != 0 {
            return Err(Error::QueryFailed);
        }

        // Read result and copy it back.
        let result = self.registers.I2C_CMD_DATA1.get().to_le_bytes();
        buffer.copy_from_slice(&result[..buffer.len()]);

        Ok(())
    }

    /// Initializes the I²C controller.
    pub fn init(&self) {
        // Enable device clock.
        self.clock.enable();

        // Setup divisor and clear the bus.
        self.registers.I2C_CLK_DIVISOR_REGISTER.set(0x50001);
        self.registers.I2C_BUS_CLEAR_CONFIG.set(0x90003);

        // Load hardware configuration.
        self.load_config();

        // Wait a while until BUS_CLEAR_DONE is set.
        for i in 0..10 {
            usleep(20000);
            if self.registers.INTERRUPT_STATUS_REGISTER.get() & 0x800 != 0 {
                break;
            }
        }

        // Dummy read.
        self.registers.I2C_BUS_CLEAR_STATUS.get();

        // Read and set the Interrupt Status.
        self.registers
            .INTERRUPT_STATUS_REGISTER
            .set(self.registers.INTERRUPT_STATUS_REGISTER.get());
    }

    /// Writes a buffer of data to a given device over I²C.
    pub fn write(&self, device: u32, register: u8, data: &[u8]) -> Result<(), Error> {
        // Limit input size to 32-bits. One byte is reserved for the device register.
        if data.len() > 3 {
            return Err(Error::BufferBoundariesBlown);
        }

        // Prepare a buffer holding the device register and the data contents.
        let mut buffer = [0; 4];
        buffer[0] = register;
        buffer[1..].copy_from_slice(data);

        // Send the buffer to the device.
        self.send(device, &buffer[..])
    }

    /// Writes an byte to a given device over I²C.
    #[inline]
    pub fn write_byte(&self, device: u32, register: u8, byte: u8) -> Result<(), Error> {
        // Write single byte to device.
        self.write(device, register, &byte.to_le_bytes())
    }

    /// Reads a register of a device over I²C and writes the result to the buffer.
    pub fn read(&self, device: u32, register: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Limit output size to 32-bits.
        if buffer.len() > 4 {
            return Err(Error::BufferBoundariesBlown);
        }

        // Write single byte register ID to device.
        self.send(device, &[register])?;

        // Receive data and write these to the buffer.
        self.receive(device, buffer)
    }

    /// Reads a byte from a given device over I²C.
    #[inline]
    pub fn read_byte(&self, device: u32, register: u8) -> Result<u8, Error> {
        let mut buffer = [0; 1];

        self.read(device, register, &mut buffer)?;

        Ok(u8::from_le_bytes(buffer.try_into().unwrap()))
    }
}

unsafe impl Sync for I2c {}
