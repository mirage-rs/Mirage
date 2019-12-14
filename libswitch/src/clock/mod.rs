//! Tegra 210 Clock And Reset Controller interface and configurations.
//!
//! # Description
//!
//! The Clock and Reset (CAR) block contains all the logic needed to
//! control most of the clocks and resets to the Tegra X1 device.
//! The CAR block provides the registers to program the PLLs and
//! controls most of the clock source programming, and most of the
//! clock dividers.
//!
//! Generally speaking, clocks are used to set up non-boot devices
//! for operation.
//!
//! # Implementation
//!
//! - The [`Car`] struct can be used to access the CAR registers.
//!
//! - The [`Clock`] struct is an abstraction of a device clock which
//! holds all the important configuration values for controlling it.
//!
//! - [`Clock`] holds pre-defined constants which represent known clocks.
//! These can be used for convenience.
//!
//! - [`Clock::enable`], [`Clock::disable`] and [`Clock::is_enabled`] can
//! be used to check and modify the state of a device.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::clock::Clock;
//!
//! fn main() {
//!     let se_clock = Clock::SE;
//!
//!     // Enable Security Engine.
//!     se_clock.enable();
//!     assert_eq!(se_clock.is_enabled(), true);
//!
//!     // Disable Security Engine.
//!     se_clock.disable();
//!     assert_eq!(se_clock.is_enabled(), false);
//! }
//! ```
//!
//! [`Car`]: struct.Car.html
//! [`Clock`]: struct.Clock.html
//! [`Clock::enable`]: struct.Clock.html#method.enable
//! [`Clock::disable`]: struct.Clock.html#method.disable
//! [`Clock::is_enabled`]: struct.Clock.html#method.is_enabled

use mirage_mmio::{BlockMmio, Mmio, VolatileStorage};

/// Base address for clock registers.
pub(crate) const CLOCK_BASE: u32 = 0x6000_6000;

pub const CLK_L_SDMMC1: u32 = (1 << 14);
pub const CLK_L_SDMMC2: u32 = (1 << 9);
pub const CLK_U_SDMMC3: u32 = (1 << 5);
pub const CLK_L_SDMMC4: u32 = (1 << 15);

pub const CLK_SOURCE_MASK: u32 = (0b111 << 29);
pub const CLK_SOURCE_FIRST: u32 = (0b000 << 29);
pub const CLK_DIVIDER_MASK: u32 = (0xff << 0);
pub const CLK_DIVIDER_UNITY: u32 = (0x00 << 0);

