//! Tegra210 system registers.

use mirage_mmio::{Mmio, VolatileStorage};

/// Base address for system registers.
pub(crate) const SYSREG_BASE: u32 = 0x6000_C000;

/// Base address for AHB Arbiter registers.
pub(crate) const AHB_BASE: u32 = SYSREG_BASE + 0x4;

/// Base address for SB registers.
pub(crate) const SB_BASE: u32 = SYSREG_BASE + 0x200;

/// Base address for Exception Vector registers.
pub(crate) const EXCEPTION_VECTOR_BASE: u32 = SYSREG_BASE + 0x3000;

/// Representation of the AHB Arbiter registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct AhbRegisters {
    /// The `ARBITRATION_DISABLE_0` register.
    pub ARBITRATION_DISABLE: Mmio<u32>,
    /// The `AHB_ARBITRATION_PRIORITY_CTRL_0` register.
    pub ARBITRATION_PRIORITY_CTRL: Mmio<u32>,
    /// The `AHB_ARBITRATION_USR_PROTECT_0` register.
    pub ARBITRATION_USR_PROTECT: Mmio<u32>,
    /// The `AHB_GIZMO_AHB_MEM_0` register.
    pub GIZMO_AHB_MEM: Mmio<u32>,
    /// The `AHB_GIZMO_APB_DMA_0` register.
    pub GIZMO_APB_DMA: Mmio<u32>,
    /// The `AHB_MASTER_SWID_0` register.
    pub MASTER_SWID_0: Mmio<u32>,
    _unk1: Mmio<u32>,
    /// The `AHB_GIZMO_USB_0` register.
    pub GIZMO_USB: Mmio<u32>,
    /// The `AHB_GIZMO_AHB_XBAR_BRIDGE_0` register.
    pub GIZMO_AHB_XBAR_BRIDGE: Mmio<u32>,
    /// The `AHB_GIZMO_CPU_AHB_BRIDGE_0` register.
    pub GIZMO_CPU_AHB_BRIDGE: Mmio<u32>,
    /// The `AHB_GIZMO_COP_AHB_BRIDGE_0` register.
    pub GIZMO_COP_AHB_BRIDGE: Mmio<u32>,
    /// The `AHB_GIZMO_XBAR_APB_CTLR_0` register.
    pub GIZMO_XBAR_APB_CTRL: Mmio<u32>,
    _unk2: Mmio<u32>,
    /// The `AHB_MASTER_SWID_1` register.
    pub MASTER_SWID_1: Mmio<u32>,
    _unk3: [Mmio<u32>; 0x5],
    /// The `AHB_GIZMO_SE_0` register.
    pub GIZMO_SE: Mmio<u32>,
    /// The `AHB_GIZMO_TZRAM_0` register.
    pub GIZMO_TZRAM: Mmio<u32>,
    _unk4: [Mmio<u32>; 0x9],
    /// The `AHB_GIZMO_USB2_0` register.
    pub GIZMO_USB2: Mmio<u32>,
    _unk5: [Mmio<u32>; 0x6],
    /// The `AHB_GIZMO_ARC_0` register.
    pub GIZMO_ARC: Mmio<u32>,
    _unk6: [Mmio<u32>; 0xA],
    /// The `AHB_AHB_WRQ_EMPTY_0` register.
    pub AHB_WRQ_EMPTY: Mmio<u32>,
    _unk7: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG5_0` register.
    pub AHB_MEM_PREFETCH_CFG5: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG6_0` register.
    pub AHB_MEM_PREFETCH_CFG6: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG7_0` register.
    pub AHB_MEM_PREFETCH_CFG7: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG8_0` register.
    pub AHB_MEM_PREFETCH_CFG8: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG_X_0` register.
    pub AHB_MEM_PREFETCH_CFG_X: Mmio<u32>,
    /// The `AHB_ARBITRATION_XBAR_CTRL_0` register.
    pub ARBITRATION_XBAR_CTRL: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG3_0` register.
    pub AHB_MEM_PREFETCH_CFG3: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG4_0` register.
    pub AHB_MEM_PREFETCH_CFG4: Mmio<u32>,
    /// The `AHB_AVP_PPCS_RD_COH_STATUS_0` register.
    pub AVP_PPCS_RD_COH_STATUS: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG1_0` register.
    pub AHB_MEM_PREFETCH_CFG1: Mmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG2_0` register.
    pub AHB_MEM_PREFETCH_CFG2: Mmio<u32>,
    /// The `AHB_AHBSLVMEM_STATUS_0` register.
    pub AHBSLVMEM_STATUS: Mmio<u32>,
    /// The `AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID_0` register.
    pub ARBITRATION_AHB_MEM_WRQUE_MST_ID: Mmio<u32>,
    /// The `AHB_ARBITRATION_CPU_ABORT_ADDR_0` register.
    pub ARBITRATION_CPU_ABORT_ADDR: Mmio<u32>,
    /// The `AHB_ARBITRATION_CPU_ABORT_INFO_0` register.
    pub ARBITRATION_CPU_ABORT_INFO: Mmio<u32>,
    /// The `AHB_ARBITRATION_COP_ABORT_ADDR_0` register.
    pub ARBITRATION_COP_ABORT_ADDR: Mmio<u32>,
    /// The `AHB_ARBITRATION_COP_ABORT_INFO_0` register.
    pub ARBITRATION_COP_ABORT_INFO: Mmio<u32>,
    /// The `AHB_AHB_SPARE_REG_0` register.
    pub AHB_SPARE_REG: Mmio<u32>,
    /// The `AHB_XBAR_SPARE_REG_0` register.
    pub XBAR_SPARE_REG: Mmio<u32>,
    _unk8: Mmio<u32>,
    /// The `AHB_AVPC_MCCIF_FIFOCTRL_0` register.
    pub AVPC_MCCIF_FIFOCTRL: Mmio<u32>,
    /// The `AHB_TIMEOUT_WCOAL_AVPC_0` register.
    pub TIMEOUT_WCOAL_AVPC: Mmio<u32>,
    /// The `AHB_MPCORE_MCCIF_FIFOCTRL_0` register.
    pub MPCORE_MCCIF_FIFOCTRL: Mmio<u32>,
}

impl VolatileStorage for AhbRegisters {
    unsafe fn make_ptr() -> *const Self {
        AHB_BASE as *const _
    }
}

/// Representation of the Secure Boot registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct SbRegisters {
    /// The `SB_CSR_0` register.
    pub CSR: Mmio<u32>,
    /// The `SB_PIROM_START_0` register.
    pub PIROM_START: Mmio<u32>,
    /// The `SB_PFCFG_0` register.
    pub PFCFG: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_0_0` register.
    pub SECURE_SPAREREG_0: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_1_0` register.
    pub SECURE_SPAREREG_1: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_2_0` register.
    pub SECURE_SPAREREG_2: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_3_0` register.
    pub SECURE_SPAREREG_3: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_4_0` register.
    pub SECURE_SPAREREG_4: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_5_0` register.
    pub SECURE_SPAREREG_5: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_6_0` register.
    pub SECURE_SPAREREG_6: Mmio<u32>,
    /// The `SB_SECURE_SPAREREG_7_0` register.
    pub SECURE_SPAREREG_7: Mmio<u32>,
    _unk: Mmio<u32>,
    /// The `SB_AA64_RESET_LOW_0` register.
    pub AA64_RESET_LOW: Mmio<u32>,
    /// The `SB_AA64_RESET_HIGH_0` register.
    pub AA64_RESET_HIGH: Mmio<u32>,
}

impl VolatileStorage for SbRegisters {
    unsafe fn make_ptr() -> *const Self {
        SB_BASE as *const _
    }
}
