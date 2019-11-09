//! Tegra 210 Clock And Reset Controller interface and configurations.
//!
//! # Description
//!
//! The Clock and Reset (CAR) block contains all the logic needed to
//! control most of the clocks and resets to the Tegra X1 device.
//! The CAR block provides the registers to program the PLLs and
//! controls most of the clock source programming, and most of the
//! clock dividers.
//!
//! Generally speaking, clocks are used to set up non-boot devices
//! for operation.
//!
//! # Implementation
//!
//! - Most of the CAR registers were ignored as they aren't required.
//! A few ones however are exposed as global constants within the crate.
//!
//! - The [`Clock`] struct is an abstraction of a device clock which
//! holds all the important configuration values for controlling it.
//!
//! - [`Clock`] holds pre-defined constants which represent known clocks.
//! These can be used for convenience.
//!
//! - [`Clock::enable`], [`Clock::disable`] and [`Clock::is_enabled`] can
//! be used to check and modify the state of a device.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::clock::Clock;
//!
//! fn main() {
//!     let se_clock = Clock::SE;
//!
//!     // Enable Security Engine.
//!     se_clock.enable();
//!     assert_eq!(se_clock.is_enabled(), true);
//!
//!     // Disable Security Engine.
//!     se_clock.disable();
//!     assert_eq!(se_clock.is_enabled(), false);
//! }
//! ```
//!
//! [`Clock`]: struct.Clock.html
//! [`Clock::enable`]: struct.Clock.html#method.enable
//! [`Clock::disable`]: struct.Clock.html#method.disable
//! [`Clock::is_enabled`]: struct.Clock.html#method.is_enabled

use register::mmio::ReadWrite;

/// Base address for clock registers.
const CLOCK_BASE: u32 = 0x6000_6000;

register!(SPARE_REG0, CLOCK_BASE + 0x55C);

register!(OSC_CTRL, CLOCK_BASE + 0x50);

register!(CLK_SYSTEM_RATE, CLOCK_BASE + 0x30);

register!(PLLMB_BASE, CLOCK_BASE + 0x5E8);

register!(CLK_SOURCE_SYS, CLOCK_BASE + 0x400);

register!(SCLK_BURST_POLICY, CLOCK_BASE + 0x28);

register!(SCLK_DIVIDER, CLOCK_BASE + 0x2C);

/// Representation of a device clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Clock {
    /// The clock device reset register.
    reset: u32,
    /// The clock device enable register.
    enable: u32,
    /// The clock source register.
    source: u32,
    /// The clock index.
    index: u8,
    /// The clock source value.
    clock_source: u32,
    /// The clock divisor register.
    clock_divisor: u32,
}

const CLK_RST_CONTROLLER_RST_DEVICES_L: u32 = 0x4;
const CLK_RST_CONTROLLER_RST_DEVICES_H: u32 = 0x8;
const CLK_RST_CONTROLLER_RST_DEVICES_U: u32 = 0xC;
const CLK_RST_CONTROLLER_RST_DEVICES_X: u32 = 0x28C; const CLK_RST_CONTROLLER_RST_DEVICES_Y: u32 = 0x2A4;
const CLK_RST_CONTROLLER_RST_DEVICES_V: u32 = 0x358;
const CLK_RST_CONTROLLER_RST_DEVICES_W: u32 = 0x35C;

const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_X: u32 = 0x280;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_Y: u32 = 0x298;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_V: u32 = 0x360;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_W: u32 = 0x364;

const CLK_NO_SOURCE: u32 = 0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_A: u32 = 0x178;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_B: u32 = 0x17C;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_C: u32 = 0x1A0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_D: u32 = 0x1C0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE: u32 = 0x710;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1: u32 = 0x124;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2: u32 = 0x198;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3: u32 = 0x1B8;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4: u32 = 0x3C4;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5: u32 = 0x128;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6: u32 = 0x65C;
const CLK_RST_CONTROLLER_CLK_SOURCE_SE: u32 = 0x42C;
const CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X: u32 = 0x180;
const CLK_RST_CONTROLLER_CLK_SOURCE_TSEC: u32 = 0x1F4;
const CLK_RST_CONTROLLER_CLK_SOURCE_SOR1: u32 = 0x410;
const CLK_RST_CONTROLLER_CLK_SOURCE_CSITE: u32 = 0x1D4;
const CLK_RST_CONTROLLER_CLK_SOURCE_PWM: u32 = 0x11;

