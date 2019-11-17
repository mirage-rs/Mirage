//! Driver for the Switch's Maxim77620 Power regulators.
//!
//! # Overview
//!
//! **Switch Power domains (max77620):**
//!
//! | Name  | Usage         | uV step | uV min | uV default | uV max  | Init             |
//! |-------|---------------|---------|--------|------------|---------|------------------|
//! |  sd0  | SoC           | 12500   | 600000 |  625000    | 1400000 | 1.125V (pkg1.1)  |
//! |  sd1  | SDRAM         | 12500   | 600000 | 1125000    | 1125000 | 1.1V   (pkg1.1)  |
//! |  sd2  | ldo{0-1, 7-8} | 12500   | 600000 | 1325000    | 1350000 | 1.325V (pcv)     |
//! |  sd3  | 1.8V general  | 12500   | 600000 | 1800000    | 1800000 |                  |
//! |  ldo0 | Display Panel | 25000   | 800000 | 1200000    | 1200000 | 1.2V   (pkg1.1)  |
//! |  ldo1 | XUSB, PCIE    | 25000   | 800000 | 1050000    | 1050000 | 1.05V  (pcv)     |
//! |  ldo2 | SDMMC1        | 50000   | 800000 | 1800000    | 3300000 |                  |
//! |  ldo3 | GC ASIC       | 50000   | 800000 | 3100000    | 3100000 | 3.1V   (pcv)     |
//! |  ldo4 | RTC           | 12500   | 800000 |  850000    |  850000 |                  |
//! |  ldo5 | GC ASIC       | 50000   | 800000 | 1800000    | 1800000 | 1.8V   (pcv)     |
//! |  ldo6 | Touch, ALS    | 50000   | 800000 | 2900000    | 2900000 | 2.9V             |
//! |  ldo7 | XUSB          | 50000   | 800000 | 1050000    | 1050000 |                  |
//! |  ldo8 | XUSB, DC      | 50000   | 800000 | 1050000    | 1050000 |                  |
//!
//! # Implementation
//!
//! - The [`Regulator`] struct represents a Maxim77620 regulator and should
//! be used as a wrapper for the I²C configuration commands.
//!
//! - [`Regulator`] holds the constants for all known regulators which should
//! be used instead of creating own instances of this struct.
//!
//! - [`Regulator`] features two static methods, [`Regulator::config_default`]
//! and [`Regulator::low_battery_monitor_config`], which can be used without an
//! instance as they generally pertain to all of the regulators.
//!
//! - Regulators can be individually enabled and disabled with [`Regulator::enable`]
//! and [`Regulator::disable`]. Voltage and FPS may be configured with
//! [`Regulator::set_voltage`] and [`Regulator::config_fps`].
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::power::max77620::Regulator;
//!
//! fn main() {
//!     // Configure all regulators.
//!     Regulator::config_default();
//! }
//! ```
//!
//! [`Regulator`]: struct.Regulator.html
//! [`Regulator::config_default`]: struct.Regulator.html#method.config_default
//! [`Regulator::low_battery_monitor_config`]: struct.Regulator.html#method.low_battery_monitor_config
//! [`Regulator::enable`]: struct.Regulator.html#method.enable
//! [`Regulator::disable`]: struct.Regulator.html#method.disable
//! [`Regulator::set_voltage`]: struct.Regulator.html#method.set_voltage
//! [`Regulator::config_fps`]: struct.Regulator.html#method.config_fps

use crate::{
    i2c::{I2c, MAX77620_PWR_I2C_ADDR},
    timer::usleep,
};

const REGULATOR_SD: u8 = 0;
const REGULATOR_LDO: u8 = 1;

/// Representation of a Maxim 77620 regulator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Regulator<'a> {
    pub regulator_type: u8,
    pub name: &'a str,
    pub reg_sd: u8,
    pub mv_step: u32,
    pub mv_min: u32,
    pub mv_default: u32,
    pub mv_max: u32,
    pub volt_addr: u8,
    pub cfg_addr: u8,
    pub volt_mask: u8,
    pub enable_mask: u8,
    pub enable_shift: u8,
    pub status_mask: u8,

    pub fps_addr: u8,
    pub fps_src: u8,
    pub pd_period: u8,
    pub pu_period: u8,
}

