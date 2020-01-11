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
//! - The [`Device`] enum represents I2C slaves that can be accessed.
//!
//! - The [`Registers`] struct provides abstractions over the I2C registers
//! and the possibility to create pointers to each I2C mapped at a different
//! address.
//!
//! - The [`I2c`] represents an I2C controller and holds the [`Clock`] to enable
//! the device and the respective [`Registers`] block pointer to communicate over
//! I²C.
//!
//! - [`I2c`] holds pre-defined constants which represent the I2C
//! controllers 1 through 6 and should be preferred over creating
//! instances of the [`I2c`] struct manually.
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
//! - I2C operations may fail for various reasons. Thus we return a [`Result`]
//! which, in case of failure, provides access to a member of [`Error`], which
//! can give more detailed information about the cause.
//!
//! - The [`Sync`] and [`Send`] traits are implemented for [`I2c`], it is
//! considered thread-safe.
//!
//! [`Device`]: enum.Device.html
//! [`Registers`]: struct.Registers.html
//! [`I2c`]: struct.I2c.html
//! [`Clock`]: ../clock/struct.Clock.html
//! [`I2c::init`]: struct.I2c.html#method.init
//! [`I2c::read`]: struct.I2c.html#method.read
//! [`I2c::write`]: struct.I2c.html#method.write
//! [`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html
//! [`Error`]: enum.Error.html
//! [`Sync`]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html
//! [`Send`]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html

use core::{
    convert::TryInto,
    marker::{Send, Sync},
};

use mirage_mmio::Mmio;

use crate::{clock::Clock, timer::usleep};

/// Base address for the I²C 1 controller.
pub(crate) const I2C_1_BASE: u32 = 0x7000_C000;

/// Base address for the I²C 2 controller.
pub(crate) const I2C_2_BASE: u32 = 0x7000_C400;

/// Base address for the I²C 3 controller.
pub(crate) const I2C_3_BASE: u32 = 0x7000_C500;

/// Base address for the I²C 4 controller.
pub(crate) const I2C_4_BASE: u32 = 0x7000_C700;

/// Base address for the I²C 5 controller.
pub(crate) const I2C_5_BASE: u32 = 0x7000_D000;

/// Base address for the I²C 6 controller.
pub(crate) const I2C_6_BASE: u32 = 0x7000_D100;

/// Enumeration of I²C devices the controller can access.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Device {
    /// The Maxim 77621 CPU device.
    Max77621Cpu = 0x1B,
    /// The Maxim 77621 GPU device.
    Max77621Gpu = 0x1C,
    /// The Maxim 17050 device.
    Max17050 = 0x36,
    /// The Maxim 77620 PWR device.
    Max77620Pwr = 0x3C,
    /// The Maxim 77620 RTC device.
    Max77620Rtc = 0x68,
    /// The TI BQ24193 device.
    Bq24193 = 0x6B,
}

/// Enumeration of possible errors when communicating over the I²C protocol.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// Generic I²C error. Not closer specified.
    Generic,
    /// An issue with memory organization, e.g. a
    /// buffer is too large to fit an I2C register.
    MemoryError,
    /// An I/O error that occurred during communication
    /// over I²C. Indicated through the MMIOs.
    IOError,
}

