//! Tegra210 KFuse implementation.

use register::mmio::ReadWrite;

use crate::clock::Clock;

const KFUSE_BASE: u32 = 0x7000_FC00;

const KFUSE_STATE_DONE: u32 = (1 << 16);
const KFUSE_STATE_CRCPASS: u32 = (1 << 17);

const KFUSE_KEYADDR_AUTOINC: u32 = (1 << 16);

pub const KFUSE_NUM_WORDS: u32 = 144;

/// Reads the KFuse contents into a buffer.
#[optimize(size)]
pub fn read(buffer: &mut [u32]) -> Result<(), ()> {
    let state_reg = unsafe { &(*((KFUSE_BASE + 0x80) as *const ReadWrite<u32>)) };
    let keyaddr_reg = unsafe { &(*((KFUSE_BASE + 0x88) as *const ReadWrite<u32>)) };
    let keys_reg = unsafe { &(*((KFUSE_BASE + 0x8C) as *const ReadWrite<u32>)) };

    Clock::KFUSE.enable();

    while !(state_reg.get() & KFUSE_STATE_DONE) {}

    if !(state_reg.get() & KFUSE_STATE_CRCPASS) {
        Clock::KFUSE.disable();
        return Err(());
    }

    keyaddr_reg.set(KFUSE_KEYADDR_AUTOINC);

    for i in 0..KFUSE_NUM_WORDS {
        buffer[i] = keys_reg.get();
    }

    Clock::KFUSE.disable();

    Ok(())
}
