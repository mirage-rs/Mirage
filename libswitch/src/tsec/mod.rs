//! NVIDIA Tegra Security Co-processor driver.
//!
//! # Description
//!
//! The TSEC is a dedicated unit powered by a NVIDIA Falcon
//! microprocessor with crypto extensions.
//!
//! It is configured and initialized during the boot process
//! to be used for key generation. It loads a firmware which
//! is divided into 4 binary blobs which are required to
//! derive the final TSEC key.
//!
//! # Implementation
//!
//! - Important SOR1 registers are exposed as global constants
//! within the crate.
//!
//! - The [`Registers`] struct represents the TSEC registers
//! that are mapped to address `0x54500000`.
//!
//! - The [`Tsec`] struct holds an instance of [`Registers`] and
//! provides further hardware abstractions. It allows for loading
//! and executing Falcon firmware and finally deriving the TSEC
//! key.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::tsec::Tsec;
//!
//! // Global instance of the TSEC.
//! const TSEC: Tsec = Tsec::new();
//!
//! // Include the TSEC firmware blob stored in another source file.
//! include!("falcon_fw.rs");
//!
//! fn main() {
//!     // Load and execute the firmware.
//!     TSEC.load_firmware(FALCON_FIRMWARE);
//!     TSEC.execute_firmware(None);
//!
//!     // Derive the TSEC key.
//!     let key = TSEC.get_key(1, FALCON_FIRMWARE).unwrap();
//! }
//! ```
//!
//! [`Registers`]: struct.Registers.html
//! [`Registers::get`]: struct.Registers.html#method.get
//! [`Tsec`]: struct.Tsec.html

use mirage_mmio::{BlockMmio, Mmio, VolatileStorage};

use crate::{clock::Clock, timer::get_milliseconds};

/// Base address for the TSEC registers.
pub(crate) const TSEC_BASE: u32 = 0x5450_0000;

/// Base address for SOR1 registers.
pub(crate) const SOR1_BASE: u32 = 0x5458_0000;

/// Base address for HOST1X registers.
pub(crate) const HOST1X_BASE: u32 = 0x5000_0000;

pub(crate) const SOR1_DP_HDCP_BKSV_LSB: Mmio<u32> =
    unsafe { Mmio::new((SOR1_BASE + 0x1E8) as *const u32) };

pub(crate) const SOR1_TMDS_HDCP_BKSV_LSB: Mmio<u32> =
    unsafe { Mmio::new((SOR1_BASE + 0x21C) as *const u32) };

pub(crate) const SOR1_TMDS_HDCP_CN_MSB: Mmio<u32> =
    unsafe { Mmio::new((SOR1_BASE + 0x208) as *const u32) };

pub(crate) const SOR1_TMDS_HDCP_CN_LSB: Mmio<u32> =
    unsafe { Mmio::new((SOR1_BASE + 0x20C) as *const u32) };