/// Representation of the I²C registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Registers {
    pub I2C_CNFG: Mmio<u32>,
    pub I2C_CMD_ADDR0: Mmio<u32>,
    pub I2C_CMD_ADDR1: Mmio<u32>,
    pub I2C_CMD_DATA1: Mmio<u32>,
    pub I2C_CMD_DATA2: Mmio<u32>,
    _reserved0: [Mmio<u8>; 0x8],
    pub I2C_STATUS: Mmio<u32>,
    pub I2C_SL_CNFG: Mmio<u32>,
    pub I2C_SL_RCVD: Mmio<u32>,
    pub I2C_SL_STATUS: Mmio<u32>,
    pub I2C_SL_ADDR1: Mmio<u32>,
    pub I2C_SL_ADDR2: Mmio<u32>,
    pub I2C_TLOW_SEXT: Mmio<u32>,
    _reserved1: [Mmio<u8>; 0x4],
    pub I2C_SL_DELAY_COUNT: Mmio<u32>,
    pub I2C_SL_INT_MASK: Mmio<u32>,
    pub I2C_SL_INT_SOURCE: Mmio<u32>,
    pub I2C_SL_INT_SET: Mmio<u32>,
    _reserved2: [Mmio<u8>; 0x4],
    pub I2C_TX_PACKET_FIFO: Mmio<u32>,
    pub I2C_RX_FIFO: Mmio<u32>,
    pub PACKET_TRANSFER_STATUS: Mmio<u32>,
    pub FIFO_CONTROL: Mmio<u32>,
    pub FIFO_STATUS: Mmio<u32>,
    pub INTERRUPT_MASK: Mmio<u32>,
    pub INTERRUPT_STATUS: Mmio<u32>,
    pub I2C_CLK_DIVISOR: Mmio<u32>,
    pub I2C_INTERRUPT_SOURCE: Mmio<u32>,
    pub I2C_INTERRUPT_SET: Mmio<u32>,
    pub I2C_SLV_TX_PACKET_FIFO: Mmio<u32>,
    pub I2C_SLV_RX_FIFO: Mmio<u32>,
    pub I2C_SLV_PACKET_STATUS: Mmio<u32>,
    pub I2C_BUS_CLEAR_CONFIG: Mmio<u32>,
    pub I2C_BUS_CLEAR_STATUS: Mmio<u32>,
    pub I2C_CONFIG_LOAD: Mmio<u32>,
    _reserved3: [Mmio<u8>; 0x4],
    pub I2C_INTERFACE_TIMING_0: Mmio<u32>,
    pub I2C_INTERFACE_TIMING_1: Mmio<u32>,
    pub I2C_HS_INTERFACE_TIMING_0: Mmio<u32>,
    pub I2C_HS_INTERFACE_TIMING_1: Mmio<u32>,
}

/// Representation of an I²C controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I2c {
    /// A pointer to the respective registers, used for communication.
    registers: *const Registers,
    /// The respective device clock for the controller.
    clock: &'static Clock,
}

// Definitions of known I²C controllers.
impl I2c {
    /// Representation of the I²C 1 controller.
    pub const C1: Self = I2c {
        registers: I2C_1_BASE as *const _,
        clock: &Clock::I2C_1,
    };

    /// Representation of the I²C 2 controller.
    pub const C2: Self = I2c {
        registers: I2C_2_BASE as *const _,
        clock: &Clock::I2C_2,
    };

    /// Representation of the I²C 3 controller.
    pub const C3: Self = I2c {
        registers: I2C_3_BASE as *const _,
        clock: &Clock::I2C_3,
    };

    /// Representation of the I²C 4 controller.
    pub const C4: Self = I2c {
        registers: I2C_4_BASE as *const _,
        clock: &Clock::I2C_4,
    };

    /// Representation of the I²C 5 controller.
    pub const C5: Self = I2c {
        registers: I2C_5_BASE as *const _,
        clock: &Clock::I2C_5,
    };

    /// Representation of the I²C 6 controller.
    pub const C6: Self = I2c {
        registers: I2C_6_BASE as *const _,
        clock: &Clock::I2C_6,
    };
}

impl I2c {
    /// Loads the hardware configuration for the controller.
    fn load_config(&self) {
        let register_base = unsafe { &*self.registers };

        // Set MSTR_CONFIG_LOAD, TIMEOUT_CONFIG_LOAD, undocumented bit.
        register_base.I2C_CONFIG_LOAD.write(0x25);

        // Wait a bit for master config to be loaded.
        for _ in 0..20 {
            usleep(1);

            if register_base.I2C_CONFIG_LOAD.read() & 1 == 0 {
                break;
            }
        }
    }

    /// Transmits a packet of data to a given device over I²C.
    fn write_packet(&self, device: Device, packet: &[u8]) -> Result<(), Error> {
        let register_base = unsafe { &*self.registers };

        // Set device for 7-bit write mode.
        register_base.I2C_CMD_ADDR0.write((device as u32) << 1);

        // Load in data to write.
        let data = u32::from_le_bytes(packet.try_into().unwrap());
        register_base.I2C_CMD_DATA1.write(data);

        // Set config with LENGTH = packet.len(), NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        register_base.I2C_CNFG.write((((packet.len() - 1) << 1) | 0x2800) as u32);

        // Kick off the transaction.
        self.load_config();

        // CONFIG |= SEND
        register_base.I2C_CNFG.write((register_base.I2C_CNFG.read() & 0xFFFF_FDFF) | 0x200);

        while (register_base.I2C_STATUS.read() & 0x100) != 0 {
            // Wait until not busy.
        }

        // Check whether the translation was successful and determine the appropriate Result.
        // CMD1_STAT == SL1_XFER_SUCCESSFUL
        if (register_base.I2C_STATUS.read() & 0xF) == 0 {
            Ok(())
        } else {
            Err(Error::IOError)
        }
    }

