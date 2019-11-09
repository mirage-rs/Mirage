//! PMIC Real-Time Clock driver for the Switch's Maxim77620-RTC.
//!
//! # Description
//!
//! The Maxim77620 exposes an additional interface to the RTC over I²C
//! for reading current time as accurate as possible.
//!
//! # Implementation
//!
//! - The [`RtcTime`] is a representation of such an RTC time.
//! All the attributes are publicly accessible for people who
//! want to do formatting or comparisons.
//!
//! - [`RtcTime::new`] crates a new instance of this struct, using the
//! RTC values that were read over the I2C 5 controller.
//!
//! - The [Display] trait is implemented for a human-readable
//! representation of the current point in time.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::rtc::RtcTime;
//!
//! fn main() {
//!     let time = RtcTime::new();
//!
//!     println!("{:?}", time); // Saturday, November 09, 2019 17:39:36
//! }
//! ```
//!
//! [`RtcTime`]: struct.RtcTime.html
//! [`RtcTime::new`]: struct.RtcTime.html#method.new
//! [`Display`]: https://doc.rust-lang.org/core/fmt/trait.Display.html

use core::fmt;

use crate::i2c::{I2c, MAX77620_RTC_I2C_ADDR};

/// Representation of a point in time as provided by the RTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RtcTime {
    /// The current year.
    pub year: u16,
    /// The current month.
    pub month: u8,
    /// The current day.
    pub day: u8,
    /// The current hour of the day.
    pub hour: u8,
    /// The current minute of the hour.
    pub minute: u8,
    /// The current second of the minute.
    pub second: u8,
    /// The current weekday.
    pub weekday: u8,
}

impl RtcTime {
    /// Constructor which reads the time from the RTC.
    pub fn now() -> Self {
        // Update RTC registers from RTC clock.
        I2c::C5
            .write_byte(MAX77620_RTC_I2C_ADDR, 0x04, 0x10)
            .unwrap();

        // Get control register config.
        let control_config = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x03).unwrap();

        // Get time.
        let mut hour = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x09).unwrap() & 0x1F;

        if control_config & 0x02 == 0 && hour & 0x40 != 0 {
            hour = (hour & 0xF) + 12;
        }

        let minute = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x08).unwrap() & 0x7F;
        let second = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x07).unwrap() & 0x7F;

        // Get day of week.
        let mut weekday = 0;
        let mut remainder = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x0A).unwrap();
        for i in 0..8 {
            weekday += 1;

            if remainder & 1 != 0 {
                break;
            }

            remainder >>= 1;
        }

        // Get date.
        let mut year = [0; 2];
        I2c::C5
            .read(MAX77620_RTC_I2C_ADDR, 0x0C, &mut year)
            .unwrap();
        let year = (u16::from_le_bytes(year) & 0x7F) + 2000;

        let month = (I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x0B).unwrap() & 0xF) - 1;
        let day = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x0D).unwrap() & 0x1F;

        RtcTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            weekday,
        }
    }

    /// Gets a human-readable representation of the month.
    fn get_month(&self) -> &str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        }
    }

    /// Gets a human-readable representation of the weekday.
    fn get_weekday(&self) -> &str {
        match self.weekday {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "Unknown",
        }
    }
}

impl fmt::Display for RtcTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} {}, {} {}:{}:{}",
            self.get_weekday(),
            self.get_month(),
            self.day,
            self.year,
            self.hour,
            self.minute,
            self.second
        )
    }
}
