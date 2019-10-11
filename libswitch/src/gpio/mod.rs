//! Interface to control the Tegra210 GPIO pins.

use core::fmt;

use enum_primitive::FromPrimitive;
use register::mmio::ReadWrite;

const GPIO_PORTS_COUNT: usize = 4;
const GPIO_BANKS_COUNT: usize = 8;

/// The GPIO ports.
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Debug, Clone)]
pub enum GpioPinOffset {
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
    pub enum GpioMode {
        SFIO = 0,
        GPIO = 1,
    }
}

enum_from_primitive! {
    /// Possible GPIO directions.
    pub enum GpioDirection {
        Input = 0,
        Output = 1,
    }
}

enum_from_primitive! {
    /// Possible GPIO levels.
    pub enum GpioLevel {
        Low = 0,
        High = 1,
    }
}

/// GPIO configurations.
#[derive(Copy, Clone)]
pub enum GpioConfig {
    Input,
    OutputLow,
    OutputHigh,
}

#[repr(C)]
struct GpioBank {
    gpio_config: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_direction_out: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_out: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_in: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_int_status: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_int_enable: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_int_level: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_int_clear: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_config: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_dir_out: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_out: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_in: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_status: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_enable: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_level: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    gpio_masked_int_clear: [ReadWrite<u32>; GPIO_PORTS_COUNT],
}

/// Representation of the GPIO controller.
#[repr(C)]
pub struct GpioController {
    banks: [GpioBank; GPIO_BANKS_COUNT],
}

impl GpioController {
    /// Gets the controller.
    pub fn get() -> *const Self {
        0x6000_D000 as *const GpioController
    }
}

/// A GPIO pin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpioPin {
    pub port: GpioPort,
    pub offset: GpioPinOffset,
}

impl GpioPin {
    pub const BUTTON_VOL_DOWN: Self = GpioPin {
        port: GpioPort::X,
        offset: GpioPinOffset::P7,
    };

    pub const BUTTON_VOL_UP: Self = GpioPin {
        port: GpioPort::X,
        offset: GpioPinOffset::P6,
    };

    pub const MICROSD_CARD_DETECT: Self = GpioPin {
        port: GpioPort::Z,
        offset: GpioPinOffset::P1,
    };

    pub const MICROSD_WRITE_PROTECT: Self = GpioPin {
        port: GpioPort::Z,
        offset: GpioPinOffset::P4,
    };

    pub const MICROSD_SUPPLY_ENABLE: Self = GpioPin {
        port: GpioPort::E,
        offset: GpioPinOffset::P4,
    };

    pub const LCD_BL_P5V: Self = GpioPin {
        port: GpioPort::I,
        offset: GpioPinOffset::P0,
    };

    pub const LCD_BL_N5V: Self = GpioPin {
        port: GpioPort::I,
        offset: GpioPinOffset::P1,
    };

    pub const LCD_BL_PWM: Self = GpioPin {
        port: GpioPort::V,
        offset: GpioPinOffset::P0,
    };

    pub const LCD_BL_EN: Self = GpioPin {
        port: GpioPort::V,
        offset: GpioPinOffset::P1,
    };

    pub const LCD_BL_RST: Self = GpioPin {
        port: GpioPort::V,
        offset: GpioPinOffset::P2,
    };
}

impl GpioPin {
    fn get_pin_value(&self) -> usize {
        ((self.port as usize) * 8 + self.offset as usize)
    }

    fn get_bank(&self) -> usize {
        self.get_pin_value() >> 5
    }

    fn read_flag(&self, reg: &ReadWrite<u32, ()>) -> u32 {
        (reg.get() >> self.offset as u32) & 1
    }

    /// Gets the GPIO mode the pin is currently set to.
    pub fn get_mode(&self) -> GpioMode {
        let controller = GpioController::get();

        let config_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_config
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };

        GpioMode::from_u32(self.read_flag(config_reg)).unwrap()
    }

    /// Sets the GPIO mode for the pin.
    pub fn set_mode(&self, mode: GpioMode) {
        let controller = GpioController::get();

        let config_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_config
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };
        let mut value = config_reg.get();

        match mode {
            GpioMode::SFIO => {
                value |= (1 << self.offset as u32);
            }
            GpioMode::GPIO => {
                value &= !(1 << self.offset as u32);
            }
        }

        config_reg.set(value);

        config_reg.get(); // Dummy read.
    }

    /// Gets the direction the pin is currently set to.
    pub fn get_direction(&self) -> GpioDirection {
        let controller = GpioController::get();

        let direction_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_direction_out
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };

        GpioDirection::from_u32(self.read_flag(direction_reg)).unwrap()
    }

    /// Sets the direction of the pin.
    pub fn set_direction(&self, direction: GpioDirection) {
        let controller = GpioController::get();

        let direction_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_direction_out
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };
        let mut value = direction_reg.get();

        match direction {
            GpioDirection::Input => {
                value &= !(1 << self.offset as u32);
            }
            GpioDirection::Output => {
                value |= (1 << self.offset as u32);
            }
        }

        direction_reg.set(value);

        direction_reg.get(); // Dummy read.
    }

    /// Configures the pin.
    pub fn config(&self, config: GpioConfig) {
        self.set_mode(GpioMode::GPIO);

        match config {
            GpioConfig::Input => {
                self.set_direction(GpioDirection::Input);
            }
            GpioConfig::OutputLow => {
                self.set_direction(GpioDirection::Output);
                self.write(GpioLevel::Low);
            }
            GpioConfig::OutputHigh => {
                self.set_direction(GpioDirection::Output);
                self.write(GpioLevel::High);
            }
        }
    }

    /// Writes a level to the pin.
    pub fn write(&self, level: GpioLevel) {
        let controller = GpioController::get();

        let out_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_out
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };
        let mut value = out_reg.get();

        match level {
            GpioLevel::Low => {
                value &= !(1 << self.offset as u32);
            }
            GpioLevel::High => {
                value |= (1 << self.offset as u32);
            }
        }

        out_reg.set(value);
    }

    /// Reads the GPIO level of the pin.
    pub fn read(&self) -> GpioLevel {
        let controller = GpioController::get();

        let in_reg = unsafe {
            &(*controller).banks[self.get_bank()].gpio_in
                [self.port as usize & (GPIO_PORTS_COUNT - 1)]
        };

        GpioLevel::from_u32(self.read_flag(in_reg)).unwrap()
    }
}

impl fmt::Display for GpioPin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!("pin: {}, port: {}, bank: {}, ")
    }
}
