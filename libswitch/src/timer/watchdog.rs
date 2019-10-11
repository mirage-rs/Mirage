use register::mmio::ReadWrite;

/// Representation of a Watchdog timer (WDT).
#[repr(C)]
pub struct Watchdog {
    config: ReadWrite<u32>,
    status: ReadWrite<u32>,
    command: ReadWrite<u32>,
    pattern: ReadWrite<u32>,
}

impl Watchdog {
    /// Retrieves a Watchdog timer given an identifier.
    pub fn get(identifier: u32) -> *const Self {
        (0x60000_5100 + 0x20 * identifier) as *const Watchdog
    }

    /// Reboots the Watchdog timer given the CPU core ID it is running on.
    pub fn reboot(&self, core: u32) -> ! {
        let pattern = unsafe { &(*self).pattern };
        let command = unsafe { &(*self).command };
        let config = unsafe { &(*self).config };

        // Set reboot pattern.
        pattern.set(0xC45A);

        // Disable counter.
        command.set(2);

        let wdt_reboot_cfg_reg =
            unsafe { &(*((0x6000_5060 + 0x8 * core) as *const ReadWrite<u32>)) };
        wdt_reboot_cfg_reg.set(0xC000_0000);

        // Full system reset.
        config.set(0x8015 + core);

        // Enable counter.
        command.set(1);

        loop {
            // Wait for reboot.
        }
    }
}
