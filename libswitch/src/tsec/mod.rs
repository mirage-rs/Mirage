//! NVIDIA Tegra Security Co-processor driver.

use byteorder::{LittleEndian, ReadBytesExt};
use register::mmio::ReadWrite;

use crate::clock::Clock;
use crate::timer::get_milliseconds;

#[repr(C)]
struct TsecRegisters {
    pub tsec_thi_incr_syncpt: ReadWrite<u32>,       // 0x0
    pub tsec_thi_incr_syncpt_ctrl: ReadWrite<u32>,  // 0x4
    pub tsec_thi_incr_syncpt_err: ReadWrite<u32>,   // 0x8
    pub tsec_thi_ctxsw_incr_syncpt: ReadWrite<u32>, // 0xc
    reserved4: [u8; 0x10],
    pub tsec_thi_ctxsw: ReadWrite<u32>,           // 0x20
    pub tsec_thi_ctxsw_next: ReadWrite<u32>,      // 0x24
    pub tsec_thi_cont_syncpt_eof: ReadWrite<u32>, // 0x28
    pub tsec_thi_cont_syncpt_l1: ReadWrite<u32>,  // 0x2c
    pub tsec_thi_streamid0: ReadWrite<u32>,       // 0x30
    pub tsec_thi_streamid1: ReadWrite<u32>,       // 0x34
    pub tsec_thi_thi_sec: ReadWrite<u32>,         // 0x38
    reserved11: [u8; 0x4],
    pub tsec_thi_method0: ReadWrite<u32>, // 0x40
    pub tsec_thi_method1: ReadWrite<u32>, // 0x44
    reserved13: [u8; 0x18],
    pub tsec_thi_context_switch: ReadWrite<u32>, // 0x60
    reserved14: [u8; 0x14],
    pub tsec_thi_int_status: ReadWrite<u32>,           // 0x78
    pub tsec_thi_int_mask: ReadWrite<u32>,             // 0x7c
    pub tsec_thi_config0: ReadWrite<u32>,              // 0x80
    pub tsec_thi_dbg_misc: ReadWrite<u32>,             // 0x84
    pub tsec_thi_slcg_override_high_a: ReadWrite<u32>, // 0x88
    pub tsec_thi_slcg_override_low_a: ReadWrite<u32>,  // 0x8c
    reserved20: [u8; 0xd70],
    pub tsec_thi_clk_override: ReadWrite<u32>, // 0xe00
    reserved21: [u8; 0x1fc],
    pub falcon_irqsset: ReadWrite<u32>,   // 0x1000
    pub falcon_irqsclr: ReadWrite<u32>,   // 0x1004
    pub falcon_irqstat: ReadWrite<u32>,   // 0x1008
    pub falcon_irqmode: ReadWrite<u32>,   // 0x100c
    pub falcon_irqmset: ReadWrite<u32>,   // 0x1010
    pub falcon_irqmclr: ReadWrite<u32>,   // 0x1014
    pub falcon_irqmask: ReadWrite<u32>,   // 0x1018
    pub falcon_irqdest: ReadWrite<u32>,   // 0x101c
    pub falcon_gptmrint: ReadWrite<u32>,  // 0x1020
    pub falcon_gptmrval: ReadWrite<u32>,  // 0x1024
    pub falcon_gptmrctl: ReadWrite<u32>,  // 0x1028
    pub falcon_ptimer0: ReadWrite<u32>,   // 0x102c
    pub falcon_ptimer1: ReadWrite<u32>,   // 0x1030
    pub falcon_wdtmrval: ReadWrite<u32>,  // 0x1034
    pub falcon_wdtmrctl: ReadWrite<u32>,  // 0x1038
    pub falcon_irqdest2: ReadWrite<u32>,  // 0x103c
    pub falcon_mailbox0: ReadWrite<u32>,  // 0x1040
    pub falcon_mailbox1: ReadWrite<u32>,  // 0x1044
    pub falcon_itfen: ReadWrite<u32>,     // 0x1048
    pub falcon_idlestate: ReadWrite<u32>, // 0x104c
    pub falcon_curctx: ReadWrite<u32>,    // 0x1050
    pub falcon_nxtctx: ReadWrite<u32>,    // 0x1054
    pub falcon_ctxack: ReadWrite<u32>,    // 0x1058
    pub falcon_fhstate: ReadWrite<u32>,   // 0x105c
    pub falcon_privstate: ReadWrite<u32>, // 0x1060
    pub falcon_mthddata: ReadWrite<u32>,  // 0x1064
    pub falcon_mthdid: ReadWrite<u32>,    // 0x1068
    pub falcon_mthdwdat: ReadWrite<u32>,  // 0x106c
    pub falcon_mthdcount: ReadWrite<u32>, // 0x1070
    pub falcon_mthdpop: ReadWrite<u32>,   // 0x1074
    pub falcon_mthdramsz: ReadWrite<u32>, // 0x1078
    pub falcon_sftreset: ReadWrite<u32>,  // 0x107c
    pub falcon_os: ReadWrite<u32>,        // 0x1080
    pub falcon_rm: ReadWrite<u32>,        // 0x1084
    pub falcon_soft_pm: ReadWrite<u32>,   // 0x1088
    pub falcon_soft_mode: ReadWrite<u32>, // 0x108c
    pub falcon_debug1: ReadWrite<u32>,    // 0x1090
    pub falcon_debuginfo: ReadWrite<u32>, // 0x1094
    pub falcon_ibrkpt1: ReadWrite<u32>,   // 0x1098
    pub falcon_ibrkpt2: ReadWrite<u32>,   // 0x109c
    pub falcon_cgctl: ReadWrite<u32>,     // 0x10a0
    pub falcon_engctl: ReadWrite<u32>,    // 0x10a4
    pub falcon_pmm: ReadWrite<u32>,       // 0x10a8
    pub falcon_addr: ReadWrite<u32>,      // 0x10ac
    pub falcon_ibrkpt3: ReadWrite<u32>,   // 0x10b0
    pub falcon_ibrkpt4: ReadWrite<u32>,   // 0x10b4
    pub falcon_ibrkpt5: ReadWrite<u32>,   // 0x10b8
    reserved68: [u8; 0x14],
    pub falcon_exci: ReadWrite<u32>,     // 0x10d0
    pub falcon_svec_spr: ReadWrite<u32>, // 0x10d4
    pub falcon_rstat0: ReadWrite<u32>,   // 0x10d8
    pub falcon_rstat3: ReadWrite<u32>,   // 0x10dc
    pub falcon_unk_e0: ReadWrite<u32>,   // 0x10e0
    reserved73: [u8; 0x1c],
    pub falcon_cpuctl: ReadWrite<u32>,       // 0x1100
    pub falcon_bootvec: ReadWrite<u32>,      // 0x1104
    pub falcon_hwcfg: ReadWrite<u32>,        // 0x1108
    pub falcon_dmactl: ReadWrite<u32>,       // 0x110c
    pub falcon_dmatrfbase: ReadWrite<u32>,   // 0x1110
    pub falcon_dmatrfmoffs: ReadWrite<u32>,  // 0x1114
    pub falcon_dmatrfcmd: ReadWrite<u32>,    // 0x1118
    pub falcon_dmatrffboffs: ReadWrite<u32>, // 0x111c
    pub falcon_dmapoll_fb: ReadWrite<u32>,   // 0x1120
    pub falcon_dmapoll_cp: ReadWrite<u32>,   // 0x1124
    pub falcon_dbg_state: ReadWrite<u32>,    // 0x1128
    pub falcon_hwcfg1: ReadWrite<u32>,       // 0x112c
    pub falcon_cpuctl_alias: ReadWrite<u32>, // 0x1130
    reserved86: [u8; 0x4],
    pub falcon_stackcfg: ReadWrite<u32>, // 0x1138
    reserved87: [u8; 0x4],
    pub falcon_imctl: ReadWrite<u32>,       // 0x1140
    pub falcon_imstat: ReadWrite<u32>,      // 0x1144
    pub falcon_traceidx: ReadWrite<u32>,    // 0x1148
    pub falcon_tracepc: ReadWrite<u32>,     // 0x114c
    pub falcon_imfillrng0: ReadWrite<u32>,  // 0x1150
    pub falcon_imfillrng1: ReadWrite<u32>,  // 0x1154
    pub falcon_imfillctl: ReadWrite<u32>,   // 0x1158
    pub falcon_imctl_debug: ReadWrite<u32>, // 0x115c
    pub falcon_cmembase: ReadWrite<u32>,    // 0x1160
    pub falcon_dmemapert: ReadWrite<u32>,   // 0x1164
    pub falcon_exterraddr: ReadWrite<u32>,  // 0x1168
    pub falcon_exterrstat: ReadWrite<u32>,  // 0x116c
    reserved99: [u8; 0xc],
    pub falcon_cg2: ReadWrite<u32>,    // 0x117c
    pub falcon_imemc0: ReadWrite<u32>, // 0x1180
    pub falcon_imemd0: ReadWrite<u32>, // 0x1184
    pub falcon_imemt0: ReadWrite<u32>, // 0x1188
    reserved103: [u8; 0x4],
    pub falcon_imemc1: ReadWrite<u32>, // 0x1190
    pub falcon_imemd1: ReadWrite<u32>, // 0x1194
    pub falcon_imemt1: ReadWrite<u32>, // 0x1198
    reserved106: [u8; 0x4],
    pub falcon_imemc2: ReadWrite<u32>, // 0x11a0
    pub falcon_imemd2: ReadWrite<u32>, // 0x11a4
    pub falcon_imemt2: ReadWrite<u32>, // 0x11a8
    reserved109: [u8; 0x4],
    pub falcon_imemc3: ReadWrite<u32>, // 0x11b0
    pub falcon_imemd3: ReadWrite<u32>, // 0x11b4
    pub falcon_imemt3: ReadWrite<u32>, // 0x11b8
    reserved112: [u8; 0x4],
    pub falcon_dmemc0: ReadWrite<u32>,    // 0x11c0
    pub falcon_dmemd0: ReadWrite<u32>,    // 0x11c4
    pub falcon_dmemc1: ReadWrite<u32>,    // 0x11c8
    pub falcon_dmemd1: ReadWrite<u32>,    // 0x11cc
    pub falcon_dmemc2: ReadWrite<u32>,    // 0x11d0
    pub falcon_dmemd2: ReadWrite<u32>,    // 0x11d4
    pub falcon_dmemc3: ReadWrite<u32>,    // 0x11d8
    pub falcon_dmemd3: ReadWrite<u32>,    // 0x11dc
    pub falcon_dmemc4: ReadWrite<u32>,    // 0x11e0
    pub falcon_dmemd4: ReadWrite<u32>,    // 0x11e4
    pub falcon_dmemc5: ReadWrite<u32>,    // 0x11e8
    pub falcon_dmemd5: ReadWrite<u32>,    // 0x11ec
    pub falcon_dmemc6: ReadWrite<u32>,    // 0x11f0
    pub falcon_dmemd6: ReadWrite<u32>,    // 0x11f4
    pub falcon_dmemc7: ReadWrite<u32>,    // 0x11f8
    pub falcon_dmemd7: ReadWrite<u32>,    // 0x11fc
    pub falcon_icd_cmd: ReadWrite<u32>,   // 0x1200
    pub falcon_icd_addr: ReadWrite<u32>,  // 0x1204
    pub falcon_icd_wdata: ReadWrite<u32>, // 0x1208
    pub falcon_icd_rdata: ReadWrite<u32>, // 0x120c
    reserved132: [u8; 0x30],
    pub falcon_sctl: ReadWrite<u32>,    // 0x1240
    pub falcon_sstat: ReadWrite<u32>,   // 0x1244
    pub falcon_unk_248: ReadWrite<u32>, // 0x1248
    pub falcon_unk_24c: ReadWrite<u32>, // 0x124c
    pub falcon_unk_250: ReadWrite<u32>, // 0x1250
    reserved137: [u8; 0xc],
    pub falcon_unk_260: ReadWrite<u32>, // 0x1260
    reserved138: [u8; 0x1c],
    pub falcon_sprot_imem: ReadWrite<u32>,   // 0x1280
    pub falcon_sprot_dmem: ReadWrite<u32>,   // 0x1284
    pub falcon_sprot_cpuctl: ReadWrite<u32>, // 0x1288
    pub falcon_sprot_misc: ReadWrite<u32>,   // 0x128c
    pub falcon_sprot_irq: ReadWrite<u32>,    // 0x1290
    pub falcon_sprot_mthd: ReadWrite<u32>,   // 0x1294
    pub falcon_sprot_sctl: ReadWrite<u32>,   // 0x1298
    pub falcon_sprot_wdtmr: ReadWrite<u32>,  // 0x129c
    reserved146: [u8; 0x20],
    pub falcon_dmainfo_finished_fbrd_low: ReadWrite<u32>, // 0x12c0
    pub falcon_dmainfo_finished_fbrd_high: ReadWrite<u32>, // 0x12c4
    pub falcon_dmainfo_finished_fbwr_low: ReadWrite<u32>, // 0x12c8
    pub falcon_dmainfo_finished_fbwr_high: ReadWrite<u32>, // 0x12cc
    pub falcon_dmainfo_current_fbrd_low: ReadWrite<u32>,  // 0x12d0
    pub falcon_dmainfo_current_fbrd_high: ReadWrite<u32>, // 0x12d4
    pub falcon_dmainfo_current_fbwr_low: ReadWrite<u32>,  // 0x12d8
    pub falcon_dmainfo_current_fbwr_high: ReadWrite<u32>, // 0x12dc
    pub falcon_dmainfo_ctl: ReadWrite<u32>,               // 0x12e0
    reserved155: [u8; 0x11c],
    pub tsec_scp_ctl0: ReadWrite<u32>,     // 0x1400
    pub tsec_scp_ctl1: ReadWrite<u32>,     // 0x1404
    pub tsec_scp_ctl_stat: ReadWrite<u32>, // 0x1408
    pub tsec_scp_ctl_lock: ReadWrite<u32>, // 0x140c
    pub tsec_scp_unk_10: ReadWrite<u32>,   // 0x1410
    pub tsec_scp_unk_14: ReadWrite<u32>,   // 0x1414
    pub tsec_scp_ctl_pkey: ReadWrite<u32>, // 0x1418
    pub tsec_scp_unk_1c: ReadWrite<u32>,   // 0x141c
    pub tsec_scp_seq_ctl: ReadWrite<u32>,  // 0x1420
    pub tsec_scp_seq_val: ReadWrite<u32>,  // 0x1424
    pub tsec_scp_seq_stat: ReadWrite<u32>, // 0x1428
    reserved166: [u8; 0x4],
    pub tsec_scp_insn_stat: ReadWrite<u32>, // 0x1430
    reserved167: [u8; 0x1c],
    pub tsec_scp_unk_50: ReadWrite<u32>,    // 0x1450
    pub tsec_scp_auth_stat: ReadWrite<u32>, // 0x1454
    pub tsec_scp_aes_stat: ReadWrite<u32>,  // 0x1458
    reserved170: [u8; 0x14],
    pub tsec_scp_unk_70: ReadWrite<u32>, // 0x1470
    reserved171: [u8; 0xc],
    pub tsec_scp_irqstat: ReadWrite<u32>, // 0x1480
    pub tsec_scp_irqmask: ReadWrite<u32>, // 0x1484
    reserved173: [u8; 0x8],
    pub tsec_scp_acl_err: ReadWrite<u32>,  // 0x1490
    pub tsec_scp_unk_94: ReadWrite<u32>,   // 0x1494
    pub tsec_scp_insn_err: ReadWrite<u32>, // 0x1498
    reserved176: [u8; 0x64],
    pub tsec_trng_clk_limit_low: ReadWrite<u32>, // 0x1500
    pub tsec_trng_clk_limit_high: ReadWrite<u32>, // 0x1504
    pub tsec_trng_unk_08: ReadWrite<u32>,        // 0x1508
    pub tsec_trng_test_ctl: ReadWrite<u32>,      // 0x150c
    pub tsec_trng_test_cfg0: ReadWrite<u32>,     // 0x1510
    pub tsec_trng_test_seed0: ReadWrite<u32>,    // 0x1514
    pub tsec_trng_test_cfg1: ReadWrite<u32>,     // 0x1518
    pub tsec_trng_test_seed1: ReadWrite<u32>,    // 0x151c
    pub tsec_trng_unk_20: ReadWrite<u32>,        // 0x1520
    pub tsec_trng_unk_24: ReadWrite<u32>,        // 0x1524
    pub tsec_trng_unk_28: ReadWrite<u32>,        // 0x1528
    pub tsec_trng_ctl: ReadWrite<u32>,           // 0x152c
    reserved188: [u8; 0xd0],
    pub tsec_tfbif_ctl: ReadWrite<u32>,             // 0x1600
    pub tsec_tfbif_mccif_fifoctrl: ReadWrite<u32>,  // 0x1604
    pub tsec_tfbif_throttle: ReadWrite<u32>,        // 0x1608
    pub tsec_tfbif_dbg_stat0: ReadWrite<u32>,       // 0x160c
    pub tsec_tfbif_dbg_stat1: ReadWrite<u32>,       // 0x1610
    pub tsec_tfbif_dbg_rdcount_lo: ReadWrite<u32>,  // 0x1614
    pub tsec_tfbif_dbg_rdcount_hi: ReadWrite<u32>,  // 0x1618
    pub tsec_tfbif_dbg_wrcount_lo: ReadWrite<u32>,  // 0x161c
    pub tsec_tfbif_dbg_wrcount_hi: ReadWrite<u32>,  // 0x1620
    pub tsec_tfbif_dbg_r32count: ReadWrite<u32>,    // 0x1624
    pub tsec_tfbif_dbg_r64count: ReadWrite<u32>,    // 0x1628
    pub tsec_tfbif_dbg_r128count: ReadWrite<u32>,   // 0x162c
    pub tsec_tfbif_unk_30: ReadWrite<u32>,          // 0x1630
    pub tsec_tfbif_mccif_fifoctrl1: ReadWrite<u32>, // 0x1634
    pub tsec_tfbif_wrr_rdp: ReadWrite<u32>,         // 0x1638
    reserved203: [u8; 0x4],
    pub tsec_tfbif_sprot_emem: ReadWrite<u32>, // 0x1640
    pub tsec_tfbif_transcfg: ReadWrite<u32>,   // 0x1644
    pub tsec_tfbif_regioncfg: ReadWrite<u32>,  // 0x1648
    pub tsec_tfbif_actmon_active_mask: ReadWrite<u32>, // 0x164c
    pub tsec_tfbif_actmon_active_borps: ReadWrite<u32>, // 0x1650
    pub tsec_tfbif_actmon_active_weight: ReadWrite<u32>, // 0x1654
    reserved209: [u8; 0x8],
    pub tsec_tfbif_actmon_mcb_mask: ReadWrite<u32>, // 0x1660
    pub tsec_tfbif_actmon_mcb_borps: ReadWrite<u32>, // 0x1664
    pub tsec_tfbif_actmon_mcb_weight: ReadWrite<u32>, // 0x1668
    reserved212: [u8; 0x4],
    pub tsec_tfbif_thi_transprop: ReadWrite<u32>, // 0x1670
    reserved213: [u8; 0x5c],
    pub tsec_cg: ReadWrite<u32>, // 0x16d0
    reserved214: [u8; 0x2c],
    pub tsec_bar0_ctl: ReadWrite<u32>,     // 0x1700
    pub tsec_bar0_addr: ReadWrite<u32>,    // 0x1704
    pub tsec_bar0_data: ReadWrite<u32>,    // 0x1708
    pub tsec_bar0_timeout: ReadWrite<u32>, // 0x170c
    reserved218: [u8; 0xf0],
    pub tsec_tegra_falcon_ip_ver: ReadWrite<u32>, // 0x1800
    pub tsec_tegra_unk_04: ReadWrite<u32>,        // 0x1804
    pub tsec_tegra_unk_08: ReadWrite<u32>,        // 0x1808
    pub tsec_tegra_unk_0c: ReadWrite<u32>,        // 0x180c
    pub tsec_tegra_unk_10: ReadWrite<u32>,        // 0x1810
    pub tsec_tegra_unk_14: ReadWrite<u32>,        // 0x1814
    pub tsec_tegra_unk_18: ReadWrite<u32>,        // 0x1818
    pub tsec_tegra_unk_1c: ReadWrite<u32>,        // 0x181c
    pub tsec_tegra_unk_20: ReadWrite<u32>,        // 0x1820
    pub tsec_tegra_unk_24: ReadWrite<u32>,        // 0x1824
    pub tsec_tegra_unk_28: ReadWrite<u32>,        // 0x1828
    pub tsec_tegra_unk_2c: ReadWrite<u32>,        // 0x182c
    pub tsec_tegra_unk_30: ReadWrite<u32>,        // 0x1830
    pub tsec_tegra_unk_34: ReadWrite<u32>,        // 0x1834
    pub tsec_tegra_ctl: ReadWrite<u32>,           // 0x1838
}

