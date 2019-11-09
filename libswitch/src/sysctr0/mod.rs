//! SYSCTR0 control registers.
//!
//! Also referred to as PMC Counter 0 registers.

/// Base address for SYSCTR0 registers.
const SYSCTR0_BASE: u32 = 0x700F_0000;

register!(CNTCR, SYSCTR0_BASE + 0x0);

register!(CNTSR, SYSCTR0_BASE + 0x4);

register!(CNTCV0, SYSCTR0_BASE + 0x8);

register!(CNTCV1, SYSCTR0_BASE + 0xC);

register!(CNTFID0, SYSCTR0_BASE + 0x20);

register!(CNTFID1, SYSCTR0_BASE + 0x24);

register!(COUNTERID4, SYSCTR0_BASE + 0xFD0);

register!(COUNTERID5, SYSCTR0_BASE + 0xFD4);

register!(COUNTERID6, SYSCTR0_BASE + 0xFD8);

register!(COUNTERID7, SYSCTR0_BASE + 0xFDC);

register!(COUNTERID0, SYSCTR0_BASE + 0xFE0);

register!(COUNTERID1, SYSCTR0_BASE + 0xFE4);

register!(COUNTERID2, SYSCTR0_BASE + 0xFE8);

register!(COUNTERID3, SYSCTR0_BASE + 0xFEC);

register!(COUNTERID8, SYSCTR0_BASE + 0xFF0);

register!(COUNTERID9, SYSCTR0_BASE + 0xFF4);

register!(COUNTERID10, SYSCTR0_BASE + 0xFF8);

register!(COUNTERID11, SYSCTR0_BASE + 0xFFC);
