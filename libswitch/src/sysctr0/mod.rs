//! SYSCTR0 control registers.
//!
//! Also referred to as PMC Counter 0 registers.

/// Base address for SYSCTR0 registers.
const SYSCTR0_BASE: u32 = 0x700F_0000;

/// The `SYSCTR0_CNTCR_0` register.
register!(CNTCR, SYSCTR0_BASE + 0x0);

/// The `SYSCTR0_CNTSR_0` register.
register!(CNTSR, SYSCTR0_BASE + 0x4);

/// The `SYSCTR0_CNTCV0_0` register.
register!(CNTCV0, SYSCTR0_BASE + 0x8);

/// The `SYSCTR0_CNTCV1_0` register.
register!(CNTCV1, SYSCTR0_BASE + 0xC);

/// The `SYSCTR0_CNTFID0_0` register.
register!(CNTFID0, SYSCTR0_BASE + 0x20);

/// The `SYSCTR0_CNTFID1_0` register.
register!(CNTFID1, SYSCTR0_BASE + 0x24);

/// The `SYSCTR0_COUNTERID4_0` register.
register!(COUNTERID4, SYSCTR0_BASE + 0xFD0);

/// The `SYSCTR0_COUNTERID5_0` register.
register!(COUNTERID5, SYSCTR0_BASE + 0xFD4);

/// The `SYSCTR0_COUNTERID6_0` register.
register!(COUNTERID6, SYSCTR0_BASE + 0xFD8);

/// The `SYSCTR0_COUNTERID7_0` register.
register!(COUNTERID7, SYSCTR0_BASE + 0xFDC);

/// The `SYSCTR0_COUNTERID0_0` register.
register!(COUNTERID0, SYSCTR0_BASE + 0xFE0);

/// The `SYSCTR0_COUNTERID1_0` register.
register!(COUNTERID1, SYSCTR0_BASE + 0xFE4);

/// The `SYSCTR0_COUNTERID2_0` register.
register!(COUNTERID2, SYSCTR0_BASE + 0xFE8);

/// The `SYSCTR0_COUNTERID3_0` register.
register!(COUNTERID3, SYSCTR0_BASE + 0xFEC);

/// The `SYSCTR0_COUNTERID8_0` register.
register!(COUNTERID8, SYSCTR0_BASE + 0xFF0);

/// The `SYSCTR0_COUNTERID9_0` register.
register!(COUNTERID9, SYSCTR0_BASE + 0xFF4);

/// The `SYSCTR0_COUNTERID10_0` register.
register!(COUNTERID10, SYSCTR0_BASE + 0xFF8);

/// The `SYSCTR0_COUNTERID11_0` register.
register!(COUNTERID11, SYSCTR0_BASE + 0xFFC);