// Definitions for known devices.
impl Clock {
    /// Representation of the UART A clock.
    pub const UART_A: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_A,
        index: 0x6,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART B clock.
    pub const UART_B: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_B,
        index: 0x7,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART C clock.
    pub const UART_C: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_C,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART D clock.
    pub const UART_D: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_D,
        index: 0x1,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART APE clock.
    pub const UART_APE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE,
        index: 0x14,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the I²C 1 clock.
    pub const I2C_1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1,
        index: 0xC,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 2 clock.
    pub const I2C_2: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2,
        index: 0x16,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 3 clock.
    pub const I2C_3: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3,
        index: 0x3,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 4 clock.
    pub const I2C_4: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4,
        index: 0x7,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 5 clock.
    pub const I2C_5: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5,
        index: 0xF,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 6 clock.
    pub const I2C_6: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6,
        index: 0x6,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the Security Engine clock.
    pub const SE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SE,
        index: 0x1F,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the TZRAM clock.
    pub const TZRAM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the HOST1X clock.
    pub const HOST1X: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X,
        index: 0x1C,
        clock_source: 0x4,
        clock_divisor: 0x3,
    };

    /// Representation of the TSEC clock.
    pub const TSEC: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSEC,
        index: 0x13,
        clock_source: 0,
        clock_divisor: 0x2,
    };

    /// Representation of the SOR_SAFE clock.
    pub const SOR_SAFE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR0 clock.
    pub const SOR0: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: 0x16,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR1 clock.
    pub const SOR1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SOR1,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 0x2,
    };

    /// Representation of the KFUSE clock.
    pub const KFUSE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x8,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CL-DVFS clock.
    pub const CL_DVFS: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_W,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_W,
        source: CLK_NO_SOURCE,
        index: 0x1B,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CSITE clock.
    pub const CORESIGHT: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_CSITE,
        index: 0x9,
        clock_source: 0,
        clock_divisor: 0x4,
    };

    /// Representation of the PWM clock.
    pub const PWM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_PWM,
        index: 0x11,
        clock_source: 0x6,
        clock_divisor: 0x4,
    };
}

impl Clock {
    /// Sets whether the clock should be reset or not.
    fn set_reset(&self, set_reset: bool) {
        let reset_reg = unsafe { &(*((CLOCK_BASE + self.reset) as *const ReadWrite<u32>)) };

        let current_value = reset_reg.get();
        let mask = (1 << self.index & 0x1F) as u32;

        let new_value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        reset_reg.set(new_value);
    }

    /// Sets whether the clock should be enabled or disabled.
    fn set_enable(&self, set_enable: bool) {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };

        let current_value = enable_reg.get();
        let mask = (1 << (self.index & 0x1F)) as u32;

        let new_value = if set_enable {
            current_value | mask
        } else {
            current_value & !mask
        };

        enable_reg.set(new_value);
    }

    /// Enables the clock.
    pub fn enable(&self) {
        // Disable clock.
        self.disable();

        // Setup clock source if needed.
        if self.source != 0 {
            let source_reg = unsafe { &(*((CLOCK_BASE + self.source) as *const ReadWrite<u32>)) };
            source_reg.set(self.clock_divisor | (self.clock_source << 29));
        }

        // Enable clock.
        self.set_enable(true);
        self.set_reset(false);
    }

    /// Disables the clock.
    pub fn disable(&self) {
        // Put clock into reset.
        self.set_reset(true);
        // Disable.
        self.set_enable(false);
    }

    /// Resets the clock
    pub fn reset(&self) {
        self.disable();
        self.enable();
    }

    /// Whether the clock is enabled or not.
    pub fn is_enabled(&self) -> bool {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };
        let mask = (1 << (self.index & 0x1F)) as u32;

        (enable_reg.get() & mask) == mask
    }
}