/// Representation of the TSEC registers.
#[repr(C)]
pub struct Registers {
    pub tsec_thi_incr_syncpt: BlockMmio<u32>,       // 0x0
    pub tsec_thi_incr_syncpt_ctrl: BlockMmio<u32>,  // 0x4
    pub tsec_thi_incr_syncpt_err: BlockMmio<u32>,   // 0x8
    pub tsec_thi_ctxsw_incr_syncpt: BlockMmio<u32>, // 0xc
    _reserved4: [BlockMmio<u8>; 0x10],
    pub tsec_thi_ctxsw: BlockMmio<u32>,           // 0x20
    pub tsec_thi_ctxsw_next: BlockMmio<u32>,      // 0x24
    pub tsec_thi_cont_syncpt_eof: BlockMmio<u32>, // 0x28
    pub tsec_thi_cont_syncpt_l1: BlockMmio<u32>,  // 0x2c
    pub tsec_thi_streamid0: BlockMmio<u32>,       // 0x30
    pub tsec_thi_streamid1: BlockMmio<u32>,       // 0x34
    pub tsec_thi_thi_sec: BlockMmio<u32>,         // 0x38
    _reserved11: [BlockMmio<u8>; 0x4],
    pub tsec_thi_method0: BlockMmio<u32>, // 0x40
    pub tsec_thi_method1: BlockMmio<u32>, // 0x44
    _reserved13: [BlockMmio<u8>; 0x18],
    pub tsec_thi_context_switch: BlockMmio<u32>, // 0x60
    _reserved14: [BlockMmio<u8>; 0x14],
    pub tsec_thi_int_status: BlockMmio<u32>,           // 0x78
    pub tsec_thi_int_mask: BlockMmio<u32>,             // 0x7c
    pub tsec_thi_config0: BlockMmio<u32>,              // 0x80
    pub tsec_thi_dbg_misc: BlockMmio<u32>,             // 0x84
    pub tsec_thi_slcg_override_high_a: BlockMmio<u32>, // 0x88
    pub tsec_thi_slcg_override_low_a: BlockMmio<u32>,  // 0x8c
    _reserved20: [BlockMmio<u8>; 0xD70],
    pub tsec_thi_clk_override: BlockMmio<u32>, // 0xe00
    _reserved21: [BlockMmio<u8>; 0x1FC],
    pub falcon_irqsset: BlockMmio<u32>,   // 0x1000
    pub falcon_irqsclr: BlockMmio<u32>,   // 0x1004
    pub falcon_irqstat: BlockMmio<u32>,   // 0x1008
    pub falcon_irqmode: BlockMmio<u32>,   // 0x100c
    pub falcon_irqmset: BlockMmio<u32>,   // 0x1010
    pub falcon_irqmclr: BlockMmio<u32>,   // 0x1014
    pub falcon_irqmask: BlockMmio<u32>,   // 0x1018
    pub falcon_irqdest: BlockMmio<u32>,   // 0x101c
    pub falcon_gptmrint: BlockMmio<u32>,  // 0x1020
    pub falcon_gptmrval: BlockMmio<u32>,  // 0x1024
    pub falcon_gptmrctl: BlockMmio<u32>,  // 0x1028
    pub falcon_ptimer0: BlockMmio<u32>,   // 0x102c
    pub falcon_ptimer1: BlockMmio<u32>,   // 0x1030
    pub falcon_wdtmrval: BlockMmio<u32>,  // 0x1034
    pub falcon_wdtmrctl: BlockMmio<u32>,  // 0x1038
    pub falcon_irqdest2: BlockMmio<u32>,  // 0x103c
    pub falcon_mailbox0: BlockMmio<u32>,  // 0x1040
    pub falcon_mailbox1: BlockMmio<u32>,  // 0x1044
    pub falcon_itfen: BlockMmio<u32>,     // 0x1048
    pub falcon_idlestate: BlockMmio<u32>, // 0x104c
    pub falcon_curctx: BlockMmio<u32>,    // 0x1050
    pub falcon_nxtctx: BlockMmio<u32>,    // 0x1054
    pub falcon_ctxack: BlockMmio<u32>,    // 0x1058
    pub falcon_fhstate: BlockMmio<u32>,   // 0x105c
    pub falcon_privstate: BlockMmio<u32>, // 0x1060
    pub falcon_mthddata: BlockMmio<u32>,  // 0x1064
    pub falcon_mthdid: BlockMmio<u32>,    // 0x1068
    pub falcon_mthdwdat: BlockMmio<u32>,  // 0x106c
    pub falcon_mthdcount: BlockMmio<u32>, // 0x1070
    pub falcon_mthdpop: BlockMmio<u32>,   // 0x1074
    pub falcon_mthdramsz: BlockMmio<u32>, // 0x1078
    pub falcon_sftreset: BlockMmio<u32>,  // 0x107c
    pub falcon_os: BlockMmio<u32>,        // 0x1080
    pub falcon_rm: BlockMmio<u32>,        // 0x1084
    pub falcon_soft_pm: BlockMmio<u32>,   // 0x1088
    pub falcon_soft_mode: BlockMmio<u32>, // 0x108c
    pub falcon_debug1: BlockMmio<u32>,    // 0x1090
    pub falcon_debuginfo: BlockMmio<u32>, // 0x1094
    pub falcon_ibrkpt1: BlockMmio<u32>,   // 0x1098
    pub falcon_ibrkpt2: BlockMmio<u32>,   // 0x109c
    pub falcon_cgctl: BlockMmio<u32>,     // 0x10a0
    pub falcon_engctl: BlockMmio<u32>,    // 0x10a4
    pub falcon_pmm: BlockMmio<u32>,       // 0x10a8
    pub falcon_addr: BlockMmio<u32>,      // 0x10ac
    pub falcon_ibrkpt3: BlockMmio<u32>,   // 0x10b0
    pub falcon_ibrkpt4: BlockMmio<u32>,   // 0x10b4
    pub falcon_ibrkpt5: BlockMmio<u32>,   // 0x10b8
    _reserved68: [BlockMmio<u8>; 0x14],
    pub falcon_exci: BlockMmio<u32>,     // 0x10d0
    pub falcon_svec_spr: BlockMmio<u32>, // 0x10d4
    pub falcon_rstat0: BlockMmio<u32>,   // 0x10d8
    pub falcon_rstat3: BlockMmio<u32>,   // 0x10dc
    pub falcon_unk_e0: BlockMmio<u32>,   // 0x10e0
    _reserved73: [BlockMmio<u8>; 0x1C],
    pub falcon_cpuctl: BlockMmio<u32>,       // 0x1100
    pub falcon_bootvec: BlockMmio<u32>,      // 0x1104
    pub falcon_hwcfg: BlockMmio<u32>,        // 0x1108
    pub falcon_dmactl: BlockMmio<u32>,       // 0x110c
    pub falcon_dmatrfbase: BlockMmio<u32>,   // 0x1110
    pub falcon_dmatrfmoffs: BlockMmio<u32>,  // 0x1114
    pub falcon_dmatrfcmd: BlockMmio<u32>,    // 0x1118
    pub falcon_dmatrffboffs: BlockMmio<u32>, // 0x111c
    pub falcon_dmapoll_fb: BlockMmio<u32>,   // 0x1120
    pub falcon_dmapoll_cp: BlockMmio<u32>,   // 0x1124
    pub falcon_dbg_state: BlockMmio<u32>,    // 0x1128
    pub falcon_hwcfg1: BlockMmio<u32>,       // 0x112c
    pub falcon_cpuctl_alias: BlockMmio<u32>, // 0x1130
    _reserved86: [BlockMmio<u8>; 0x4],
    pub falcon_stackcfg: BlockMmio<u32>, // 0x1138
    _reserved87: [BlockMmio<u8>; 0x4],
    pub falcon_imctl: BlockMmio<u32>,       // 0x1140
    pub falcon_imstat: BlockMmio<u32>,      // 0x1144
    pub falcon_traceidx: BlockMmio<u32>,    // 0x1148
    pub falcon_tracepc: BlockMmio<u32>,     // 0x114c
    pub falcon_imfillrng0: BlockMmio<u32>,  // 0x1150
    pub falcon_imfillrng1: BlockMmio<u32>,  // 0x1154
    pub falcon_imfillctl: BlockMmio<u32>,   // 0x1158
    pub falcon_imctl_debug: BlockMmio<u32>, // 0x115c
    pub falcon_cmembase: BlockMmio<u32>,    // 0x1160
    pub falcon_dmemapert: BlockMmio<u32>,   // 0x1164
    pub falcon_exterraddr: BlockMmio<u32>,  // 0x1168
    pub falcon_exterrstat: BlockMmio<u32>,  // 0x116c
    _reserved99: [BlockMmio<u8>; 0xC],
    pub falcon_cg2: BlockMmio<u32>,    // 0x117c
    pub falcon_imemc0: BlockMmio<u32>, // 0x1180
    pub falcon_imemd0: BlockMmio<u32>, // 0x1184
    pub falcon_imemt0: BlockMmio<u32>, // 0x1188
    _reserved103: [BlockMmio<u8>; 0x4],
    pub falcon_imemc1: BlockMmio<u32>, // 0x1190
    pub falcon_imemd1: BlockMmio<u32>, // 0x1194
    pub falcon_imemt1: BlockMmio<u32>, // 0x1198
    _reserved106: [BlockMmio<u8>; 0x4],
    pub falcon_imemc2: BlockMmio<u32>, // 0x11a0
    pub falcon_imemd2: BlockMmio<u32>, // 0x11a4
    pub falcon_imemt2: BlockMmio<u32>, // 0x11a8
    _reserved109: [BlockMmio<u8>; 0x4],
    pub falcon_imemc3: BlockMmio<u32>, // 0x11b0
    pub falcon_imemd3: BlockMmio<u32>, // 0x11b4
    pub falcon_imemt3: BlockMmio<u32>, // 0x11b8
    _reserved112: [BlockMmio<u8>; 0x4],
    pub falcon_dmemc0: BlockMmio<u32>,    // 0x11c0
    pub falcon_dmemd0: BlockMmio<u32>,    // 0x11c4
    pub falcon_dmemc1: BlockMmio<u32>,    // 0x11c8
    pub falcon_dmemd1: BlockMmio<u32>,    // 0x11cc
    pub falcon_dmemc2: BlockMmio<u32>,    // 0x11d0
    pub falcon_dmemd2: BlockMmio<u32>,    // 0x11d4
    pub falcon_dmemc3: BlockMmio<u32>,    // 0x11d8
    pub falcon_dmemd3: BlockMmio<u32>,    // 0x11dc
    pub falcon_dmemc4: BlockMmio<u32>,    // 0x11e0
    pub falcon_dmemd4: BlockMmio<u32>,    // 0x11e4
    pub falcon_dmemc5: BlockMmio<u32>,    // 0x11e8
    pub falcon_dmemd5: BlockMmio<u32>,    // 0x11ec
    pub falcon_dmemc6: BlockMmio<u32>,    // 0x11f0
    pub falcon_dmemd6: BlockMmio<u32>,    // 0x11f4
    pub falcon_dmemc7: BlockMmio<u32>,    // 0x11f8
    pub falcon_dmemd7: BlockMmio<u32>,    // 0x11fc
    pub falcon_icd_cmd: BlockMmio<u32>,   // 0x1200
    pub falcon_icd_addr: BlockMmio<u32>,  // 0x1204
    pub falcon_icd_wdata: BlockMmio<u32>, // 0x1208
    pub falcon_icd_rdata: BlockMmio<u32>, // 0x120c
    _reserved132: [BlockMmio<u8>; 0x30],
    pub falcon_sctl: BlockMmio<u32>,    // 0x1240
    pub falcon_sstat: BlockMmio<u32>,   // 0x1244
    pub falcon_unk_248: BlockMmio<u32>, // 0x1248
    pub falcon_unk_24c: BlockMmio<u32>, // 0x124c
    pub falcon_unk_250: BlockMmio<u32>, // 0x1250
    _reserved137: [BlockMmio<u8>; 0xC],
    pub falcon_unk_260: BlockMmio<u32>, // 0x1260
    _reserved138: [BlockMmio<u8>; 0x1C],
    pub falcon_sprot_imem: BlockMmio<u32>,   // 0x1280
    pub falcon_sprot_dmem: BlockMmio<u32>,   // 0x1284
    pub falcon_sprot_cpuctl: BlockMmio<u32>, // 0x1288
    pub falcon_sprot_misc: BlockMmio<u32>,   // 0x128c
    pub falcon_sprot_irq: BlockMmio<u32>,    // 0x1290
    pub falcon_sprot_mthd: BlockMmio<u32>,   // 0x1294
    pub falcon_sprot_sctl: BlockMmio<u32>,   // 0x1298
    pub falcon_sprot_wdtmr: BlockMmio<u32>,  // 0x129c
    _reserved146: [BlockMmio<u8>; 0x20],
    pub falcon_dmainfo_finished_fbrd_low: BlockMmio<u32>, // 0x12c0
    pub falcon_dmainfo_finished_fbrd_high: BlockMmio<u32>, // 0x12c4
    pub falcon_dmainfo_finished_fbwr_low: BlockMmio<u32>, // 0x12c8
    pub falcon_dmainfo_finished_fbwr_high: BlockMmio<u32>, // 0x12cc
    pub falcon_dmainfo_current_fbrd_low: BlockMmio<u32>,  // 0x12d0
    pub falcon_dmainfo_current_fbrd_high: BlockMmio<u32>, // 0x12d4
    pub falcon_dmainfo_current_fbwr_low: BlockMmio<u32>,  // 0x12d8
    pub falcon_dmainfo_current_fbwr_high: BlockMmio<u32>, // 0x12dc
    pub falcon_dmainfo_ctl: BlockMmio<u32>,               // 0x12e0
    _reserved155: [BlockMmio<u8>; 0x11C],
    pub tsec_scp_ctl0: BlockMmio<u32>,     // 0x1400
    pub tsec_scp_ctl1: BlockMmio<u32>,     // 0x1404
    pub tsec_scp_ctl_stat: BlockMmio<u32>, // 0x1408
    pub tsec_scp_ctl_lock: BlockMmio<u32>, // 0x140c
    pub tsec_scp_unk_10: BlockMmio<u32>,   // 0x1410
    pub tsec_scp_unk_14: BlockMmio<u32>,   // 0x1414
    pub tsec_scp_ctl_pkey: BlockMmio<u32>, // 0x1418
    pub tsec_scp_unk_1c: BlockMmio<u32>,   // 0x141c
    pub tsec_scp_seq_ctl: BlockMmio<u32>,  // 0x1420
    pub tsec_scp_seq_val: BlockMmio<u32>,  // 0x1424
    pub tsec_scp_seq_stat: BlockMmio<u32>, // 0x1428
    _reserved166: [BlockMmio<u8>; 0x4],
    pub tsec_scp_insn_stat: BlockMmio<u32>, // 0x1430
    _reserved167: [BlockMmio<u8>; 0x1c],
    pub tsec_scp_unk_50: BlockMmio<u32>,    // 0x1450
    pub tsec_scp_auth_stat: BlockMmio<u32>, // 0x1454
    pub tsec_scp_aes_stat: BlockMmio<u32>,  // 0x1458
    _reserved170: [BlockMmio<u8>; 0x14],
    pub tsec_scp_unk_70: BlockMmio<u32>, // 0x1470
    _reserved171: [BlockMmio<u8>; 0xC],
    pub tsec_scp_irqstat: BlockMmio<u32>, // 0x1480
    pub tsec_scp_irqmask: BlockMmio<u32>, // 0x1484
    _reserved173: [BlockMmio<u8>; 0x8],
    pub tsec_scp_acl_err: BlockMmio<u32>,  // 0x1490
    pub tsec_scp_unk_94: BlockMmio<u32>,   // 0x1494
    pub tsec_scp_insn_err: BlockMmio<u32>, // 0x1498
    _reserved176: [BlockMmio<u8>; 0x64],
    pub tsec_trng_clk_limit_low: BlockMmio<u32>, // 0x1500
    pub tsec_trng_clk_limit_high: BlockMmio<u32>, // 0x1504
    pub tsec_trng_unk_08: BlockMmio<u32>,        // 0x1508
    pub tsec_trng_test_ctl: BlockMmio<u32>,      // 0x150c
    pub tsec_trng_test_cfg0: BlockMmio<u32>,     // 0x1510
    pub tsec_trng_test_seed0: BlockMmio<u32>,    // 0x1514
    pub tsec_trng_test_cfg1: BlockMmio<u32>,     // 0x1518
    pub tsec_trng_test_seed1: BlockMmio<u32>,    // 0x151c
    pub tsec_trng_unk_20: BlockMmio<u32>,        // 0x1520
    pub tsec_trng_unk_24: BlockMmio<u32>,        // 0x1524
    pub tsec_trng_unk_28: BlockMmio<u32>,        // 0x1528
    pub tsec_trng_ctl: BlockMmio<u32>,           // 0x152c
    _reserved188: [BlockMmio<u8>; 0xD0],
    pub tsec_tfbif_ctl: BlockMmio<u32>,             // 0x1600
    pub tsec_tfbif_mccif_fifoctrl: BlockMmio<u32>,  // 0x1604
    pub tsec_tfbif_throttle: BlockMmio<u32>,        // 0x1608
    pub tsec_tfbif_dbg_stat0: BlockMmio<u32>,       // 0x160c
    pub tsec_tfbif_dbg_stat1: BlockMmio<u32>,       // 0x1610
    pub tsec_tfbif_dbg_rdcount_lo: BlockMmio<u32>,  // 0x1614
    pub tsec_tfbif_dbg_rdcount_hi: BlockMmio<u32>,  // 0x1618
    pub tsec_tfbif_dbg_wrcount_lo: BlockMmio<u32>,  // 0x161c
    pub tsec_tfbif_dbg_wrcount_hi: BlockMmio<u32>,  // 0x1620
    pub tsec_tfbif_dbg_r32count: BlockMmio<u32>,    // 0x1624
    pub tsec_tfbif_dbg_r64count: BlockMmio<u32>,    // 0x1628
    pub tsec_tfbif_dbg_r128count: BlockMmio<u32>,   // 0x162c
    pub tsec_tfbif_unk_30: BlockMmio<u32>,          // 0x1630
    pub tsec_tfbif_mccif_fifoctrl1: BlockMmio<u32>, // 0x1634
    pub tsec_tfbif_wrr_rdp: BlockMmio<u32>,         // 0x1638
    _reserved203: [BlockMmio<u8>; 0x4],
    pub tsec_tfbif_sprot_emem: BlockMmio<u32>, // 0x1640
    pub tsec_tfbif_transcfg: BlockMmio<u32>,   // 0x1644
    pub tsec_tfbif_regioncfg: BlockMmio<u32>,  // 0x1648
    pub tsec_tfbif_actmon_active_mask: BlockMmio<u32>, // 0x164c
    pub tsec_tfbif_actmon_active_borps: BlockMmio<u32>, // 0x1650
    pub tsec_tfbif_actmon_active_weight: BlockMmio<u32>, // 0x1654
    _reserved209: [BlockMmio<u8>; 0x8],
    pub tsec_tfbif_actmon_mcb_mask: BlockMmio<u32>, // 0x1660
    pub tsec_tfbif_actmon_mcb_borps: BlockMmio<u32>, // 0x1664
    pub tsec_tfbif_actmon_mcb_weight: BlockMmio<u32>, // 0x1668
    _reserved212: [BlockMmio<u8>; 0x4],
    pub tsec_tfbif_thi_transprop: BlockMmio<u32>, // 0x1670
    _reserved213: [BlockMmio<u8>; 0x5C],
    pub tsec_cg: BlockMmio<u32>, // 0x16d0
    _reserved214: [BlockMmio<u8>; 0x2C],
    pub tsec_bar0_ctl: BlockMmio<u32>,     // 0x1700
    pub tsec_bar0_addr: BlockMmio<u32>,    // 0x1704
    pub tsec_bar0_data: BlockMmio<u32>,    // 0x1708
    pub tsec_bar0_timeout: BlockMmio<u32>, // 0x170c
    _reserved218: [BlockMmio<u8>; 0xF0],
    pub tsec_tegra_falcon_ip_ver: BlockMmio<u32>, // 0x1800
    pub tsec_tegra_unk_04: BlockMmio<u32>,        // 0x1804
    pub tsec_tegra_unk_08: BlockMmio<u32>,        // 0x1808
    pub tsec_tegra_unk_0c: BlockMmio<u32>,        // 0x180c
    pub tsec_tegra_unk_10: BlockMmio<u32>,        // 0x1810
    pub tsec_tegra_unk_14: BlockMmio<u32>,        // 0x1814
    pub tsec_tegra_unk_18: BlockMmio<u32>,        // 0x1818
    pub tsec_tegra_unk_1c: BlockMmio<u32>,        // 0x181c
    pub tsec_tegra_unk_20: BlockMmio<u32>,        // 0x1820
    pub tsec_tegra_unk_24: BlockMmio<u32>,        // 0x1824
    pub tsec_tegra_unk_28: BlockMmio<u32>,        // 0x1828
    pub tsec_tegra_unk_2c: BlockMmio<u32>,        // 0x182c
    pub tsec_tegra_unk_30: BlockMmio<u32>,        // 0x1830
    pub tsec_tegra_unk_34: BlockMmio<u32>,        // 0x1834
    pub tsec_tegra_ctl: BlockMmio<u32>,           // 0x1838
}