/// Representation of the CAR.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Car {
    pub rst_src: BlockMmio<u32>,

    pub rst_dev_l: BlockMmio<u32>,
    pub rst_dev_h: BlockMmio<u32>,
    pub rst_dev_u: BlockMmio<u32>,

    pub clk_out_enb_l: BlockMmio<u32>,
    pub clk_out_enb_h: BlockMmio<u32>,
    pub clk_out_enb_u: BlockMmio<u32>,

    _0x1C: BlockMmio<u32>,
    pub cclk_brst_pol: BlockMmio<u32>,
    pub super_cclk_div: BlockMmio<u32>,
    pub sclk_brst_pol: BlockMmio<u32>,
    pub super_sclk_div: BlockMmio<u32>,
    pub clk_sys_rate: BlockMmio<u32>,
    pub prog_dly_clk: BlockMmio<u32>,
    pub aud_sync_clk_rate: BlockMmio<u32>,
    _0x3C: BlockMmio<u32>,
    pub cop_clk_skip_plcy: BlockMmio<u32>,
    pub clk_mask_arm: BlockMmio<u32>,
    pub misc_clk_enb: BlockMmio<u32>,
    pub clk_cpu_cmplx: BlockMmio<u32>,
    pub osc_ctrl: BlockMmio<u32>,
    pub pll_lfsr: BlockMmio<u32>,
    pub osc_freq_det: BlockMmio<u32>,
    pub osc_freq_det_stat: BlockMmio<u32>,
    _0x60: [BlockMmio<u32>; 2],
    pub plle_ss_cntl: BlockMmio<u32>,
    pub plle_misc1: BlockMmio<u32>,
    _0x70: [BlockMmio<u32>; 4],

    pub pllc_base: BlockMmio<u32>,
    pub pllc_out: BlockMmio<u32>,
    pub pllc_misc0: BlockMmio<u32>,
    pub pllc_misc1: BlockMmio<u32>,

    pub pllm_base: BlockMmio<u32>,
    pub pllm_out: BlockMmio<u32>,
    pub pllm_misc1: BlockMmio<u32>,
    pub pllm_misc2: BlockMmio<u32>,

    pub pllp_base: BlockMmio<u32>,
    pub pllp_outa: BlockMmio<u32>,
    pub pllp_outb: BlockMmio<u32>,
    pub pllp_misc: BlockMmio<u32>,

    pub plla_base: BlockMmio<u32>,
    pub plla_out: BlockMmio<u32>,
    pub plla_misc0: BlockMmio<u32>,
    pub plla_misc1: BlockMmio<u32>,

    pub pllu_base: BlockMmio<u32>,
    pub pllu_out: BlockMmio<u32>,
    pub pllu_misc1: BlockMmio<u32>,
    pub pllu_misc2: BlockMmio<u32>,

    pub plld_base: BlockMmio<u32>,
    pub plld_out: BlockMmio<u32>,
    pub plld_misc1: BlockMmio<u32>,
    pub plld_misc2: BlockMmio<u32>,

    pub pllx_base: BlockMmio<u32>,
    pub pllx_misc: BlockMmio<u32>,

    pub plle_base: BlockMmio<u32>,
    pub plle_misc: BlockMmio<u32>,
    pub plle_ss_cntl1: BlockMmio<u32>,
    pub plle_ss_cntl2: BlockMmio<u32>,

    pub lvl2_clk_gate_ovra: BlockMmio<u32>,
    pub lvl2_clk_gate_ovrb: BlockMmio<u32>,

    pub clk_source_i2s2: BlockMmio<u32>,
    pub clk_source_i2s3: BlockMmio<u32>,
    pub clk_source_spdif_out: BlockMmio<u32>,
    pub clk_source_spdif_in: BlockMmio<u32>,
    pub clk_source_pwm: BlockMmio<u32>,
    _0x114: BlockMmio<u32>,
    pub clk_source_spi2: BlockMmio<u32>,
    pub clk_source_spi3: BlockMmio<u32>,
    _0x120: BlockMmio<u32>,
    pub clk_source_i2c1: BlockMmio<u32>,
    pub clk_source_i2c5: BlockMmio<u32>,
    _0x12c: [BlockMmio<u32>; 2],
    pub clk_source_spi1: BlockMmio<u32>,
    pub clk_source_disp1: BlockMmio<u32>,
    pub clk_source_disp2: BlockMmio<u32>,
    _0x140: BlockMmio<u32>,
    pub clk_source_isp: BlockMmio<u32>,
    pub clk_source_vi: BlockMmio<u32>,
    _0x14c: BlockMmio<u32>,
    pub clk_source_sdmmc1: BlockMmio<u32>,
    pub clk_source_sdmmc2: BlockMmio<u32>,
    _0x158: [BlockMmio<u32>; 3],
    pub clk_source_sdmmc4: BlockMmio<u32>,
    _0x168: [BlockMmio<u32>; 4],
    pub clk_source_uarta: BlockMmio<u32>,
    pub clk_source_uartb: BlockMmio<u32>,
    pub clk_source_host1x: BlockMmio<u32>,
    _0x184: [BlockMmio<u32>; 5],
    pub clk_source_i2c2: BlockMmio<u32>,
    pub clk_source_emc: BlockMmio<u32>,
    pub clk_source_uartc: BlockMmio<u32>,
    _0x1a4: BlockMmio<u32>,
    pub clk_source_vi_sensor: BlockMmio<u32>,
    _0x1ac: [BlockMmio<u32>; 2],
    pub clk_source_spi4: BlockMmio<u32>,
    pub clk_source_i2c3: BlockMmio<u32>,
    pub clk_source_sdmmc3: BlockMmio<u32>,
    pub clk_source_uartd: BlockMmio<u32>,
    _0x1c4: [BlockMmio<u32>; 2],
    pub clk_source_owr: BlockMmio<u32>,
    _0x1d0: BlockMmio<u32>,
    pub clk_source_csite: BlockMmio<u32>,
    pub clk_source_i2s1: BlockMmio<u32>,
    pub clk_source_dtv: BlockMmio<u32>,
    _0x1e0: [BlockMmio<u32>; 5],
    pub clk_source_tsec: BlockMmio<u32>,
    _0x1f8: BlockMmio<u32>,

    pub clk_spare2: BlockMmio<u32>,
    _0x200: [BlockMmio<u32>; 32],

    pub clk_out_enb_x: BlockMmio<u32>,
    pub clk_enb_x_set: BlockMmio<u32>,
    pub clk_enb_x_clr: BlockMmio<u32>,

    pub rst_devices_x: BlockMmio<u32>,
    pub rst_dev_x_set: BlockMmio<u32>,
    pub rst_dev_x_clr: BlockMmio<u32>,

    pub clk_out_enb_y: BlockMmio<u32>,
    pub clk_enb_y_set: BlockMmio<u32>,
    pub clk_enb_y_clr: BlockMmio<u32>,

    pub rst_devices_y: BlockMmio<u32>,
    pub rst_dev_y_set: BlockMmio<u32>,
    pub rst_dev_y_clr: BlockMmio<u32>,

    _0x2b0: [BlockMmio<u32>; 17],
    pub dfll_base: BlockMmio<u32>,
    _0x2f8: [BlockMmio<u32>; 2],

    pub rst_dev_l_set: BlockMmio<u32>,
    pub rst_dev_l_clr: BlockMmio<u32>,
    pub rst_dev_h_set: BlockMmio<u32>,
    pub rst_dev_h_clr: BlockMmio<u32>,
    pub rst_dev_u_set: BlockMmio<u32>,
    pub rst_dev_u_clr: BlockMmio<u32>,

    _0x318: [BlockMmio<u32>; 2],

    pub clk_enb_l_set: BlockMmio<u32>,
    pub clk_enb_l_clr: BlockMmio<u32>,
    pub clk_enb_h_set: BlockMmio<u32>,
    pub clk_enb_h_clr: BlockMmio<u32>,
    pub clk_enb_u_set: BlockMmio<u32>,
    pub clk_enb_u_clr: BlockMmio<u32>,

    _0x338: BlockMmio<u32>,
    pub ccplex_pg_sm_ovrd: BlockMmio<u32>,
    pub rst_cpu_cmplx_set: BlockMmio<u32>,
    pub rst_cpu_cmplx_clr: BlockMmio<u32>,

    pub clk_cpu_cmplx_set: BlockMmio<u32>,
    pub clk_cpu_cmplx_clr: BlockMmio<u32>,

    _0x350: [BlockMmio<u32>; 2],
    pub rst_dev_v: BlockMmio<u32>,
    pub rst_dev_w: BlockMmio<u32>,
    pub clk_out_enb_v: BlockMmio<u32>,
    pub clk_out_enb_w: BlockMmio<u32>,
    pub cclkg_brst_pol: BlockMmio<u32>,
    pub super_cclkg_div: BlockMmio<u32>,
    pub cclklp_brst_pol: BlockMmio<u32>,
    pub super_cclkp_div: BlockMmio<u32>,
    pub clk_cpug_cmplx: BlockMmio<u32>,
    pub clk_cpulp_cmplx: BlockMmio<u32>,
    pub cpu_softrst_ctrl: BlockMmio<u32>,
    pub cpu_softrst_ctrl1: BlockMmio<u32>,
    pub cpu_softrst_ctrl2: BlockMmio<u32>,
    _0x38c: [BlockMmio<u32>; 5],
    pub lvl2_clk_gate_ovrc: BlockMmio<u32>,
    pub lvl2_clk_gate_ovrd: BlockMmio<u32>,
    _0x3a8: [BlockMmio<u32>; 2],

    _0x3b0: BlockMmio<u32>,
    pub clk_source_mselect: BlockMmio<u32>,
    pub clk_source_tsensor: BlockMmio<u32>,
    pub clk_source_i2s4: BlockMmio<u32>,
    pub clk_source_i2s5: BlockMmio<u32>,
    pub clk_source_i2c4: BlockMmio<u32>,
    _0x3c8: [BlockMmio<u32>; 2],
    pub clk_source_ahub: BlockMmio<u32>,
    _0x3d4: [BlockMmio<u32>; 4],
    pub clk_source_hda2codec_2x: BlockMmio<u32>,
    pub clk_source_actmon: BlockMmio<u32>,
    pub clk_source_extperiph1: BlockMmio<u32>,
    pub clk_source_extperiph2: BlockMmio<u32>,
    pub clk_source_extperiph3: BlockMmio<u32>,
    _0x3f8: BlockMmio<u32>,
    pub clk_source_i2c_slow: BlockMmio<u32>,
    pub clk_source_sys: BlockMmio<u32>,
    pub clk_source_ispb: BlockMmio<u32>,
    _0x408: [BlockMmio<u32>; 2],
    pub clk_source_sor1: BlockMmio<u32>,
    pub clk_source_sor0: BlockMmio<u32>,
    _0x418: [BlockMmio<u32>; 2],
    pub clk_source_sata_oob: BlockMmio<u32>,
    pub clk_source_sata: BlockMmio<u32>,
    pub clk_source_hda: BlockMmio<u32>,
    _0x42c: BlockMmio<u32>,

    pub rst_dev_v_set: BlockMmio<u32>,
    pub rst_dev_v_clr: BlockMmio<u32>,
    pub rst_dev_w_set: BlockMmio<u32>,
    pub rst_dev_w_clr: BlockMmio<u32>,

    pub clk_enb_v_set: BlockMmio<u32>,
    pub clk_enb_v_clr: BlockMmio<u32>,
    pub clk_enb_w_set: BlockMmio<u32>,
    pub clk_enb_w_clr: BlockMmio<u32>,

    pub rst_cpug_cmplx_set: BlockMmio<u32>,
    pub rst_cpug_cmplx_clr: BlockMmio<u32>,
    pub rst_cpulp_cmplx_set: BlockMmio<u32>,
    pub rst_cpulp_cmplx_clr: BlockMmio<u32>,
    pub clk_cpug_cmplx_set: BlockMmio<u32>,
    pub clk_cpug_cmplx_clr: BlockMmio<u32>,
    pub clk_cpulp_cmplx_set: BlockMmio<u32>,
    pub clk_cpulp_cmplx_clr: BlockMmio<u32>,
    pub cpu_cmplx_status: BlockMmio<u32>,
    _0x474: BlockMmio<u32>,
    pub intstatus: BlockMmio<u32>,
    pub intmask: BlockMmio<u32>,
    pub utmip_pll_cfg0: BlockMmio<u32>,
    pub utmip_pll_cfg1: BlockMmio<u32>,
    pub utmip_pll_cfg2: BlockMmio<u32>,

    pub plle_aux: BlockMmio<u32>,
    pub sata_pll_cfg0: BlockMmio<u32>,
    pub sata_pll_cfg1: BlockMmio<u32>,
    pub pcie_pll_cfg0: BlockMmio<u32>,

    pub prog_audio_dly_clk: BlockMmio<u32>,
    pub audio_sync_clk_i2s0: BlockMmio<u32>,
    pub audio_sync_clk_i2s1: BlockMmio<u32>,
    pub audio_sync_clk_i2s2: BlockMmio<u32>,
    pub audio_sync_clk_i2s3: BlockMmio<u32>,
    pub audio_sync_clk_i2s4: BlockMmio<u32>,
    pub audio_sync_clk_spdif: BlockMmio<u32>,

    pub plld2_base: BlockMmio<u32>,
    pub plld2_misc: BlockMmio<u32>,
    pub utmip_pll_cfg3: BlockMmio<u32>,
    pub pllrefe_base: BlockMmio<u32>,
    pub pllrefe_misc: BlockMmio<u32>,
    pub pllrefe_out: BlockMmio<u32>,
    pub cpu_finetrim_byp: BlockMmio<u32>,
    pub cpu_finetrim_select: BlockMmio<u32>,
    pub cpu_finetrim_dr: BlockMmio<u32>,
    pub cpu_finetrim_df: BlockMmio<u32>,
    pub cpu_finetrim_f: BlockMmio<u32>,
    pub cpu_finetrim_r: BlockMmio<u32>,
    pub pllc2_base: BlockMmio<u32>,
    pub pllc2_misc0: BlockMmio<u32>,
    pub pllc2_misc1: BlockMmio<u32>,
    pub pllc2_misc2: BlockMmio<u32>,
    pub pllc2_misc3: BlockMmio<u32>,
    pub pllc3_base: BlockMmio<u32>,
    pub pllc3_misc0: BlockMmio<u32>,
    pub pllc3_misc1: BlockMmio<u32>,
    pub pllc3_misc2: BlockMmio<u32>,
    pub pllc3_misc3: BlockMmio<u32>,
    pub pllx_misc1: BlockMmio<u32>,
    pub pllx_misc2: BlockMmio<u32>,
    pub pllx_misc3: BlockMmio<u32>,
    pub xusbio_pll_cfg0: BlockMmio<u32>,
    pub xusbio_pll_cfg1: BlockMmio<u32>,
    pub plle_aux1: BlockMmio<u32>,
    pub pllp_reshift: BlockMmio<u32>,
    pub utmipll_hw_pwrdn_cfg0: BlockMmio<u32>,
    pub pllu_hw_pwrdn_cfg0: BlockMmio<u32>,
    pub xusb_pll_cfg0: BlockMmio<u32>,
    _0x538: BlockMmio<u32>,
    pub clk_cpu_misc: BlockMmio<u32>,
    pub clk_cpug_misc: BlockMmio<u32>,
    pub clk_cpulp_misc: BlockMmio<u32>,
    pub pllx_hw_ctrl_cfg: BlockMmio<u32>,
    pub pllx_sw_ramp_cfg: BlockMmio<u32>,
    pub pllx_hw_ctrl_status: BlockMmio<u32>,
    pub lvl2_clk_gate_ovre: BlockMmio<u32>,
    pub super_gr3d_clk_div: BlockMmio<u32>,
    pub spare_reg0: BlockMmio<u32>,
    pub audio_sync_clk_dmic1: BlockMmio<u32>,
    pub audio_sync_clk_dmic2: BlockMmio<u32>,

    _0x568: [BlockMmio<u32>; 2],
    pub plld2_ss_cfg: BlockMmio<u32>,
    pub plld2_ss_ctrl1: BlockMmio<u32>,
    pub plld2_ss_ctrl2: BlockMmio<u32>,
    _0x57c: [BlockMmio<u32>; 5],

    pub plldp_base: BlockMmio<u32>,
    pub plldp_misc: BlockMmio<u32>,
    pub plldp_ss_cfg: BlockMmio<u32>,
    pub plldp_ss_ctrl1: BlockMmio<u32>,
    pub plldp_ss_ctrl2: BlockMmio<u32>,
    pub pllc4_base: BlockMmio<u32>,
    pub pllc4_misc: BlockMmio<u32>,
    _0x5ac: [BlockMmio<u32>; 6],
    pub clk_spare0: BlockMmio<u32>,
    pub clk_spare1: BlockMmio<u32>,
    pub gpu_isob_ctrl: BlockMmio<u32>,
    pub pllc_misc2: BlockMmio<u32>,
    pub pllc_misc3: BlockMmio<u32>,
    pub plla_misc2: BlockMmio<u32>,
    _0x5dc: [BlockMmio<u32>; 2],
    pub pllc4_out: BlockMmio<u32>,
    pub pllmb_base: BlockMmio<u32>,
    pub pllmb_misc1: BlockMmio<u32>,
    pub pllx_misc4: BlockMmio<u32>,
    pub pllx_misc5: BlockMmio<u32>,
    _0x5f8: [BlockMmio<u32>; 2],

    pub clk_source_xusb_core_host: BlockMmio<u32>,
    pub clk_source_xusb_falcon: BlockMmio<u32>,
    pub clk_source_xusb_fs: BlockMmio<u32>,
    pub clk_source_xusb_core_dev: BlockMmio<u32>,
    pub clk_source_xusb_ss: BlockMmio<u32>,
    pub clk_source_cilab: BlockMmio<u32>,
    pub clk_source_cilcd: BlockMmio<u32>,
    pub clk_source_cilef: BlockMmio<u32>,
    pub clk_source_dsia_lp: BlockMmio<u32>,
    pub clk_source_dsib_lp: BlockMmio<u32>,
    pub clk_source_entropy: BlockMmio<u32>,
    pub clk_source_dvfs_ref: BlockMmio<u32>,
    pub clk_source_dvfs_soc: BlockMmio<u32>,
    _0x634: [BlockMmio<u32>; 3],
    pub clk_source_emc_latency: BlockMmio<u32>,
    pub clk_source_soc_therm: BlockMmio<u32>,
    _0x648: BlockMmio<u32>,
    pub clk_source_dmic1: BlockMmio<u32>,
    pub clk_source_dmic2: BlockMmio<u32>,
    _0x654: BlockMmio<u32>,
    pub clk_source_vi_sensor2: BlockMmio<u32>,
    pub clk_source_i2c6: BlockMmio<u32>,
    pub clk_source_mipibif: BlockMmio<u32>,
    pub clk_source_emc_dll: BlockMmio<u32>,
    _0x668: BlockMmio<u32>,
    pub clk_source_uart_fst_mipi_cal: BlockMmio<u32>,
    _0x670: [BlockMmio<u32>; 2],
    pub clk_source_vic: BlockMmio<u32>,

    pub pllp_outc: BlockMmio<u32>,
    pub pllp_misc1: BlockMmio<u32>,
    _0x684: [BlockMmio<u32>; 2],
    pub emc_div_clk_shaper_ctrl: BlockMmio<u32>,
    pub emc_pllc_shaper_ctrl: BlockMmio<u32>,

    pub clk_source_sdmmc_legacy_tm: BlockMmio<u32>,
    pub clk_source_nvdec: BlockMmio<u32>,
    pub clk_source_nvjpg: BlockMmio<u32>,
    pub clk_source_nvenc: BlockMmio<u32>,

    pub plla1_base: BlockMmio<u32>,
    pub plla1_misc0: BlockMmio<u32>,
    pub plla1_misc1: BlockMmio<u32>,
    pub plla1_misc2: BlockMmio<u32>,
    pub plla1_misc3: BlockMmio<u32>,
    pub audio_sync_clk_dmic3: BlockMmio<u32>,

    pub clk_source_dmic3: BlockMmio<u32>,
    pub clk_source_ape: BlockMmio<u32>,
    pub clk_source_qspi: BlockMmio<u32>,
    pub clk_source_vi_i2c: BlockMmio<u32>,
    pub clk_source_usb2_hsic_trk: BlockMmio<u32>,
    pub clk_source_pex_sata_usb_rx_byp: BlockMmio<u32>,
    pub clk_source_maud: BlockMmio<u32>,
    pub clk_source_tsecb: BlockMmio<u32>,

    pub clk_cpug_misc1: BlockMmio<u32>,
    pub aclk_burst_policy: BlockMmio<u32>,
    pub super_aclk_divider: BlockMmio<u32>,

    pub nvenc_super_clk_divider: BlockMmio<u32>,
    pub vi_super_clk_divider: BlockMmio<u32>,
    pub vic_super_clk_divider: BlockMmio<u32>,
    pub nvdec_super_clk_divider: BlockMmio<u32>,
    pub isp_super_clk_divider: BlockMmio<u32>,
    pub ispb_super_clk_divider: BlockMmio<u32>,
    pub nvjpg_super_clk_divider: BlockMmio<u32>,
    pub se_super_clk_divider: BlockMmio<u32>,
    pub tsec_super_clk_divider: BlockMmio<u32>,
    pub tsecb_super_clk_divider: BlockMmio<u32>,

    pub clk_source_uartape: BlockMmio<u32>,
    pub clk_cpug_misc2: BlockMmio<u32>,
    pub clk_source_dbgapb: BlockMmio<u32>,
    pub clk_ccplex_cc4_ret_clk_enb: BlockMmio<u32>,
    pub actmon_cpu_clk: BlockMmio<u32>,
    pub clk_source_emc_safe: BlockMmio<u32>,
    pub sdmmc2_pllc4_out0_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc2_pllc4_out1_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc2_pllc4_out2_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc2_div_clk_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc4_pllc4_out0_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc4_pllc4_out1_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc4_pllc4_out2_shaper_ctrl: BlockMmio<u32>,
    pub sdmmc4_div_clk_shaper_ctrl: BlockMmio<u32>,
}

