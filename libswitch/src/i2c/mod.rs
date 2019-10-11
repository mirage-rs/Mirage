//! Tegra210 I²C driver.

use core::marker::{Send, Sync};

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use register::mmio::ReadWrite;

use crate::timer::usleep;

pub const MAX77621_CPU_I2C_ADDR: u8 = 0x1B;
pub const MAX77621_GPU_I2C_ADDR: u8 = 0x1C;
pub const MAX17050_I2C_ADDR: u8 = 0x36;
pub const MAX77620_PWR_I2C_ADDR: u8 = 0x3C;
pub const MAX77620_RTC_I2C_ADDR: u8 = 0x68;
pub const BQ24193_I2C_ADDR: u8 = 0x6B;

#[repr(C)]
struct I2cRegisters {
    i2c_cnfg_0: ReadWrite<u32>,
    i2c_cmd_addr0_0: ReadWrite<u32>,
    i2c_cmd_addr1_0: ReadWrite<u32>,
    i2c_cmd_data1_0: ReadWrite<u32>,
    i2c_cmd_data2_0: ReadWrite<u32>,
    _0x14: ReadWrite<u32>,
    _0x18: ReadWrite<u32>,
    i2c_status_0: ReadWrite<u32>,
    i2c_sl_cnfg: ReadWrite<u32>,
    i2c_sl_rcvd: ReadWrite<u32>,
    i2c_sl_status_0: ReadWrite<u32>,
    i2c_sl_addr1_0: ReadWrite<u32>,
    i2c_sl_addr2_0: ReadWrite<u32>,
    i2c_tlow_sext_0: ReadWrite<u32>,
    _0x38: ReadWrite<u32>,
    i2c_sl_delay_count_0: ReadWrite<u32>,
    i2c_sl_int_mask_0: ReadWrite<u32>,
    i2c_sl_int_source_0: ReadWrite<u32>,
    i2c_sl_int_set_0: ReadWrite<u32>,
    _0x4c: ReadWrite<u32>,
    i2c_tx_packet_fifo_0: ReadWrite<u32>,
    i2c_rx_fifo_0: ReadWrite<u32>,
    packet_transfer_status_0: ReadWrite<u32>,
    fifo_control_0: ReadWrite<u32>,
    fifo_status_0: ReadWrite<u32>,
    interrupt_mask_register_0: ReadWrite<u32>,
    interrupt_status_register_0: ReadWrite<u32>,
    i2c_clk_divisor_register_0: ReadWrite<u32>,
    i2c_interrupt_source_register_0: ReadWrite<u32>,
    i2c_interrupt_set_register_0: ReadWrite<u32>,
    i2c_slv_tx_packet_fifo_0: ReadWrite<u32>,
    i2c_slv_rx_fifo_0: ReadWrite<u32>,
    i2c_slv_packet_status_0: ReadWrite<u32>,
    i2c_bus_clear_config_0: ReadWrite<u32>,
    i2c_bus_clear_status_0: ReadWrite<u32>,
    i2c_config_load_0: ReadWrite<u32>,
    _0x90: ReadWrite<u32>,
    i2c_interface_timing_0_0: ReadWrite<u32>,
    i2c_interface_timing_1_0: ReadWrite<u32>,
    i2c_hs_interface_timing_0_0: ReadWrite<u32>,
    i2c_hs_interface_timing_1_0: ReadWrite<u32>,
}

/// I²C transfer error.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Error;

/// Representation of an I²C device.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I2cDevice {
    registers: *const I2cRegisters,
}

unsafe impl Send for I2cDevice {}

unsafe impl Sync for I2cDevice {}

impl I2cDevice {
    pub const I1: Self = I2cDevice {
        registers: 0x7000_C000 as *const I2cRegisters,
    };

    pub const I2: Self = I2cDevice {
        registers: 0x7000_C400 as *const I2cRegisters,
    };

    pub const I3: Self = I2cDevice {
        registers: 0x7000_C500 as *const I2cRegisters,
    };

    pub const I4: Self = I2cDevice {
        registers: 0x7000_C700 as *const I2cRegisters,
    };

    pub const I5: Self = I2cDevice {
        registers: 0x7000_D000 as *const I2cRegisters,
    };

    pub const I6: Self = I2cDevice {
        registers: 0x7000_D100 as *const I2cRegisters,
    };
}

impl I2cDevice {
    fn load_config(&self) {
        let config_load = unsafe { &((*self.registers).i2c_config_load_0) };

        // Set MSTR_CONFIG_LOAD, TIMEOUT_CONFIG_LOAD, undocumented bit.
        config_load.set(0x25);

        // Wait for a bit of the master config to be loaded.
        for i in 0..20 {
            usleep(1);
            if !(config_load.get() & 1) {
                break;
            }
        }
    }