impl VolatileStorage for Registers {
    unsafe fn make_ptr() -> *const Self {
        TSEC_BASE as *const _
    }
}

/// Representation of the TSEC.
pub struct Tsec;

impl Tsec {
    /// Waits until DMA has entered an idle state.
    fn dma_wait_idle(&self) -> Result<(), ()> {
        let registers = unsafe { Registers::get() };

        let timeout = get_milliseconds() + 10000;

        while (registers.falcon_dmatrfcmd.read() & (1 << 1)) == 0 {
            if get_milliseconds() > timeout {
                return Err(());
            }
        }

        Ok(())
    }

    /// Configures physical DMA transfers to Falcon.
    fn dma_phys_to_flcn(
        &self,
        is_imem: bool,
        flcn_offset: u32,
        phys_offset: u32,
    ) -> Result<(), ()> {
        let registers = unsafe { Registers::get() };

        let cmd = if is_imem { 0x10 } else { 0x600 };

        registers.falcon_dmatrfmoffs.write(flcn_offset);
        registers.falcon_dmatrffboffs.write(phys_offset);
        registers.falcon_dmatrfcmd.write(cmd);

        self.dma_wait_idle()
    }

    /// Creates a new TSEC object.
    pub const fn new() -> Self {
        Tsec
    }

