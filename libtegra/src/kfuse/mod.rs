//! Tegra210 KFUSE implementation.

use mirage_mmio::{Mmio, VolatileStorage};

use crate::clock::Clock;

pub(crate) const KFUSE_BASE: u32 = 0x7000_FC00;

pub(crate) const KFUSE_STATE_DONE: u32 = (1 << 16);
pub(crate) const KFUSE_STATE_CRCPASS: u32 = (1 << 17);

pub(crate) const KFUSE_KEYADDR_AUTOINC: u32 = (1 << 16);

pub const KFUSE_NUM_WORDS: u32 = 144;

/// Representation of the HDCP KFUSE registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct KfuseRegisters {
    /// The `KFUSE_STATE_0` register.
    pub STATE: Mmio<u32>,
    /// The `KFUSE_ERRCOUNT_0` register.
    pub ERRCOUNT: Mmio<u32>,
    /// The `KFUSE_KEYADDR_0` register.
    pub KEYADDR: Mmio<u32>,
    /// The `KFUSE_KEYS_0` register.
    pub KEYS: Mmio<u32>,
    _unk: [Mmio<u32>; 25],
    /// The `KFUSE_PD_0` register.
    pub PD: Mmio<u32>,
}

impl VolatileStorage for KfuseRegisters {
    unsafe fn make_ptr() -> *const Self {
        (KFUSE_BASE + 0x80) as *const _
    }
}

/// Reads the KFuse contents into a buffer.
#[optimize(size)]
pub fn read(buffer: &mut [u32]) -> Result<(), ()> {
    let registers = unsafe { KfuseRegisters::get() };

    Clock::KFUSE.enable();

    while (registers.STATE.read() & KFUSE_STATE_DONE) == 0 {
        // Wait.
    }

    if (registers.STATE.read() & KFUSE_STATE_CRCPASS) == 0 {
        Clock::KFUSE.disable();
        return Err(());
    }

    registers.KEYADDR.write(KFUSE_KEYADDR_AUTOINC);

    for i in 0..KFUSE_NUM_WORDS {
        buffer[i as usize] = registers.KEYS.read();
    }

    Clock::KFUSE.disable();

    Ok(())
}