impl VolatileStorage for Car {
    unsafe fn make_ptr() -> *const Self {
        CLOCK_BASE as *const _
    }
}

/// Representation of a device clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    /// The clock device reset register.
    reset: u32,
    /// The clock device enable register.
    enable: u32,
    /// The clock source register.
    source: u32,
    /// The clock index.
    index: u8,
    /// The clock source value.
    clock_source: u32,
    /// The clock divisor register.
    clock_divisor: u32,
}

const CLK_RST_CONTROLLER_RST_DEVICES_L: u32 = 0x4;
const CLK_RST_CONTROLLER_RST_DEVICES_H: u32 = 0x8;
const CLK_RST_CONTROLLER_RST_DEVICES_U: u32 = 0xC;
const CLK_RST_CONTROLLER_RST_DEVICES_X: u32 = 0x28C;
const CLK_RST_CONTROLLER_RST_DEVICES_Y: u32 = 0x2A4;
const CLK_RST_CONTROLLER_RST_DEVICES_V: u32 = 0x358;
const CLK_RST_CONTROLLER_RST_DEVICES_W: u32 = 0x35C;

const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_X: u32 = 0x280;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_Y: u32 = 0x298;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_V: u32 = 0x360;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_W: u32 = 0x364;

const CLK_NO_SOURCE: u32 = 0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_A: u32 = 0x178;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_B: u32 = 0x17C;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_C: u32 = 0x1A0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_D: u32 = 0x1C0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE: u32 = 0x710;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1: u32 = 0x124;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2: u32 = 0x198;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3: u32 = 0x1B8;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4: u32 = 0x3C4;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5: u32 = 0x128;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6: u32 = 0x65C;
const CLK_RST_CONTROLLER_CLK_SOURCE_SE: u32 = 0x42C;
const CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X: u32 = 0x180;
const CLK_RST_CONTROLLER_CLK_SOURCE_TSEC: u32 = 0x1F4;
const CLK_RST_CONTROLLER_CLK_SOURCE_SOR1: u32 = 0x410;
const CLK_RST_CONTROLLER_CLK_SOURCE_CSITE: u32 = 0x1D4;
const CLK_RST_CONTROLLER_CLK_SOURCE_PWM: u32 = 0x11;

