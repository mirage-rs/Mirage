//! Interface to the Tegra210 RTC and the timers.
//!
//! # Description
//!
//! The Real-Time Clock (RTC) module maintains seconds and milliseconds counters,
//! and five alarm registers. The RTC is in the 'always on' power domain, allowing
//! for the counters to run and alarms to trigger when the system is in low-power
//! state. If configured, interrupts triggered by the RTC can cause the system to
//! wake up from a low-power state.
//!
//! The Fixed Time Base Registers meanwhile provide a fixed time base in
//! microseconds to be used by the rest of the system regardless of the
//! clk_m frequency.
//!
//! # Implementation
//!
//! - The most important RTC and Timers registers are exposed as global constants
//! within the crate.
//!
//! - The functions [`get_seconds`], [`get_milliseconds`] and [`get_microseconds`]
//! can be used to retrieve the current time.
//!
//! - The functions [`sleep`], [`msleep`] and [`usleep`] are built on top of this
//! to cause blocking delays.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::timer::sleep;
//!
//! fn main() {
//!     sleep(5);
//!     println!("Five seconds later.");
//! }
//! ```
//!
//! [`get_seconds`]: fn.get_seconds.html
//! [`get_milliseconds`]: fn.get_milliseconds.html
//! [`get_microseconds`]: fn.get_microseconds.html
//! [`sleep`]: fn.sleep.html
//! [`msleep`]: fn.msleep.html
//! [`usleep`]: fn.usleep.html

use register::mmio::ReadWrite;

/// Base address for Timer registers.
const TIMERS_BASE: u32 = 0x6000_5000;

/// The `TIMERUS_CNTR_1US_0` register.
pub(crate) const TIMERUS_CNTR_1US: &'static ReadWrite<u32> =
    unsafe { &(*((TIMERS_BASE + 0x10) as *const ReadWrite<u32>)) };

/// The `TIMERUS_USEC_CFG_0` register.
pub(crate) const TIMERUS_USEC_CFG: &'static ReadWrite<u32> =
    unsafe { &(*((TIMERS_BASE + 0x14) as *const ReadWrite<u32>)) };

/// Base address for RTC registers.
const RTC_BASE: u32 = 0x7000_E000;

/// The `APBDEV_RTC_SECONDS_0` register.
pub(crate) const RTC_SECONDS: &'static ReadWrite<u32> =
    unsafe { &(*((RTC_BASE + 0x8) as *const ReadWrite<u32>)) };

/// The `APBDEV_RTC_SHADOW_SECONDS_0` register.
pub(crate) const RTC_SHADOW_SECONDS: &'static ReadWrite<u32> =
    unsafe { &(*((RTC_BASE + 0xC) as *const ReadWrite<u32>)) };

/// The `APBDEV_RTC_MILLI_SECONDS_0` register.
pub(crate) const RTC_MILLI_SECONDS: &'static ReadWrite<u32> =
    unsafe { &(*((RTC_BASE + 0x10) as *const ReadWrite<u32>)) };

/// Returns the current time in seconds.
#[inline]
pub fn get_seconds() -> u32 {
    RTC_SECONDS.get()
}

/// Returns the current time in milliseconds.
#[inline]
pub fn get_milliseconds() -> u32 {
    RTC_MILLI_SECONDS.get() | (RTC_SHADOW_SECONDS.get() << 10)
}

/// Returns the current time in microseconds.
#[inline]
pub fn get_microseconds() -> u32 {
    TIMERUS_CNTR_1US.get()
}

/// Sleeps for a given duration in seconds.
#[inline]
pub fn sleep(duration: u32) {
    let start = get_seconds();

    while (get_seconds() - start) < duration {}
}

/// Sleeps for a given duration in milliseconds.
#[inline]
pub fn msleep(duration: u32) {
    let start = get_milliseconds();

    while (get_milliseconds() - start) < duration {}
}

/// Sleeps for a given duration in microseconds.
#[inline]
pub fn usleep(duration: u32) {
    let start = get_microseconds();

    while (get_microseconds() - start) < duration {}
}
