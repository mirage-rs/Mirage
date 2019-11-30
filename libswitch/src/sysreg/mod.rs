//! Tegra210 system registers.

/// Base address for system registers.
pub(crate) const SYSREG_BASE: u32 = 0x6000_C000;

/// Base address for SB registers.
pub(crate) const SB_BASE: u32 = SYSREG_BASE + 0x200;

/// Base address for Exception Vector registers.
pub(crate) const EXCEPTION_VECTOR_BASE: u32 = 0x6000F000;

register!(AHB_ARBITRATION_DISABLE_0, SYSREG_BASE + 0x004);

register!(AHB_ARBITRATION_XBAR_CTRL_0, SYSREG_BASE + 0x0E0);

register!(AHB_AHB_SPARE_REG_0, SYSREG_BASE + 0x110);

register!(SB_CSR_0, SB_BASE);

register!(SB_PIROM_START_0, SB_BASE + 0x04);

register!(SB_PFCFG_0, SB_BASE + 0x08);

register!(SB_SECURE_SPAREREG_0_0, SB_BASE + 0x0C);

register!(SB_SECURE_SPAREREG_1_0, SB_BASE + 0x10);

register!(SB_SECURE_SPAREREG_2_0, SB_BASE + 0x14);

register!(SB_SECURE_SPAREREG_3_0, SB_BASE + 0x18);

register!(SB_SECURE_SPAREREG_4_0, SB_BASE + 0x1C);

register!(SB_SECURE_SPAREREG_5_0, SB_BASE + 0x20);

register!(SB_SECURE_SPAREREG_6_0, SB_BASE + 0x24);

register!(SB_SECURE_SPAREREG_7_0, SB_BASE + 0x28);

register!(SB_AA64_RESET_LOW_0, SB_BASE + 0x30);

register!(SB_AA64_RESET_HIGH_0, SB_BASE + 0x34);
