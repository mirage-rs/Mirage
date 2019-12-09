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
//! const TSEC: &Tsec = Tsec::new();
//!
//! // Include the TSEC firmware blob stored in another source file.
//! include!("falcon_fw.rs");
//!
//! fn main() {
//!     // Load and execute the firmware.
//!     TSEC.load_firmware(FALCON_FIRMWARE);
//!     TSEC.execute_firmware();
//!
//!     // Derive the TSEC key.
//!     let key = TSEC.get_key(1, FALCON_FIRMWARE).unwrap();
//! }
//! ```
//!
//! [`Registers`]: struct.Registers.html
//! [`Registers::get`]: struct.Registers.html#method.get
//! [`Tsec`]: struct.Tsec.html

use mirage_mmio::{Mmio, VolatileStorage};

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
    pub tsec_thi_incr_syncpt: Mmio<u32>,       // 0x0
    pub tsec_thi_incr_syncpt_ctrl: Mmio<u32>,  // 0x4
    pub tsec_thi_incr_syncpt_err: Mmio<u32>,   // 0x8
    pub tsec_thi_ctxsw_incr_syncpt: Mmio<u32>, // 0xc
    _reserved4: [u8; 0x10],
    pub tsec_thi_ctxsw: Mmio<u32>,           // 0x20
    pub tsec_thi_ctxsw_next: Mmio<u32>,      // 0x24
    pub tsec_thi_cont_syncpt_eof: Mmio<u32>, // 0x28
    pub tsec_thi_cont_syncpt_l1: Mmio<u32>,  // 0x2c
    pub tsec_thi_streamid0: Mmio<u32>,       // 0x30
    pub tsec_thi_streamid1: Mmio<u32>,       // 0x34
    pub tsec_thi_thi_sec: Mmio<u32>,         // 0x38
    _reserved11: [u8; 0x4],
    pub tsec_thi_method0: Mmio<u32>, // 0x40
    pub tsec_thi_method1: Mmio<u32>, // 0x44
    _reserved13: [u8; 0x18],
    pub tsec_thi_context_switch: Mmio<u32>, // 0x60
    _reserved14: [u8; 0x14],
    pub tsec_thi_int_status: Mmio<u32>,           // 0x78
    pub tsec_thi_int_mask: Mmio<u32>,             // 0x7c
    pub tsec_thi_config0: Mmio<u32>,              // 0x80
    pub tsec_thi_dbg_misc: Mmio<u32>,             // 0x84
    pub tsec_thi_slcg_override_high_a: Mmio<u32>, // 0x88
    pub tsec_thi_slcg_override_low_a: Mmio<u32>,  // 0x8c
    _reserved20: [u8; 0xd70],
    pub tsec_thi_clk_override: Mmio<u32>, // 0xe00
    _reserved21: [u8; 0x1fc],
    pub falcon_irqsset: Mmio<u32>,   // 0x1000
    pub falcon_irqsclr: Mmio<u32>,   // 0x1004
    pub falcon_irqstat: Mmio<u32>,   // 0x1008
    pub falcon_irqmode: Mmio<u32>,   // 0x100c
    pub falcon_irqmset: Mmio<u32>,   // 0x1010
    pub falcon_irqmclr: Mmio<u32>,   // 0x1014
    pub falcon_irqmask: Mmio<u32>,   // 0x1018
    pub falcon_irqdest: Mmio<u32>,   // 0x101c
    pub falcon_gptmrint: Mmio<u32>,  // 0x1020
    pub falcon_gptmrval: Mmio<u32>,  // 0x1024
    pub falcon_gptmrctl: Mmio<u32>,  // 0x1028
    pub falcon_ptimer0: Mmio<u32>,   // 0x102c
    pub falcon_ptimer1: Mmio<u32>,   // 0x1030
    pub falcon_wdtmrval: Mmio<u32>,  // 0x1034
    pub falcon_wdtmrctl: Mmio<u32>,  // 0x1038
    pub falcon_irqdest2: Mmio<u32>,  // 0x103c
    pub falcon_mailbox0: Mmio<u32>,  // 0x1040
    pub falcon_mailbox1: Mmio<u32>,  // 0x1044
    pub falcon_itfen: Mmio<u32>,     // 0x1048
    pub falcon_idlestate: Mmio<u32>, // 0x104c
    pub falcon_curctx: Mmio<u32>,    // 0x1050
    pub falcon_nxtctx: Mmio<u32>,    // 0x1054
    pub falcon_ctxack: Mmio<u32>,    // 0x1058
    pub falcon_fhstate: Mmio<u32>,   // 0x105c
    pub falcon_privstate: Mmio<u32>, // 0x1060
    pub falcon_mthddata: Mmio<u32>,  // 0x1064
    pub falcon_mthdid: Mmio<u32>,    // 0x1068
    pub falcon_mthdwdat: Mmio<u32>,  // 0x106c
    pub falcon_mthdcount: Mmio<u32>, // 0x1070
    pub falcon_mthdpop: Mmio<u32>,   // 0x1074
    pub falcon_mthdramsz: Mmio<u32>, // 0x1078
    pub falcon_sftreset: Mmio<u32>,  // 0x107c
    pub falcon_os: Mmio<u32>,        // 0x1080
    pub falcon_rm: Mmio<u32>,        // 0x1084
    pub falcon_soft_pm: Mmio<u32>,   // 0x1088
    pub falcon_soft_mode: Mmio<u32>, // 0x108c
    pub falcon_debug1: Mmio<u32>,    // 0x1090
    pub falcon_debuginfo: Mmio<u32>, // 0x1094
    pub falcon_ibrkpt1: Mmio<u32>,   // 0x1098
    pub falcon_ibrkpt2: Mmio<u32>,   // 0x109c
    pub falcon_cgctl: Mmio<u32>,     // 0x10a0
    pub falcon_engctl: Mmio<u32>,    // 0x10a4
    pub falcon_pmm: Mmio<u32>,       // 0x10a8
    pub falcon_addr: Mmio<u32>,      // 0x10ac
    pub falcon_ibrkpt3: Mmio<u32>,   // 0x10b0
    pub falcon_ibrkpt4: Mmio<u32>,   // 0x10b4
    pub falcon_ibrkpt5: Mmio<u32>,   // 0x10b8
    _reserved68: [u8; 0x14],
    pub falcon_exci: Mmio<u32>,     // 0x10d0
    pub falcon_svec_spr: Mmio<u32>, // 0x10d4
    pub falcon_rstat0: Mmio<u32>,   // 0x10d8
    pub falcon_rstat3: Mmio<u32>,   // 0x10dc
    pub falcon_unk_e0: Mmio<u32>,   // 0x10e0
    _reserved73: [u8; 0x1c],
    pub falcon_cpuctl: Mmio<u32>,       // 0x1100
    pub falcon_bootvec: Mmio<u32>,      // 0x1104
    pub falcon_hwcfg: Mmio<u32>,        // 0x1108
    pub falcon_dmactl: Mmio<u32>,       // 0x110c
    pub falcon_dmatrfbase: Mmio<u32>,   // 0x1110
    pub falcon_dmatrfmoffs: Mmio<u32>,  // 0x1114
    pub falcon_dmatrfcmd: Mmio<u32>,    // 0x1118
    pub falcon_dmatrffboffs: Mmio<u32>, // 0x111c
    pub falcon_dmapoll_fb: Mmio<u32>,   // 0x1120
    pub falcon_dmapoll_cp: Mmio<u32>,   // 0x1124
    pub falcon_dbg_state: Mmio<u32>,    // 0x1128
    pub falcon_hwcfg1: Mmio<u32>,       // 0x112c
    pub falcon_cpuctl_alias: Mmio<u32>, // 0x1130
    _reserved86: [u8; 0x4],
    pub falcon_stackcfg: Mmio<u32>, // 0x1138
    _reserved87: [u8; 0x4],
    pub falcon_imctl: Mmio<u32>,       // 0x1140
    pub falcon_imstat: Mmio<u32>,      // 0x1144
    pub falcon_traceidx: Mmio<u32>,    // 0x1148
    pub falcon_tracepc: Mmio<u32>,     // 0x114c
    pub falcon_imfillrng0: Mmio<u32>,  // 0x1150
    pub falcon_imfillrng1: Mmio<u32>,  // 0x1154
    pub falcon_imfillctl: Mmio<u32>,   // 0x1158
    pub falcon_imctl_debug: Mmio<u32>, // 0x115c
    pub falcon_cmembase: Mmio<u32>,    // 0x1160
    pub falcon_dmemapert: Mmio<u32>,   // 0x1164
    pub falcon_exterraddr: Mmio<u32>,  // 0x1168
    pub falcon_exterrstat: Mmio<u32>,  // 0x116c
    _reserved99: [u8; 0xc],
    pub falcon_cg2: Mmio<u32>,    // 0x117c
    pub falcon_imemc0: Mmio<u32>, // 0x1180
    pub falcon_imemd0: Mmio<u32>, // 0x1184
    pub falcon_imemt0: Mmio<u32>, // 0x1188
    _reserved103: [u8; 0x4],
    pub falcon_imemc1: Mmio<u32>, // 0x1190
    pub falcon_imemd1: Mmio<u32>, // 0x1194
    pub falcon_imemt1: Mmio<u32>, // 0x1198
    _reserved106: [u8; 0x4],
    pub falcon_imemc2: Mmio<u32>, // 0x11a0
    pub falcon_imemd2: Mmio<u32>, // 0x11a4
    pub falcon_imemt2: Mmio<u32>, // 0x11a8
    _reserved109: [u8; 0x4],
    pub falcon_imemc3: Mmio<u32>, // 0x11b0
    pub falcon_imemd3: Mmio<u32>, // 0x11b4
    pub falcon_imemt3: Mmio<u32>, // 0x11b8
    _reserved112: [u8; 0x4],
    pub falcon_dmemc0: Mmio<u32>,    // 0x11c0
    pub falcon_dmemd0: Mmio<u32>,    // 0x11c4
    pub falcon_dmemc1: Mmio<u32>,    // 0x11c8
    pub falcon_dmemd1: Mmio<u32>,    // 0x11cc
    pub falcon_dmemc2: Mmio<u32>,    // 0x11d0
    pub falcon_dmemd2: Mmio<u32>,    // 0x11d4
    pub falcon_dmemc3: Mmio<u32>,    // 0x11d8
    pub falcon_dmemd3: Mmio<u32>,    // 0x11dc
    pub falcon_dmemc4: Mmio<u32>,    // 0x11e0
    pub falcon_dmemd4: Mmio<u32>,    // 0x11e4
    pub falcon_dmemc5: Mmio<u32>,    // 0x11e8
    pub falcon_dmemd5: Mmio<u32>,    // 0x11ec
    pub falcon_dmemc6: Mmio<u32>,    // 0x11f0
    pub falcon_dmemd6: Mmio<u32>,    // 0x11f4
    pub falcon_dmemc7: Mmio<u32>,    // 0x11f8
    pub falcon_dmemd7: Mmio<u32>,    // 0x11fc
    pub falcon_icd_cmd: Mmio<u32>,   // 0x1200
    pub falcon_icd_addr: Mmio<u32>,  // 0x1204
    pub falcon_icd_wdata: Mmio<u32>, // 0x1208
    pub falcon_icd_rdata: Mmio<u32>, // 0x120c
    _reserved132: [u8; 0x30],
    pub falcon_sctl: Mmio<u32>,    // 0x1240
    pub falcon_sstat: Mmio<u32>,   // 0x1244
    pub falcon_unk_248: Mmio<u32>, // 0x1248
    pub falcon_unk_24c: Mmio<u32>, // 0x124c
    pub falcon_unk_250: Mmio<u32>, // 0x1250
    _reserved137: [u8; 0xc],
    pub falcon_unk_260: Mmio<u32>, // 0x1260
    _reserved138: [u8; 0x1c],
    pub falcon_sprot_imem: Mmio<u32>,   // 0x1280
    pub falcon_sprot_dmem: Mmio<u32>,   // 0x1284
    pub falcon_sprot_cpuctl: Mmio<u32>, // 0x1288
    pub falcon_sprot_misc: Mmio<u32>,   // 0x128c
    pub falcon_sprot_irq: Mmio<u32>,    // 0x1290
    pub falcon_sprot_mthd: Mmio<u32>,   // 0x1294
    pub falcon_sprot_sctl: Mmio<u32>,   // 0x1298
    pub falcon_sprot_wdtmr: Mmio<u32>,  // 0x129c
    _reserved146: [u8; 0x20],
    pub falcon_dmainfo_finished_fbrd_low: Mmio<u32>, // 0x12c0
    pub falcon_dmainfo_finished_fbrd_high: Mmio<u32>, // 0x12c4
    pub falcon_dmainfo_finished_fbwr_low: Mmio<u32>, // 0x12c8
    pub falcon_dmainfo_finished_fbwr_high: Mmio<u32>, // 0x12cc
    pub falcon_dmainfo_current_fbrd_low: Mmio<u32>,  // 0x12d0
    pub falcon_dmainfo_current_fbrd_high: Mmio<u32>, // 0x12d4
    pub falcon_dmainfo_current_fbwr_low: Mmio<u32>,  // 0x12d8
    pub falcon_dmainfo_current_fbwr_high: Mmio<u32>, // 0x12dc
    pub falcon_dmainfo_ctl: Mmio<u32>,               // 0x12e0
    _reserved155: [u8; 0x11c],
    pub tsec_scp_ctl0: Mmio<u32>,     // 0x1400
    pub tsec_scp_ctl1: Mmio<u32>,     // 0x1404
    pub tsec_scp_ctl_stat: Mmio<u32>, // 0x1408
    pub tsec_scp_ctl_lock: Mmio<u32>, // 0x140c
    pub tsec_scp_unk_10: Mmio<u32>,   // 0x1410
    pub tsec_scp_unk_14: Mmio<u32>,   // 0x1414
    pub tsec_scp_ctl_pkey: Mmio<u32>, // 0x1418
    pub tsec_scp_unk_1c: Mmio<u32>,   // 0x141c
    pub tsec_scp_seq_ctl: Mmio<u32>,  // 0x1420
    pub tsec_scp_seq_val: Mmio<u32>,  // 0x1424
    pub tsec_scp_seq_stat: Mmio<u32>, // 0x1428
    _reserved166: [u8; 0x4],
    pub tsec_scp_insn_stat: Mmio<u32>, // 0x1430
    _reserved167: [u8; 0x1c],
    pub tsec_scp_unk_50: Mmio<u32>,    // 0x1450
    pub tsec_scp_auth_stat: Mmio<u32>, // 0x1454
    pub tsec_scp_aes_stat: Mmio<u32>,  // 0x1458
    _reserved170: [u8; 0x14],
    pub tsec_scp_unk_70: Mmio<u32>, // 0x1470
    _reserved171: [u8; 0xc],
    pub tsec_scp_irqstat: Mmio<u32>, // 0x1480
    pub tsec_scp_irqmask: Mmio<u32>, // 0x1484
    _reserved173: [u8; 0x8],
    pub tsec_scp_acl_err: Mmio<u32>,  // 0x1490
    pub tsec_scp_unk_94: Mmio<u32>,   // 0x1494
    pub tsec_scp_insn_err: Mmio<u32>, // 0x1498
    _reserved176: [u8; 0x64],
    pub tsec_trng_clk_limit_low: Mmio<u32>, // 0x1500
    pub tsec_trng_clk_limit_high: Mmio<u32>, // 0x1504
    pub tsec_trng_unk_08: Mmio<u32>,        // 0x1508
    pub tsec_trng_test_ctl: Mmio<u32>,      // 0x150c
    pub tsec_trng_test_cfg0: Mmio<u32>,     // 0x1510
    pub tsec_trng_test_seed0: Mmio<u32>,    // 0x1514
    pub tsec_trng_test_cfg1: Mmio<u32>,     // 0x1518
    pub tsec_trng_test_seed1: Mmio<u32>,    // 0x151c
    pub tsec_trng_unk_20: Mmio<u32>,        // 0x1520
    pub tsec_trng_unk_24: Mmio<u32>,        // 0x1524
    pub tsec_trng_unk_28: Mmio<u32>,        // 0x1528
    pub tsec_trng_ctl: Mmio<u32>,           // 0x152c
    _reserved188: [u8; 0xd0],
    pub tsec_tfbif_ctl: Mmio<u32>,             // 0x1600
    pub tsec_tfbif_mccif_fifoctrl: Mmio<u32>,  // 0x1604
    pub tsec_tfbif_throttle: Mmio<u32>,        // 0x1608
    pub tsec_tfbif_dbg_stat0: Mmio<u32>,       // 0x160c
    pub tsec_tfbif_dbg_stat1: Mmio<u32>,       // 0x1610
    pub tsec_tfbif_dbg_rdcount_lo: Mmio<u32>,  // 0x1614
    pub tsec_tfbif_dbg_rdcount_hi: Mmio<u32>,  // 0x1618
    pub tsec_tfbif_dbg_wrcount_lo: Mmio<u32>,  // 0x161c
    pub tsec_tfbif_dbg_wrcount_hi: Mmio<u32>,  // 0x1620
    pub tsec_tfbif_dbg_r32count: Mmio<u32>,    // 0x1624
    pub tsec_tfbif_dbg_r64count: Mmio<u32>,    // 0x1628
    pub tsec_tfbif_dbg_r128count: Mmio<u32>,   // 0x162c
    pub tsec_tfbif_unk_30: Mmio<u32>,          // 0x1630
    pub tsec_tfbif_mccif_fifoctrl1: Mmio<u32>, // 0x1634
    pub tsec_tfbif_wrr_rdp: Mmio<u32>,         // 0x1638
    _reserved203: [u8; 0x4],
    pub tsec_tfbif_sprot_emem: Mmio<u32>, // 0x1640
    pub tsec_tfbif_transcfg: Mmio<u32>,   // 0x1644
    pub tsec_tfbif_regioncfg: Mmio<u32>,  // 0x1648
    pub tsec_tfbif_actmon_active_mask: Mmio<u32>, // 0x164c
    pub tsec_tfbif_actmon_active_borps: Mmio<u32>, // 0x1650
    pub tsec_tfbif_actmon_active_weight: Mmio<u32>, // 0x1654
    _reserved209: [u8; 0x8],
    pub tsec_tfbif_actmon_mcb_mask: Mmio<u32>, // 0x1660
    pub tsec_tfbif_actmon_mcb_borps: Mmio<u32>, // 0x1664
    pub tsec_tfbif_actmon_mcb_weight: Mmio<u32>, // 0x1668
    _reserved212: [u8; 0x4],
    pub tsec_tfbif_thi_transprop: Mmio<u32>, // 0x1670
    _reserved213: [u8; 0x5c],
    pub tsec_cg: Mmio<u32>, // 0x16d0
    _reserved214: [u8; 0x2c],
    pub tsec_bar0_ctl: Mmio<u32>,     // 0x1700
    pub tsec_bar0_addr: Mmio<u32>,    // 0x1704
    pub tsec_bar0_data: Mmio<u32>,    // 0x1708
    pub tsec_bar0_timeout: Mmio<u32>, // 0x170c
    _reserved218: [u8; 0xf0],
    pub tsec_tegra_falcon_ip_ver: Mmio<u32>, // 0x1800
    pub tsec_tegra_unk_04: Mmio<u32>,        // 0x1804
    pub tsec_tegra_unk_08: Mmio<u32>,        // 0x1808
    pub tsec_tegra_unk_0c: Mmio<u32>,        // 0x180c
    pub tsec_tegra_unk_10: Mmio<u32>,        // 0x1810
    pub tsec_tegra_unk_14: Mmio<u32>,        // 0x1814
    pub tsec_tegra_unk_18: Mmio<u32>,        // 0x1818
    pub tsec_tegra_unk_1c: Mmio<u32>,        // 0x181c
    pub tsec_tegra_unk_20: Mmio<u32>,        // 0x1820
    pub tsec_tegra_unk_24: Mmio<u32>,        // 0x1824
    pub tsec_tegra_unk_28: Mmio<u32>,        // 0x1828
    pub tsec_tegra_unk_2c: Mmio<u32>,        // 0x182c
    pub tsec_tegra_unk_30: Mmio<u32>,        // 0x1830
    pub tsec_tegra_unk_34: Mmio<u32>,        // 0x1834
    pub tsec_tegra_ctl: Mmio<u32>,           // 0x1838
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
