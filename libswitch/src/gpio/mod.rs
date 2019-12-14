//! Interface to control the Tegra210 General-purpose Input/Output pins.
//!
//! # Description
//!
//! The GPIO controller for Tegra X1 devices provides the tools for configuring
//! each MPIO for use as a software-controlled GPIO.
//!
//! The GPIO controller is divided into 8 banks. Each bank handles the GPIO
//! functionality for up to 32 MPIOs. Within a bank, the GPIOs are arranged
//! as four ports of 8 bits each. The ports are labeled consecutively from A
//! through Z and then AA through FF. Ports A through D are in bank 0. Ports
//! E through H are in bank 1. In total, there are approximately 170 GPIOs,
//! (however, approximately 170 physical GPIOs are available on the chip) and
//! the banking and the banking and numbering conventions will have some
//! break in between but will maintain backward compatibility in register
//! configurations for the GPIOs as that of previous generation chips.
//!
//! Each GPIO can be individually configured as Output/Input/Interrupt sources
//! with level/edge controls.
//!
//! GPIO configuration has a lock bit controlling every bit separately.
//! When the LOCK bit is set, the associated control aspects of the bits (for
//! example, whether it is an Output/Input or used as GPIO or SFIO or values
//! driven) cannot be modified (locked). The LOCK bit gets cleared only by
//! system reset; it is sticky. This bit can be used for security-related
//! functionality where an authorized entity owning the GPIO can set the
//! configuration and lock it. The lock bit also covers the GPIO output vale,
//! so this may not be varied dynamically once lock is enabled.
//!
//! The GPIO controller also has masked-write registers. Values written to
//! these registers specify both a mask of bits to be updated in the
//! underlying state (the mask bits are not sticky) as well as new values
//! for that state. Individual bits of this state can be updated without
//! the need for a read-modify-write sequence. Thus different portions of
//! software can modify the GPIO controller state without coordination.
//!
//! # Implementation
//!
//! Please note that all reads and writes are issued to non-masked registers.
//!
//! - Abstraction and implementation of the GPIO registers is done with the
//! [`GpioController`], which holds an array of 8 [`GpioBank`]s. Within
//! a [`GpioBank`], the GPIOs are arranged as arrays of registers, each
//! of them with a size of 4. [`GpioController::get`] is used to create
//! pointers to the GPIO controller which is mapped at address `0x6000D000`.
//!
//! - GPIOs are represented by the [`Gpio`] struct, which holds a [`GpioPort`]
//! and a [`GpioPin`] to calculate the absolute in value, the bank the
//! GPIO is located at and the mask that is used for reads writes to the
//! registers.
//!
//! - [`Gpio`] holds pre-defined constants which represent known GPIOs.
//! These can be used for convenience.
//!
//! - [`GpioMode`], [`GpioDirection`] and [`GpioLevel`] as well as pre-defined
//! [`GpioConfig`]s can be used to fully customize and control the behavior
//! of each GPIO and to read out the configuration of a GPIO.
//!
//! - The [`gpio!`] macro is a convenience method for creating [`Gpio`] objects
//! which reduces boilerplate to a minimum.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::gpio::*;
//!
//! fn main() {
//!     let gpio = Gpio::BUTTON_VOL_DOWN;
//!
//!     match gpio.read() {
//!         GpioLevel::High => {
//!             println!("Volume Down pressed!");
//!         }
//!         GpioLevel::Low => {
//!             println!("Volume Down not pressed!");
//!         }
//!     }
//! }
//! ```
//!
//! [`GpioController`]: struct.GpioController.html
//! [`GpioController::get`]: struct.GpioController.html#method.get
//! [`GpioBank`]: struct.GpioBank.html
//! [`Gpio`]: struct.Gpio.html
//! [`GpioPort`]: struct.GpioPort.html
//! [`GpioPin`]: struct.GpioPin.html
//! [`GpioMode`]: enum.GpioMode.html
//! [`GpioDirection`]: enum.GpioDirection.html
//! [`GpioLevel`]: enum.GpioLevel.html
//! [`GpioConfig`]: enum.GpioConfig.html
//! [`gpio!`]: macro.gpio.html

pub use paste::expr;

use enum_primitive::FromPrimitive;
use mirage_mmio::{BlockMmio, VolatileStorage};