// Definitions for known regulators.
impl<'a> Regulator<'a> {
    /// The `sd0` power domain.
    pub const SD0: Self = Regulator {
        regulator_type: REGULATOR_SD,
        name: "sd0",
        reg_sd: 0x16,
        mv_step: 12500,
        mv_min: 600000,
        mv_default: 625000,
        mv_max: 1400000,
        volt_addr: 0x16,
        cfg_addr: 0x1D,
        volt_mask: 0x3F,
        enable_mask: 0x30,
        enable_shift: 0x4,
        status_mask: 0x80,
        fps_addr: 0x4F,
        fps_src: 0x1,
        pd_period: 0x7,
        pu_period: 0x1,
    };

    /// The `sd1` power domain.
    pub const SD1: Self = Regulator {
        regulator_type: REGULATOR_SD,
        name: "sd1",
        reg_sd: 0x17,
        mv_step: 12500,
        mv_min: 600000,
        mv_default: 1125000,
        mv_max: 1125000,
        volt_addr: 0x17,
        cfg_addr: 0x1E,
        volt_mask: 0x3F,
        enable_mask: 0x30,
        enable_shift: 0x4,
        status_mask: 0x40,
        fps_addr: 0x50,
        fps_src: 0,
        pd_period: 0x1,
        pu_period: 0x5,
    };

    /// The `sd2` power domain.
    pub const SD2: Self = Regulator {
        regulator_type: REGULATOR_SD,
        name: "sd2",
        reg_sd: 0x18,
        mv_step: 12500,
        mv_min: 600000,
        mv_default: 1325000,
        mv_max: 1350000,
        volt_addr: 0x18,
        cfg_addr: 0x1F,
        volt_mask: 0xFF,
        enable_mask: 0x30,
        enable_shift: 0x4,
        status_mask: 0x20,
        fps_addr: 0x51,
        fps_src: 0x1,
        pd_period: 0x5,
        pu_period: 0x2,
    };

    /// The `sd3` power domain.
    pub const SD3: Self = Regulator {
        regulator_type: REGULATOR_SD,
        name: "sd3",
        reg_sd: 0x19,
        mv_step: 12500,
        mv_min: 600000,
        mv_default: 1800000,
        mv_max: 1800000,
        volt_addr: 0x19,
        cfg_addr: 0x20,
        volt_mask: 0xFF,
        enable_mask: 0x30,
        enable_shift: 0x4,
        status_mask: 0x10,
        fps_addr: 0x52,
        fps_src: 0,
        pd_period: 0x3,
        pu_period: 0x3,
    };

