//! Tegra210 system registers.

use mirage_mmio::{BlockMmio, VolatileStorage};

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
    pub ARBITRATION_DISABLE: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_PRIORITY_CTRL_0` register.
    pub ARBITRATION_PRIORITY_CTRL: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_USR_PROTECT_0` register.
    pub ARBITRATION_USR_PROTECT: BlockMmio<u32>,
    /// The `AHB_GIZMO_AHB_MEM_0` register.
    pub GIZMO_AHB_MEM: BlockMmio<u32>,
    /// The `AHB_GIZMO_APB_DMA_0` register.
    pub GIZMO_APB_DMA: BlockMmio<u32>,
    /// The `AHB_MASTER_SWID_0` register.
    pub MASTER_SWID_0: BlockMmio<u32>,
    _unk1: BlockMmio<u32>,
    /// The `AHB_GIZMO_USB_0` register.
    pub GIZMO_USB: BlockMmio<u32>,
    /// The `AHB_GIZMO_AHB_XBAR_BRIDGE_0` register.
    pub GIZMO_AHB_XBAR_BRIDGE: BlockMmio<u32>,
    /// The `AHB_GIZMO_CPU_AHB_BRIDGE_0` register.
    pub GIZMO_CPU_AHB_BRIDGE: BlockMmio<u32>,
    /// The `AHB_GIZMO_COP_AHB_BRIDGE_0` register.
    pub GIZMO_COP_AHB_BRIDGE: BlockMmio<u32>,
    /// The `AHB_GIZMO_XBAR_APB_CTLR_0` register.
    pub GIZMO_XBAR_APB_CTRL: BlockMmio<u32>,
    _unk2: BlockMmio<u32>,
    /// The `AHB_MASTER_SWID_1` register.
    pub MASTER_SWID_1: BlockMmio<u32>,
    _unk3: [BlockMmio<u32>; 0x5],
    /// The `AHB_GIZMO_SE_0` register.
    pub GIZMO_SE: BlockMmio<u32>,
    /// The `AHB_GIZMO_TZRAM_0` register.
    pub GIZMO_TZRAM: BlockMmio<u32>,
    _unk4: [BlockMmio<u32>; 0x9],
    /// The `AHB_GIZMO_USB2_0` register.
    pub GIZMO_USB2: BlockMmio<u32>,
    _unk5: [BlockMmio<u32>; 0x6],
    /// The `AHB_GIZMO_ARC_0` register.
    pub GIZMO_ARC: BlockMmio<u32>,
    _unk6: [BlockMmio<u32>; 0xA],
    /// The `AHB_AHB_WRQ_EMPTY_0` register.
    pub AHB_WRQ_EMPTY: BlockMmio<u32>,
    _unk7: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG5_0` register.
    pub AHB_MEM_PREFETCH_CFG5: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG6_0` register.
    pub AHB_MEM_PREFETCH_CFG6: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG7_0` register.
    pub AHB_MEM_PREFETCH_CFG7: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG8_0` register.
    pub AHB_MEM_PREFETCH_CFG8: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG_X_0` register.
    pub AHB_MEM_PREFETCH_CFG_X: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_XBAR_CTRL_0` register.
    pub ARBITRATION_XBAR_CTRL: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG3_0` register.
    pub AHB_MEM_PREFETCH_CFG3: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG4_0` register.
    pub AHB_MEM_PREFETCH_CFG4: BlockMmio<u32>,
    /// The `AHB_AVP_PPCS_RD_COH_STATUS_0` register.
    pub AVP_PPCS_RD_COH_STATUS: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG1_0` register.
    pub AHB_MEM_PREFETCH_CFG1: BlockMmio<u32>,
    /// The `AHB_AHB_MEM_PREFETCH_CFG2_0` register.
    pub AHB_MEM_PREFETCH_CFG2: BlockMmio<u32>,
    /// The `AHB_AHBSLVMEM_STATUS_0` register.
    pub AHBSLVMEM_STATUS: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID_0` register.
    pub ARBITRATION_AHB_MEM_WRQUE_MST_ID: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_CPU_ABORT_ADDR_0` register.
    pub ARBITRATION_CPU_ABORT_ADDR: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_CPU_ABORT_INFO_0` register.
    pub ARBITRATION_CPU_ABORT_INFO: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_COP_ABORT_ADDR_0` register.
    pub ARBITRATION_COP_ABORT_ADDR: BlockMmio<u32>,
    /// The `AHB_ARBITRATION_COP_ABORT_INFO_0` register.
    pub ARBITRATION_COP_ABORT_INFO: BlockMmio<u32>,
    /// The `AHB_AHB_SPARE_REG_0` register.
    pub AHB_SPARE_REG: BlockMmio<u32>,
    /// The `AHB_XBAR_SPARE_REG_0` register.
    pub XBAR_SPARE_REG: BlockMmio<u32>,
    _unk8: BlockMmio<u32>,
    /// The `AHB_AVPC_MCCIF_FIFOCTRL_0` register.
    pub AVPC_MCCIF_FIFOCTRL: BlockMmio<u32>,
    /// The `AHB_TIMEOUT_WCOAL_AVPC_0` register.
    pub TIMEOUT_WCOAL_AVPC: BlockMmio<u32>,
    /// The `AHB_MPCORE_MCCIF_FIFOCTRL_0` register.
    pub MPCORE_MCCIF_FIFOCTRL: BlockMmio<u32>,
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
    pub CSR: BlockMmio<u32>,
    /// The `SB_PIROM_START_0` register.
    pub PIROM_START: BlockMmio<u32>,
    /// The `SB_PFCFG_0` register.
    pub PFCFG: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_0_0` register.
    pub SECURE_SPAREREG_0: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_1_0` register.
    pub SECURE_SPAREREG_1: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_2_0` register.
    pub SECURE_SPAREREG_2: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_3_0` register.
    pub SECURE_SPAREREG_3: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_4_0` register.
    pub SECURE_SPAREREG_4: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_5_0` register.
    pub SECURE_SPAREREG_5: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_6_0` register.
    pub SECURE_SPAREREG_6: BlockMmio<u32>,
    /// The `SB_SECURE_SPAREREG_7_0` register.
    pub SECURE_SPAREREG_7: BlockMmio<u32>,
    _unk: BlockMmio<u32>,
    /// The `SB_AA64_RESET_LOW_0` register.
    pub AA64_RESET_LOW: BlockMmio<u32>,
    /// The `SB_AA64_RESET_HIGH_0` register.
    pub AA64_RESET_HIGH: BlockMmio<u32>,
}

impl VolatileStorage for SbRegisters {
    unsafe fn make_ptr() -> *const Self {
        SB_BASE as *const _
    }
}