/// Base address for the GPIO registers.
pub(crate) const GPIO_BASE: u32 = 0x6000_D000;

/// The total amount of GPIO ports per bank section.
const GPIO_PORTS_COUNT: usize = 4;
/// The total amount of GPIO banks.
const GPIO_BANKS_COUNT: usize = 8;

/// The GPIO ports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioPort {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    AA,
    BB,
    CC,
    DD,
    EE,
    FF,
}

/// Representation of the GPIO pins for each port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioPin {
    P0 = 0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

enum_from_primitive! {
    /// Possible GPIO modes.
    #[derive(Debug, PartialEq, Eq)]
    pub enum GpioMode {
        /// SFIO mode.
        SFIO = 0,
        /// GPIO mode.
        GPIO = 1,
    }
}

enum_from_primitive! {
    /// Possible GPIO directions.
    #[derive(Debug, PartialEq, Eq)]
    pub enum GpioDirection {
        /// Input direction.
        Input = 0,
        /// Output direction.
        Output = 1,
    }
}

enum_from_primitive! {
    /// Possible GPIO levels.
    #[derive(Debug, PartialEq, Eq)]
    pub enum GpioLevel {
        /// Low level.
        Low = 0,
        /// High level.
        High = 1,
    }
}

/// Supported GPIO configurations.
#[derive(Debug, PartialEq, Eq)]
pub enum GpioConfig {
    Input,
    OutputLow,
    OutputHigh,
}

/// Representation of a GPIO bank.
#[repr(C)]
struct GpioBank {
    gpio_config: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_direction_out: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_out: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_in: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_int_status: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_int_enable: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_int_level: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_int_clear: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_config: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_dir_out: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_out: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_in: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_status: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_enable: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_level: [BlockMmio<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_clear: [BlockMmio<u32>; GPIO_PORTS_COUNT],
}

/// Representation of the GPIO controller.
#[repr(C)]
pub struct GpioController {
    /// The GPIO banks.
    banks: [GpioBank; GPIO_BANKS_COUNT],
}

impl VolatileStorage for GpioController {
    unsafe fn make_ptr() -> *const Self {
        GPIO_BASE as *const _
    }
}

/// Representation of a GPIO
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gpio {
    /// The GPIO port.
    pub port: GpioPort,
    /// The GPIO pin.
    pub pin: GpioPin,
}

/// A macro to facilitate the creation of a GPIO given a port and a pin.
///
/// # Example
///
/// ```
/// use mirage_libswitch::gpio::*;
///
/// let gpio = Gpio {
///     port: GpioPort::X,
///     pin: GpioPin::P7,
/// };
///
/// assert_eq!(gpio, gpio!(X, 7));
/// ```
#[macro_export]
macro_rules! gpio {
    ($port:ident, $pin:tt) => {
        $crate::gpio::Gpio {
            port: $crate::gpio::GpioPort::$port,
            pin: $crate::gpio::expr!($crate::gpio::GpioPin::[<P $pin>]),
        }
    }
}

impl Gpio {
    pub const BUTTON_VOL_DOWN: Self = Gpio {
        port: GpioPort::X,
        pin: GpioPin::P7,
    };

    pub const BUTTON_VOL_UP: Self = Gpio {
        port: GpioPort::X,
        pin: GpioPin::P6,
    };

    pub const MICROSD_CARD_DETECT: Self = Gpio {
        port: GpioPort::Z,
        pin: GpioPin::P1,
    };

    pub const MICROSD_WRITE_PROTECT: Self = Gpio {
        port: GpioPort::Z,
        pin: GpioPin::P4,
    };

    pub const MICROSD_SUPPLY_ENABLE: Self = Gpio {
        port: GpioPort::E,
        pin: GpioPin::P4,
    };

    pub const LCD_BL_P5V: Self = Gpio {
        port: GpioPort::I,
        pin: GpioPin::P0,
    };

    pub const LCD_BL_N5V: Self = Gpio {
        port: GpioPort::I,
        pin: GpioPin::P1,
    };

    pub const LCD_BL_PWM: Self = Gpio {
        port: GpioPort::V,
        pin: GpioPin::P0,
    };

    pub const LCD_BL_EN: Self = Gpio {
        port: GpioPort::V,
        pin: GpioPin::P1,
    };