    /// Enables all devices used by TSEC.
    pub fn enable_clocks(&self) {
        Clock::HOST1X.enable();
        Clock::TSEC.enable();
        Clock::SOR_SAFE.enable();
        Clock::SOR0.enable();
        Clock::SOR1.enable();
        Clock::KFUSE.enable();
    }

    /// Disables all devices used by TSEC.
    pub fn disable_clocks(&self) {
        Clock::HOST1X.disable();
        Clock::TSEC.disable();
        Clock::SOR_SAFE.disable();
        Clock::SOR0.disable();
        Clock::SOR1.disable();
        Clock::KFUSE.disable();
    }

    /// Retrieves the TSEC key.
    pub fn get_key(&self, rev: u32, firmware: &mut [u8]) -> Result<[u32; 4], ()> {
        let registers = unsafe { Registers::get() };

        self.enable_clocks();

        // Configure Falcon.
        registers.falcon_dmactl.write(0);
        registers.falcon_irqmset.write(0xFFF2);
        registers.falcon_irqdest.write(0xFFF0);
        registers.falcon_itfen.write(3);

        if self.dma_wait_idle().is_err() {
            self.disable_clocks();
            return Err(());
        }

        // Load firmware.
        if self.load_firmware(firmware).is_err() {
            self.disable_clocks();
            return Err(());
        }

        // Execute firmware.
        self.execute_firmware(Some(rev));

        if self.dma_wait_idle().is_err() {
            self.disable_clocks();
            return Err(());
        }

        let timeout = get_milliseconds() + 2000;
        while registers.falcon_mailbox1.read() == 0 {
            if get_milliseconds() > timeout {
                self.disable_clocks();
                return Err(());
            }
        }

        if registers.falcon_mailbox1.read() != 0xB0B0_B0B0 {
            self.disable_clocks();
            return Err(());
        }

        // Unknown HOST1X write.
        unsafe {
            Mmio::new((HOST1X_BASE + 0x3300) as *const u32).write(0);
        }

        // Fetch result from SOR1.
        let mut key: [u32; 0x4] = [0; 4];
        key[0] = SOR1_DP_HDCP_BKSV_LSB.read();
        key[1] = SOR1_TMDS_HDCP_BKSV_LSB.read();
        key[2] = SOR1_TMDS_HDCP_CN_MSB.read();
        key[3] = SOR1_TMDS_HDCP_CN_LSB.read();

        // Clear SOR1 registers.
        SOR1_DP_HDCP_BKSV_LSB.write(0);
        SOR1_TMDS_HDCP_BKSV_LSB.write(0);
        SOR1_TMDS_HDCP_CN_MSB.write(0);
        SOR1_TMDS_HDCP_CN_LSB.write(0);

        Ok(key)
    }