    /// The `ldo0` power domain.
    pub const LDO0: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo0",
        reg_sd: 0x00,
        mv_step: 25000,
        mv_min: 800000,
        mv_default: 1200000,
        mv_max: 1200000,
        volt_addr: 0x23,
        cfg_addr: 0x24,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x46,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo1` power domain.
    pub const LDO1: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo1",
        reg_sd: 0x00,
        mv_step: 25000,
        mv_min: 800000,
        mv_default: 1050000,
        mv_max: 1050000,
        volt_addr: 0x25,
        cfg_addr: 0x26,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x47,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo2` power domain.
    pub const LDO2: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo2",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 1800000,
        mv_max: 3300000,
        volt_addr: 0x27,
        cfg_addr: 0x28,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x48,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo3` power domain.
    pub const LDO3: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo3",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 3100000,
        mv_max: 3100000,
        volt_addr: 0x29,
        cfg_addr: 0x2A,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x49,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo4` power domain.
    pub const LDO4: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo4",
        reg_sd: 0x00,
        mv_step: 12500,
        mv_min: 800000,
        mv_default: 850000,
        mv_max: 850000,
        volt_addr: 0x2B,
        cfg_addr: 0x2C,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x4A,
        fps_src: 0,
        pd_period: 0x7,
        pu_period: 0x1,
    };

    /// The `ldo5` power domain.
    pub const LDO5: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo5",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 1800000,
        mv_max: 1800000,
        volt_addr: 0x2D,
        cfg_addr: 0x2E,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x4B,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo6` power domain.
    pub const LDO6: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo6",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 2900000,
        mv_max: 2900000,
        volt_addr: 0x2F,
        cfg_addr: 0x30,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x4C,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };

    /// The `ldo7` power domain.
    pub const LDO7: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo7",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 1050000,
        mv_max: 1050000,
        volt_addr: 0x31,
        cfg_addr: 0x32,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x4D,
        fps_src: 0x1,
        pd_period: 0x4,
        pu_period: 0x3,
    };

    /// The `ldo8` power domain.
    pub const LDO8: Self = Regulator {
        regulator_type: REGULATOR_LDO,
        name: "ldo8",
        reg_sd: 0x00,
        mv_step: 50000,
        mv_min: 800000,
        mv_default: 1050000,
        mv_max: 1050000,
        volt_addr: 0x33,
        cfg_addr: 0x34,
        volt_mask: 0x3F,
        enable_mask: 0xC0,
        enable_shift: 6,
        status_mask: 0,
        fps_addr: 0x4E,
        fps_src: 0x3,
        pd_period: 0x7,
        pu_period: 0,
    };
}

impl From<u8> for Regulator<'_> {
    fn from(id: u8) -> Self {
        match id {
            0 => Regulator::SD0,
            1 => Regulator::SD1,
            2 => Regulator::SD2,
            3 => Regulator::SD3,
            4 => Regulator::LDO0,
            5 => Regulator::LDO1,
            6 => Regulator::LDO2,
            7 => Regulator::LDO3,
            8 => Regulator::LDO4,
            9 => Regulator::LDO5,
            10 => Regulator::LDO6,
            11 => Regulator::LDO7,
            12 => Regulator::LDO8,
            _ => panic!("Invalid regulator ID given."),
        }
    }
}

impl<'a> Regulator<'a> {
    /// Configures all regulators with the default configuration options.
    pub fn config_default() {
        for i in 1..13 {
            match I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, 0x5C) {
                Ok(value) => {
                    let regulator = Regulator::from(value);
                    regulator.config_fps().unwrap();
                    regulator.set_voltage(regulator.mv_default).unwrap();

                    if regulator.fps_src != 0x3 {
                        regulator.enable();
                    }
                },
                Err(_) => {},
            };
        }

        I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x22, 4).unwrap();
    }

    /// Configures all regulators for low battery monitoring.
    pub fn low_battery_monitor_config() {
        I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0, 0x92).unwrap();
    }

    /// Enables or disables the regulator.
    fn set_enable(&self, set_enable: bool) -> Result<(), ()> {
        let addr = if self.regulator_type == REGULATOR_SD {
            self.cfg_addr
        } else {
            self.volt_addr
        };

        match I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, addr) {
            Ok(mut value) => {
                if set_enable {
                    value =
                        (value & !self.enable_mask) | ((3 << self.enable_shift) & self.enable_mask);
                } else {
                    value &= !self.enable_mask;
                }

                if I2c::C5
                    .write_byte(MAX77620_PWR_I2C_ADDR, addr, value)
                    .is_ok()
                {
                    usleep(1000);
                    return Ok(());
                }

                Err(())
            },
            Err(_) => Err(()),
        }
    }

    /// Enables the regulator.
    pub fn enable(&self) {
        self.set_enable(true).unwrap();
    }

    /// Disables the regulator.
    pub fn disable(&self) {
        self.set_enable(false).unwrap();
    }

    /// Configures the FPS value of the regulator.
    pub fn config_fps(&self) -> Result<(), ()> {
        let value = (self.fps_src << 6) | (self.pu_period << 3) | self.pd_period;

        if I2c::C5
            .write_byte(MAX77620_PWR_I2C_ADDR, self.fps_addr, value)
            .is_ok()
        {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Sets the voltage of the regulator.
    pub fn set_voltage(&self, mv: u32) -> Result<(), ()> {
        if mv < self.mv_default || mv > self.mv_max {
            return Err(());
        }

        let mult = (mv + self.mv_step - 1 - self.mv_min) / self.mv_step;

        match I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, self.volt_addr) {
            Ok(mut value) => {
                value = (value & !self.volt_mask) | (mult & self.volt_mask as u32) as u8;

                if I2c::C5
                    .write_byte(MAX77620_PWR_I2C_ADDR, self.volt_addr, value)
                    .is_ok()
                {
                    usleep(1000);
                    return Ok(());
                }

                Err(())
            },
            Err(_) => Err(()),
        }
    }
}