    pub const LCD_BL_RST: Self = Gpio {
        port: GpioPort::V,
        pin: GpioPin::P2,
    };
}

impl Gpio {
    /// Calculates the value of the wrapped GPIO.
    #[inline]
    fn get_gpio_value(&self) -> usize {
        self.port as usize * 8 + self.pin as usize
    }

    /// Calculates the value of the wrapped GPIO port.
    #[inline]
    fn get_port_value(&self) -> usize {
        self.port as usize & 3
    }

    /// Calculates the bank where the GPIO is located.
    #[inline]
    fn get_bank(&self) -> usize {
        self.get_gpio_value() >> 5
    }

    /// Calculates the GPIO mask.
    #[inline]
    fn get_mask(&self) -> u32 {
        1 << self.pin as u32
    }

    /// Reads the flag of a GPIO register.
    #[inline]
    fn read_flag(&self, reg: &BlockMmio<u32>) -> u32 {
        (reg.read() >> self.pin as u32) & 1
    }

    /// Gets the GPIO mode the pin is currently set to.
    pub fn get_mode(&self) -> GpioMode {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to read from.
        let config_reg = &controller.banks[self.get_bank()].gpio_config
            [self.get_port_value()];

        // Read the flag and wrap it into the corresponding enum.
        GpioMode::from_u32(self.read_flag(config_reg)).unwrap()
    }

    /// Sets the GPIO mode for the pin.
    pub fn set_mode(&self, mode: GpioMode) {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to write to and the mask to be used.
        let config_reg = &controller.banks[self.get_bank()].gpio_config
            [self.get_port_value()];

        // Read the value to be modified and figure out the mask to be used.
        let mut value = config_reg.read();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match mode {
            GpioMode::GPIO => {
                value |= mask;
            },
            GpioMode::SFIO => {
                value &= !mask;
            },
        }

        // Set the new value.
        config_reg.write(value);

        // Dummy read.
        config_reg.read();
    }

    /// Gets the direction the pin is currently set to.
    pub fn get_direction(&self) -> GpioDirection {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to read from.
        let direction_reg = &controller.banks[self.get_bank()].gpio_direction_out
            [self.get_port_value()];

        // Read the flag and wrap it into the corresponding enum.
        GpioDirection::from_u32(self.read_flag(direction_reg)).unwrap()
    }

    /// Sets the direction of the pin.
    pub fn set_direction(&self, direction: GpioDirection) {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to write to and the mask to be used.
        let direction_reg = &controller.banks[self.get_bank()].gpio_direction_out
            [self.get_port_value()];

        // Read the value to be modified and the mask to be used.
        let mut value = direction_reg.read();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match direction {
            GpioDirection::Output => {
                value |= mask;
            },
            GpioDirection::Input => {
                value &= !mask;
            },
        }

        // Set the new value.
        direction_reg.write(value);

        // Dummy read.
        direction_reg.read();
    }

    /// Configures a GPIO with a pre-defined configuration.
    pub fn config(&self, config: GpioConfig) {
        self.set_mode(GpioMode::GPIO);

        match config {
            GpioConfig::Input => {
                self.set_direction(GpioDirection::Input);
            },
            GpioConfig::OutputLow => {
                self.set_direction(GpioDirection::Output);
                self.write(GpioLevel::Low);
            },
            GpioConfig::OutputHigh => {
                self.set_direction(GpioDirection::Output);
                self.write(GpioLevel::High);
            },
        }
    }

    /// Writes a level to the pin.
    pub fn write(&self, level: GpioLevel) {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to write to and the mask to be used.
        let out_reg = &controller.banks[self.get_bank()].gpio_out
            [self.get_port_value()];

        // Read the value to be modified and the mask to be used.
        let mut value = out_reg.read();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match level {
            GpioLevel::High => {
                value |= mask;
            },
            GpioLevel::Low => {
                value &= !mask;
            },
        }

        // Set the new value.
        out_reg.write(value);

        // Dummy read.
        out_reg.read();
    }

    /// Reads the GPIO level of the pin.
    pub fn read(&self) -> GpioLevel {
        let controller = unsafe { GpioController::get() };

        // Figure out the register to read from.
        let in_reg = &controller.banks[self.get_bank()].gpio_in
            [self.get_port_value()];

        // Read the flag and wrap it into the corresponding enum.
        GpioLevel::from_u32(self.read_flag(in_reg)).unwrap()
    }
}