impl TsecRegisters {
    pub fn get() -> *const Self {
        0x5450_000 as *const TsecRegisters
    }
}

/// TSEC representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tsec {
    registers: *const TsecRegisters,
}

impl Tsec {
    fn dma_wait_idle(&self) -> Result<(), ()> {
        let registers = unsafe { &(*self.registers) };
        let timeout = get_milliseconds() + 10000;

        while (registers.falcon_dmatrfcmd.get() & (1 << 1)) == 0 {
            if get_milliseconds() > timeout {
                return Err(());
            }
        }

        Ok(())
    }

    fn dma_phys_to_flcn(
        &self,
        is_imem: bool,
        flcn_offset: u32,
        phys_offset: u32,
    ) -> Result<(), ()> {
        let registers = unsafe { &(*self.registers) };

        let cmd = if is_imem { 0x10 } else { 0x600 };

        self.registers.falcon_dmatrfmoffs.set(flcn_offset);
        self.registers.falcon_dmatrffboffs.set(phys_offset);
        self.registers.falcon_dmatrfcmd.set(cmd);

        self.dma_wait_idle()
    }

    /// Creates a new TSEC object.
    pub fn new() -> Self {
        Tsec {
            registers: TsecRegisters::get(),
        }
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
    pub fn get_key(&self, key: &mut [u32], rev: u32, firmware: &mut [u8]) -> Result<(), ()> {
        self.enable_clocks();

        let registers = unsafe { &(*self.registers) };

        // Configure Falcon.
        registers.falcon_dmactl.set(0);
        registers.falcon_irqmset.set(0xFFF2);
        registers.falcon_irqdest.set(0xFFF0);
        registers.falcon_itfen.set(3);

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
        self.execute_firmware(rev);

        if self.dma_wait_idle().is_err() {
            self.disable_clocks();
            return Err(());
        }

        let timeout = get_milliseconds() + 2000;
        while registers.falcon_mailbox1.get() == 0 {
            if get_milliseconds() > timeout {
                self.disable_clocks();
                return Err(());
            }
        }

        if registers.falcon_mailbox1.get() != 0xB0B0_B0B0 {
            self.disable_clocks();
            return Err(());
        }

        // Unknown HOST1X write.
        let host1x_reg = unsafe { &(*((0x5000_0000 + 0x3300) as *const ReadWrite<u32>)) };
        host1x_reg.set(0);

        // Fetch result from SOR1.
        let sor1_dp_hdcp_bksv_lsb = unsafe { &(*((0x5458_0000 + 0x1EB) as *const ReadWrite<u32>)) };
        let sor1_tmds_hdcp_bksv_lsb =
            unsafe { &(*((0x5458_0000 + 0x21C) as *const ReadWrite<u32>)) };
        let sor1_tmds_hdcp_cn_msb = unsafe { &(*((0x5458_0000 + 0x208) as *const ReadWrite<u32>)) };
        let sor1_tmds_hdcp_cn_lsb = unsafe { &(*((0x5458_0000 + 0x20C) as *const ReadWrite<u32>)) };

        let mut temp: [u32; 0x4] = [0; 4];
        temp[0] = sor1_dp_hdcp_bksv_lsb.get();
        temp[1] = sor1_tmds_hdcp_bksv_lsb.get();
        temp[2] = sor1_tmds_hdcp_cn_msb.get();
        temp[3] = sor1_tmds_hdcp_cn_lsb.get();

        // Clear SOR1 registers.
        sor1_dp_hdcp_bksv_lsb.set(0);
        sor1_tmds_hdcp_bksv_lsb.set(0);
        sor1_tmds_hdcp_cn_msb.set(0);
        sor1_tmds_hdcp_cn_lsb.set(0);

        // Copy back the key.
        key.copy_from_slice(temp[..0x10].as_ref());

        Ok(())
    }

    /// Loads the TSEC firmware.
    pub fn load_firmware(&self, firmware: &mut [u8]) -> Result<(), ()> {
        let mut res = Ok(());

        let registers = unsafe { &(*self.registers) };

        // Configure Falcon.
        registers.falcon_dmactl.set(0);
        registers.falcon_irqmset.set(0xFFF2);
        registers.falcon_irqdest.set(0xFFF0);
        registers.falcon_itfen.set(3);

        if self.dma_wait_idle().is_ok() {
            // Load firmware.
            registers
                .falcon_dmatrfbase
                .set(firmware.read_u32::<LittleEndian>().unwrap() >> 8);

            let addr = 0;
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
    pub fn execute_firmware(&self, rev: u32) {
        let registers = unsafe { &(*self.registers) };

        // Unknown HOST1X write.
        let host1x_reg = unsafe { &(*((0x5000_0000 + 0x3300) as *const ReadWrite<u32>)) };
        host1x_reg.set(0x34C2_E1DA);

        // Execute firmware.
        registers.falcon_mailbox1.set(0);
        registers.falcon_mailbox0.set(rev);
        registers.falcon_bootvec.set(0);
        registers.falcon_cpuctl.set(2);
    }
}