    /// Reads a packet of data from a given device over I²C.
    fn read_packet(&self, device: Device, packet: &mut [u8]) -> Result<(), Error> {
        let register_base = unsafe { &*self.registers };

        // Set device for 7-bit read mode.
        register_base.I2C_CMD_ADDR0.write(((device as u32) << 1) | 1);

        // Set config with LENGTH = packet.len(), NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        register_base.I2C_CNFG.write((((packet.len() - 1) << 1) | 0x2840) as u32);

        // Kick off the transaction.
        self.load_config();

        // CONFIG |= SEND
        register_base.I2C_CNFG.write((register_base.I2C_CNFG.read() & 0xFFFF_FDFF) | 0x200);

        while (register_base.I2C_STATUS.read() & 0x100) != 0 {
            // Wait until not busy.
        }

        // Check whether the translation was successful and determine the appropriate Result.
        // CMD1_STAT == SL1_XFER_SUCCESSFUL
        if (register_base.I2C_STATUS.read() & 0xF) == 0 {
            // Read and copy back the result.
            let result = register_base.I2C_CMD_DATA1.read();
            packet.copy_from_slice(&result.to_le_bytes()[..packet.len()]);

            Ok(())
        } else {
            Err(Error::IOError)
        }
    }

    /// Initializes the I²C controller.
    pub fn init(&self) {
        let register_base = unsafe { &*self.registers };

        // Enable the device clock.
        self.clock.enable();

        // Setup divisor, and clear the bus.
        register_base.I2C_CLK_DIVISOR.write(0x50001);
        register_base.I2C_BUS_CLEAR_CONFIG.write(0x90003);

        // Load hardware configuration.
        self.load_config();

        // Wait a while until BUS_CLEAR_DONE is set.
        for _ in 0..10 {
            usleep(20_000);

            if (register_base.INTERRUPT_STATUS.read() & 0x800) != 0 {
                break;
            }
        }

        // Dummy read.
        register_base.I2C_BUS_CLEAR_STATUS.read();

        // Read and set the Interrupt Status.
        register_base.INTERRUPT_STATUS.write(register_base.INTERRUPT_STATUS.read());
    }

    /// Writes a buffer of data to a register from a device over I²C.
    pub fn write(&self, device: Device, register: u8, data: &[u8]) -> Result<(), Error> {
        // Limit input size to 24 bits. One byte is reserved for the device register.
        if data.len() > 3 {
            return Err(Error::MemoryError);
        }

        // Prepare an I²C packet, composed from the device register and the provided data.
        // The u32 value that is read from the bytes will be written to the data registers.
        let mut packet = [0; 4];
        packet[0] = register;
        packet[1..=data.len()].copy_from_slice(data);

        // Write the packet to the device.
        self.write_packet(device, &packet[..])
    }

    /// Writes a byte to a register of a device over I²C.
    #[inline(always)]
    pub fn write_byte(&self, device: Device, register: u8, byte: u8) -> Result<(), Error> {
        self.write(device, register, &byte.to_le_bytes())
    }

    /// Reads the contents of a register from a device over I²C into a given buffer.
    pub fn read(&self, device: Device, register: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Limit output buffer size to 32 bits.
        if buffer.len() > 4 {
            return Err(Error::MemoryError);
        }

        // Write single byte register ID to device.
        self.write_packet(device, &[register])?;

        // Receive data and write them to the buffer.
        self.read_packet(device, buffer)
    }

    /// Reads a byte from a register of a device over I²C.
    #[inline(always)]
    pub fn read_byte(&self, device: Device, register: u8) -> Result<u8, Error> {
        let mut buffer = [0; 1];
        self.read(device, register, &mut buffer)?;

        Ok(u8::from_le_bytes(buffer.try_into().unwrap()))
    }
}

unsafe impl Send for I2c {}

unsafe impl Sync for I2c {}