    fn send_packet(&self, device: u32, data: &[u8]) -> Result<bool, Error> {
        if data.len() == 0 {
            return Ok(false);
        }

        let cmd_addr0_reg = unsafe { &((*self.registers).i2c_cmd_addr0_0) };
        let cmd_data1_reg1 = unsafe { &((*self.registers).i2c_cmd_data1_0) };
        let cnfg_reg = unsafe { &((*self.registers).i2c_cnfg_0) };
        let status_reg = unsafe { &((*self.registers).i2c_status_0) };

        // Set device for 7-bit write mode.
        cmd_addr0_reg.set(device << 1);

        // Load in data to write.
        cmd_data1_reg.set(LittleEndian::read_u32(&buffer));

        // Set config with LENGTH = buffer_len, NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        cnfg_reg.set((((buffer_len << 1) - 2) | 0x2800) as u32);

        self.load_config();

        // CONFIG |= SEND.
        cnfg_reg.set(((cnfg_reg.get() & 0xFFFF_FDFF) | 0x200));

        while status_reg.get() & 0x100 {}

        // Return CMD1_STAT == SL1_XFER_SUCCESSFUL.
        Ok(((status_reg.get() & 0xF) == 0))
    }

    fn receive_packet(&self, device: u32, buffer: &mut [u8]) -> Result<(), Error> {
        if buffer.len() > 4 {
            return Err(Error {});
        } else if buffer.len() == 0 {
            return Ok(());
        }

        let cmd_addr0_reg = unsafe { &((*self.registers).i2c_cmd_addr0_0) };
        let cmd_data1_reg = unsafe { &((*self.registers).i2c_cmd_data1_0) };
        let cnfg_reg = unsafe { &((*self.registers).i2c_cnfg_0) };
        let status_reg = unsafe { &((*self.registers).i2c_status_0) };

        // Set device for 7-bit read mode.
        cmd_addr0_reg.set((device << 1) | 1);

        // Set config with LENGTH = dst_size, NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        cnfg_reg.set((((buffer.len() - 1) << 1) | 0x2840) as u32);

        self.load_config();

        // CONFIG |= SEND.
        cnfg_reg.set(((cnfg_reg.get() & 0xFFFF_FDFF) | 0x200));

        while status_reg.get() & 0x100 {}

        // Ensure success.
        if status_reg.get() << 28 {
            return Ok(());
        }

        // Write LS value to buffer.
        buffer
            .write_u32::<LittleEndian>(cmd_data1_reg.get())
            .unwrap();

        Ok(())
    }

    /// Initializes the device.
    pub fn init(&self) {
        let clk_divisor_reg = unsafe { &((*self.registers).i2c_clk_divisor_register_0) };
        let bus_clear_config_reg = unsafe { &((*self.registers).i2c_bus_clear_config_0) };
        let bus_clear_status_reg = unsafe { &((*self.registers).i2c_bus_clear_status_0) };
        let interrupt_status_reg = unsafe { &((*self.registers).interrupt_status_register_0) };

        // Setup divisor and clear the bus.
        clk_divisor_reg.set(0x50001);
        bus_clear_config_reg.set(0x90003);

        // Load hardware configuration.
        self.load_config();

        // Wait a while until BUS_CLEAR_DONE is set.
        for i in 0..10 {
            usleep(20000);
            if interrupt_status_reg.get() & 0x800 {
                break;
            }
        }

        bus_clear_status_reg.get(); // Dummy read.

        // Read and set the Interrupt Status.
        interrupt_status_reg.set(interrupt_status_reg.get());
    }

    /// Writes data to a given device.
    pub fn write(&self, device: u8, register: u8, data: &[u8]) -> Result<(), Error> {
        if data.len() > 3 {
            return Err(Error {});
        }

        let buffer_len = data.len() + 1;
        let mut buffer: [u8; buffer_len] = [0; buffer_len];

        buffer[0] = register as u8;
        buffer[1..].copy_from_slice(data);

        let result = match self.send_packet(u32::from(device), &buffer).unwrap() {
            Ok(b) => b,
            _ => false,
        };

        if result {
            return Ok(());
        }

        Err(Error {})
    }

    /// Reads data from a given device.
    pub fn read(&self, device: u8, register: u8, buffer: &mut [u8]) -> Result<(), Error> {
        let device_id = u32::from(device);

        let can_read = match self.send_packet(device_id, &[register]).unwrap() {
            Ok(b) => b,
            _ => false,
        };

        if can_read {
            return self.receive_packet(device_id, buffer);
        }

        Err(Error {})
    }
}
