//! Interface to the Tegra210 Real-Time Clock and the timers.

use register::mmio::ReadWrite;

pub mod watchdog;

/// Returns the current time in seconds.
pub fn get_seconds() -> u32 {
    let rtc_seconds = unsafe { &(*(0x7000_E008 as *const ReadWrite<u32>)) };

    rtc_seconds.get()
}

/// Returns the current time in milliseconds.
pub fn get_milliseconds() -> u32 {
    let rtc_milli_seconds = unsafe { &(*(0x7000_E010 as *const ReadWrite<u32>)) };
    let rtc_shadow_seconds = unsafe { &(*(0x7000_E00C as *const ReadWrite<u32>)) };

    rtc_milli_seconds.get() | (rtc_shadow_seconds.get() << 10)
}

/// Returns the current time in microseconds.
pub fn get_microseconds() -> u32 {
    let timerus_cntr_1us_0 = unsafe { &(*(0x6000_5010 as *const ReadWrite<u32>)) };

    timerus_cntr_1us_0.get()
}

/// Sleeps for a given duration in seconds.
pub fn sleep(duration: u32) {
    let rtc_seconds = unsafe { &(*(0x7000_E008 as *const ReadWrite<u32>)) };

    let start = rtc_seconds.get();

    while (rtc_seconds.get() - start) < duration {}
}

/// Sleeps for a given duration in milliseconds.
pub fn msleep(duration: u32) {
    let rtc_milli_seconds = unsafe { &(*(0x7000_E010 as *const ReadWrite<u32>)) };
    let rtc_shadow_seconds = unsafe { &(*(0x7000_E00C as *const ReadWrite<u32>)) };

    let start = rtc_milli_seconds.get() | (rtc_shadow_seconds.get() << 10);

    while ((rtc_milli_seconds.get() | (rtc_shadow_seconds.get() << 10)) - start) < duration {}
}

/// Sleeps for a given duration in microseconds.
pub fn usleep(duration: u32) {
    let timerus_cntr_1us_0 = unsafe { &(*(0x6000_5010 as *const ReadWrite<u32>)) };

    let start = timerus_cntr_1us_0.get();

    while (timerus_cntr_1us_0.get() - start) < duration {}
}