// Definitions for known devices.
impl Clock {
    /// Representation of the UART A clock.
    pub const UART_A: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_A,
        index: 0x6,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART B clock.
    pub const UART_B: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_B,
        index: 0x7,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART C clock.
    pub const UART_C: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_C,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART D clock.
    pub const UART_D: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_D,
        index: 0x1,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART APE clock.
    pub const UART_APE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE,
        index: 0x14,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the I²C 1 clock.
    pub const I2C_1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1,
        index: 0xC,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 2 clock.
    pub const I2C_2: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2,
        index: 0x16,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 3 clock.
    pub const I2C_3: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3,
        index: 0x3,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 4 clock.
    pub const I2C_4: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4,
        index: 0x7,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 5 clock.
    pub const I2C_5: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5,
        index: 0xF,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the I²C 6 clock.
    pub const I2C_6: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6,
        index: 0x6,
        clock_source: 0x6,
        clock_divisor: 0,
    };

    /// Representation of the Security Engine clock.
    pub const SE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SE,
        index: 0x1F,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the TZRAM clock.
    pub const TZRAM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the HOST1X clock.
    pub const HOST1X: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X,
        index: 0x1C,
        clock_source: 0x4,
        clock_divisor: 0x3,
    };

    /// Representation of the TSEC clock.
    pub const TSEC: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSEC,
        index: 0x13,
        clock_source: 0,
        clock_divisor: 0x2,
    };

    /// Representation of the SOR_SAFE clock.
    pub const SOR_SAFE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR0 clock.
    pub const SOR0: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: 0x16,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR1 clock.
    pub const SOR1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SOR1,
        index: 0x17,
        clock_source: 0,
        clock_divisor: 0x2,
    };

    /// Representation of the KFUSE clock.
    pub const KFUSE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x8,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CL-DVFS clock.
    pub const CL_DVFS: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_W,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_W,
        source: CLK_NO_SOURCE,
        index: 0x1B,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CSITE clock.
    pub const CORESIGHT: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_CSITE,
        index: 0x9,
        clock_source: 0,
        clock_divisor: 0x4,
    };

    /// Representation of the PWM clock.
    pub const PWM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_PWM,
        index: 0x11,
        clock_source: 0x6,
        clock_divisor: 0x4,
    };
}