    /// Loads the TSEC firmware.
    pub fn load_firmware(&self, firmware: &[u8]) -> Result<(), ()> {
        let registers = unsafe { Registers::get() };

        let mut res = Ok(());

        // Configure Falcon.
        registers.falcon_dmactl.write(0);
        registers.falcon_irqmset.write(0xFFF2);
        registers.falcon_irqdest.write(0xFFF0);
        registers.falcon_itfen.write(3);

        if self.dma_wait_idle().is_ok() {
            // Load firmware.
            registers.falcon_dmatrfbase.write(firmware.as_ptr() as usize as u32 >> 8);

            let mut addr = 0;
            while addr < firmware.len() {
                if self
                    .dma_phys_to_flcn(true, addr as u32, addr as u32)
                    .is_err()
                {
                    res = Err(());
                    break;
                }

                addr += 0x100;
            }
        } else {
            res = Err(());
        }

        res
    }

    /// Executes the loaded TSEC firmware.
    pub fn execute_firmware(&self, rev: Option<u32>) {
        let registers = unsafe { Registers::get() };

        // Unknown HOST1X write.
        unsafe {
            Mmio::new((HOST1X_BASE + 0x3300) as *const u32).write(0x34C2_E1DA);
        }

        // Execute the firmware.
        registers.falcon_mailbox1.write(0);
        registers.falcon_mailbox0.write(rev.unwrap_or(0));
        registers.falcon_bootvec.write(0);
        registers.falcon_cpuctl.write(2);
    }
}
