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
//! - The RTC and Fixed Time registers are exposed within the structures
//! [`TimerRegisters`] and [`RtcRegisters`].
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
//! use mirage_libtegra::timer::sleep;
//!
//! fn main() {
//!     sleep(5);
//!     println!("Five seconds later.");
//! }
//! ```
//!
//! [`TimerRegisters`]: struct.TimerRegisters.html
//! [`RtcRegisters`]: struct.RtcRegisters.html
//! [`get_seconds`]: fn.get_seconds.html
//! [`get_milliseconds`]: fn.get_milliseconds.html
//! [`get_microseconds`]: fn.get_microseconds.html
//! [`sleep`]: fn.sleep.html
//! [`msleep`]: fn.msleep.html
//! [`usleep`]: fn.usleep.html

use mirage_mmio::{Mmio, VolatileStorage};

/// Base address for Timer registers.
pub(crate) const TIMERS_BASE: u32 = 0x6000_5000;

/// Representation of the Fixed Time Base registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct TimerRegisters {
    /// The `TIMERUS_CNTR_1US_0` register.
    pub TIMERUS_CNTR_1US: Mmio<u32>,
    /// The `TIMERUS_USEC_CFG_0` register.
    pub TIMERUS_USEC_CFG: Mmio<u32>,
    _reserved: [Mmio<u32>; 0xD],
    /// The `TIMERUS_CNTR_FREEZE_0` register.
    pub TIMERUS_CNTR_FREEZE: Mmio<u32>,
}

impl VolatileStorage for TimerRegisters {
    unsafe fn make_ptr() -> *const Self {
        (TIMERS_BASE + 0x10) as *const _
    }
}

/// Base address for RTC registers.
pub(crate) const RTC_BASE: u32 = 0x7000_E000;

/// Representation of the RTC registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct RtcRegisters {
    /// The `APBDEV_RTC_CONTROL_0` register.
    pub RTC_CONTROL: Mmio<u32>,
    /// The `APBDEV_RTC_BUSY_0` register.
    pub RTC_BUSY: Mmio<u32>,
    /// The `APBDEV_RTC_SECONDS_0` register.
    pub RTC_SECONDS: Mmio<u32>,
    /// The `APBDEV_RTC_SHADOW_SECONDS_0` register.
    pub RTC_SHADOW_SECONDS: Mmio<u32>,
    /// The `APBDEV_RTC_MILLI_SECONDS_0` register.
    pub RTC_MILLI_SECONDS: Mmio<u32>,
    /// The `APBDEV_RTC_SECONDS_ALARM0_0` register.
    pub RTC_SECONDS_ALARM0: Mmio<u32>,
    /// The `APBDEV_RTC_SECONDS_ALARM1_0` register.
    pub RTC_SECONDS_ALARM1: Mmio<u32>,
    /// The `APBDEV_RTC_MILLI_SECONDS_ALARM_0` register.
    pub RTC_MILLI_SECONDS_ALARM: Mmio<u32>,
    /// The `APBDEV_RTC_SECONDS_COUNTDOWN_ALARM_0` register.
    pub RTC_SECONDS_COUNTDOWN_ALARM: Mmio<u32>,
    /// The `APBDEV_RTC_MILLI_SECONDS_COUNTDOWN_ALARM_0` register.
    pub RTC_MILLI_SECONDS_COUNTDOWN_ALARM: Mmio<u32>,
    /// The `APBDEV_RTC_INTR_MASK_0` register.
    pub RTC_INTR_MASK: Mmio<u32>,
    /// The `APBDEV_RTC_INTR_STATUS_0` register.
    pub RTC_INTR_STATUS: Mmio<u32>,
    /// The `APBDEV_RTC_INTR_SOURCE_0` register.
    pub RTC_INTR_SOURCE: Mmio<u32>,
    /// The `APBDEV_RTC_INTR_SET_0` register.
    pub RTC_INTR_SET: Mmio<u32>,
    /// The `APBDEV_RTC_CORRECTION_FACTOR_0` register.
    pub RTC_CORRECTION_FACTOR: Mmio<u32>,
}

impl VolatileStorage for RtcRegisters {
    unsafe fn make_ptr() -> *const Self {
        RTC_BASE as *const _
    }
}

/// Returns the current time in seconds.
#[inline(always)]
pub fn get_seconds() -> u32 {
    let rtc = unsafe { RtcRegisters::get() };

    rtc.RTC_SECONDS.read()
}

/// Returns the current time in milliseconds.
#[inline(always)]
pub fn get_milliseconds() -> u32 {
    let rtc = unsafe { RtcRegisters::get() };

    rtc.RTC_MILLI_SECONDS.read() | (rtc.RTC_SHADOW_SECONDS.read() * 1000)
}

/// Returns the current time in microseconds.
#[inline(always)]
pub fn get_microseconds() -> u32 {
    let timer = unsafe { TimerRegisters::get() };

    timer.TIMERUS_CNTR_1US.read()
}

/// Gets the time that has passed since a given [`get_microseconds`].
///
/// [`get_microseconds`]: fn.get_microseconds.html
#[inline]
pub fn get_time_since(base: u32) -> u32 {
    get_microseconds() - base
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
