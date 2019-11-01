//! SYSCTR0 control registers.
//!
//! Also referred to as PMC Counter 0 registers.

use register::mmio::ReadWrite;

/// Base address for SYSCTR0 registers.
const SYSCTR0_BASE: u32 = 0x700F_0000;

/// The `SYSCTR0_CNTCR_0` register.
pub(crate) const CNTCR: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0x0) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_CNTSR_0` register.
pub(crate) const CNTSR: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0x4) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_CNTCV0_0` register.
pub(crate) const CNTCV0: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0x8) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_CNTCV1_0` register.
pub(crate) const CNTCV1: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xC) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_CNTFID0_0` register.
pub(crate) const CNTFID0: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0x20) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_CNTFID1_0` register.
pub(crate) const CNTFID1: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0x24) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID4_0` register.
pub(crate) const COUNTERID4: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFD0) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID5_0` register.
pub(crate) const COUNTERID5: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFD4) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID6_0` register.
pub(crate) const COUNTERID6: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFD8) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID7_0` register.
pub(crate) const COUNTERID7: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFDC) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID0_0` register.
pub(crate) const COUNTERID0: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFE0) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID1_0` register.
pub(crate) const COUNTERID1: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFE4) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID2_0` register.
pub(crate) const COUNTERID2: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFE8) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID3_0` register.
pub(crate) const COUNTERID3: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFEC) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID8_0` register.
pub(crate) const COUNTERID8: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFF0) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID9_0` register.
pub(crate) const COUNTERID9: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFF4) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID10_0` register.
pub(crate) const COUNTERID10: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFF8) as *const ReadWrite<u32>)) };

/// The `SYSCTR0_COUNTERID11_0` register.
pub(crate) const COUNTERID11: &'static ReadWrite<u32> =
    unsafe { &(*((SYSCTR0_BASE + 0xFFC) as *const ReadWrite<u32>)) };