impl Clock {
    /// Sets whether the clock should be reset or not.
    fn set_reset(&self, set_reset: bool) {
        let reset_reg = unsafe { Mmio::new((CLOCK_BASE + self.reset) as *const u32) };

        let current_value = reset_reg.read();
        let mask = (1 << self.index & 0x1F) as u32;

        let new_value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        reset_reg.write(new_value);
    }

    /// Sets whether the clock should be enabled or disabled.
    fn set_enable(&self, set_enable: bool) {
        let enable_reg = unsafe { Mmio::new((CLOCK_BASE + self.enable) as *const u32) };

        let current_value = enable_reg.read();
        let mask = (1 << (self.index & 0x1F)) as u32;

        let new_value = if set_enable {
            current_value | mask
        } else {
            current_value & !mask
        };

        enable_reg.write(new_value);
    }

    /// Enables the clock.
    pub fn enable(&self) {
        // Put clock into reset.
        self.set_reset(true);

        // Disable clock.
        self.disable();

        // Setup clock source if needed.
        if self.source != 0 {
            unsafe {
                Mmio::new((CLOCK_BASE + self.source) as *const u32)
                    .write(self.clock_divisor | (self.clock_source << 29));
            }
        }

        // Enable clock.
        self.set_enable(true);
        self.set_reset(false);
    }

    /// Disables the clock.
    pub fn disable(&self) {
        // Put clock into reset.
        self.set_reset(true);
        // Disable.
        self.set_enable(false);
    }

    /// Whether the clock is enabled or not.
    pub fn is_enabled(&self) -> bool {
        let enable_reg = unsafe { Mmio::new((CLOCK_BASE + self.enable) as *const u32) };
        let mask = (1 << (self.index & 0x1F)) as u32;

        (enable_reg.read() & mask) == mask
    }
}
