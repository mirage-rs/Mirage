//! SYSCTR0 control registers.
//!
//! Also referred to as PMC Counter 0 registers.

use mirage_mmio::{BlockMmio, VolatileStorage};

/// Base address for SYSCTR0 registers.
pub(crate) const SYSCTR0_BASE: u32 = 0x700F_0000;

/// Representation of the PMC Counter 0 registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Sysctr0Registers {
    /// The `SYSCTR0_CNTCR_0` register.
    pub CNTCR: BlockMmio<u32>,
    /// The `SYSCTR0_CNTSR_0` register.
    pub CNTSR: BlockMmio<u32>,
    /// The `SYSCTR0_CNTCV0_0` register.
    pub CNTCV0: BlockMmio<u32>,
    /// The `SYSCTR0_CNTCV1_0` register.
    pub CNTCV1: BlockMmio<u32>,
    _unk1: [BlockMmio<u32>; 0x4],
    /// The `SYSCTR0_CNTFID0_0` register.
    pub CNTFID0: BlockMmio<u32>,
    /// The `SYSCTR0_CNTFID1_0` register.
    pub CNTFID1: BlockMmio<u32>,
    _unk2: [BlockMmio<u32>; 0x3EA],
    /// The `SYSCTR0_COUNTERID4_0` register.
    pub COUNTERID4: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID5_0` register.
    pub COUNTERID5: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID6_0` register.
    pub COUNTERID6: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID7_0` register.
    pub COUNTERID7: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID0_0` register.
    pub COUNTERID0: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID1_0` register.
    pub COUNTERID1: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID2_0` register.
    pub COUNTERID2: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID3_0` register.
    pub COUNTERID3: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID8_0` register.
    pub COUNTERID8: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID9_0` register.
    pub COUNTERID9: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID10_0` register.
    pub COUNTERID10: BlockMmio<u32>,
    /// The `SYSCTR0_COUNTERID11_0` register.
    pub COUNTERID11: BlockMmio<u32>,
}

impl VolatileStorage for Sysctr0Registers {
    unsafe fn make_ptr() -> *const Self {
        SYSCTR0_BASE as *const _
    }
}
