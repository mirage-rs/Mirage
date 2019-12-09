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

use mirage_mmio::{Mmio, VolatileStorage};

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
    pub rst_src: Mmio<u32>,

    pub rst_dev_l: Mmio<u32>,
    pub rst_dev_h: Mmio<u32>,
    pub rst_dev_u: Mmio<u32>,

    pub clk_out_enb_l: Mmio<u32>,
    pub clk_out_enb_h: Mmio<u32>,
    pub clk_out_enb_u: Mmio<u32>,

    _0x1C: Mmio<u32>,
    pub cclk_brst_pol: Mmio<u32>,
    pub super_cclk_div: Mmio<u32>,
    pub sclk_brst_pol: Mmio<u32>,
    pub super_sclk_div: Mmio<u32>,
    pub clk_sys_rate: Mmio<u32>,
    pub prog_dly_clk: Mmio<u32>,
    pub aud_sync_clk_rate: Mmio<u32>,
    _0x3C: Mmio<u32>,
    pub cop_clk_skip_plcy: Mmio<u32>,
    pub clk_mask_arm: Mmio<u32>,
    pub misc_clk_enb: Mmio<u32>,
    pub clk_cpu_cmplx: Mmio<u32>,
    pub osc_ctrl: Mmio<u32>,
    pub pll_lfsr: Mmio<u32>,
    pub osc_freq_det: Mmio<u32>,
    pub osc_freq_det_stat: Mmio<u32>,
    _0x60: [Mmio<u32>; 2],
    pub plle_ss_cntl: Mmio<u32>,
    pub plle_misc1: Mmio<u32>,
    _0x70: [Mmio<u32>; 4],

    pub pllc_base: Mmio<u32>,
    pub pllc_out: Mmio<u32>,
    pub pllc_misc0: Mmio<u32>,
    pub pllc_misc1: Mmio<u32>,

    pub pllm_base: Mmio<u32>,
    pub pllm_out: Mmio<u32>,
    pub pllm_misc1: Mmio<u32>,
    pub pllm_misc2: Mmio<u32>,

    pub pllp_base: Mmio<u32>,
    pub pllp_outa: Mmio<u32>,
    pub pllp_outb: Mmio<u32>,
    pub pllp_misc: Mmio<u32>,

    pub plla_base: Mmio<u32>,
    pub plla_out: Mmio<u32>,
    pub plla_misc0: Mmio<u32>,
    pub plla_misc1: Mmio<u32>,

    pub pllu_base: Mmio<u32>,
    pub pllu_out: Mmio<u32>,
    pub pllu_misc1: Mmio<u32>,
    pub pllu_misc2: Mmio<u32>,

    pub plld_base: Mmio<u32>,
    pub plld_out: Mmio<u32>,
    pub plld_misc1: Mmio<u32>,
    pub plld_misc2: Mmio<u32>,

    pub pllx_base: Mmio<u32>,
    pub pllx_misc: Mmio<u32>,

    pub plle_base: Mmio<u32>,
    pub plle_misc: Mmio<u32>,
    pub plle_ss_cntl1: Mmio<u32>,
    pub plle_ss_cntl2: Mmio<u32>,

    pub lvl2_clk_gate_ovra: Mmio<u32>,
    pub lvl2_clk_gate_ovrb: Mmio<u32>,

    pub clk_source_i2s2: Mmio<u32>,
    pub clk_source_i2s3: Mmio<u32>,
    pub clk_source_spdif_out: Mmio<u32>,
    pub clk_source_spdif_in: Mmio<u32>,
    pub clk_source_pwm: Mmio<u32>,
    _0x114: Mmio<u32>,
    pub clk_source_spi2: Mmio<u32>,
    pub clk_source_spi3: Mmio<u32>,
    _0x120: Mmio<u32>,
    pub clk_source_i2c1: Mmio<u32>,
    pub clk_source_i2c5: Mmio<u32>,
    _0x12c: [Mmio<u32>; 2],
    pub clk_source_spi1: Mmio<u32>,
    pub clk_source_disp1: Mmio<u32>,
    pub clk_source_disp2: Mmio<u32>,
    _0x140: Mmio<u32>,
    pub clk_source_isp: Mmio<u32>,
    pub clk_source_vi: Mmio<u32>,
    _0x14c: Mmio<u32>,
    pub clk_source_sdmmc1: Mmio<u32>,
    pub clk_source_sdmmc2: Mmio<u32>,
    _0x158: [Mmio<u32>; 3],
    pub clk_source_sdmmc4: Mmio<u32>,
    _0x168: [Mmio<u32>; 4],
    pub clk_source_uarta: Mmio<u32>,
    pub clk_source_uartb: Mmio<u32>,
    pub clk_source_host1x: Mmio<u32>,
    _0x184: [Mmio<u32>; 5],
    pub clk_source_i2c2: Mmio<u32>,
    pub clk_source_emc: Mmio<u32>,
    pub clk_source_uartc: Mmio<u32>,
    _0x1a4: Mmio<u32>,
    pub clk_source_vi_sensor: Mmio<u32>,
    _0x1ac: [Mmio<u32>; 2],
    pub clk_source_spi4: Mmio<u32>,
    pub clk_source_i2c3: Mmio<u32>,
    pub clk_source_sdmmc3: Mmio<u32>,
    pub clk_source_uartd: Mmio<u32>,
    _0x1c4: [Mmio<u32>; 2],
    pub clk_source_owr: Mmio<u32>,
    _0x1d0: Mmio<u32>,
    pub clk_source_csite: Mmio<u32>,
    pub clk_source_i2s1: Mmio<u32>,
    pub clk_source_dtv: Mmio<u32>,
    _0x1e0: [Mmio<u32>; 5],
    pub clk_source_tsec: Mmio<u32>,
    _0x1f8: Mmio<u32>,

    pub clk_spare2: Mmio<u32>,
    _0x200: [Mmio<u32>; 32],

    pub clk_out_enb_x: Mmio<u32>,
    pub clk_enb_x_set: Mmio<u32>,
    pub clk_enb_x_clr: Mmio<u32>,

    pub rst_devices_x: Mmio<u32>,
    pub rst_dev_x_set: Mmio<u32>,
    pub rst_dev_x_clr: Mmio<u32>,

    pub clk_out_enb_y: Mmio<u32>,
    pub clk_enb_y_set: Mmio<u32>,
    pub clk_enb_y_clr: Mmio<u32>,

    pub rst_devices_y: Mmio<u32>,
    pub rst_dev_y_set: Mmio<u32>,
    pub rst_dev_y_clr: Mmio<u32>,

    _0x2b0: [Mmio<u32>; 17],
    pub dfll_base: Mmio<u32>,
    _0x2f8: [Mmio<u32>; 2],

    pub rst_dev_l_set: Mmio<u32>,
    pub rst_dev_l_clr: Mmio<u32>,
    pub rst_dev_h_set: Mmio<u32>,
    pub rst_dev_h_clr: Mmio<u32>,
    pub rst_dev_u_set: Mmio<u32>,
    pub rst_dev_u_clr: Mmio<u32>,

    _0x318: [Mmio<u32>; 2],

    pub clk_enb_l_set: Mmio<u32>,
    pub clk_enb_l_clr: Mmio<u32>,
    pub clk_enb_h_set: Mmio<u32>,
    pub clk_enb_h_clr: Mmio<u32>,
    pub clk_enb_u_set: Mmio<u32>,
    pub clk_enb_u_clr: Mmio<u32>,

    _0x338: Mmio<u32>,
    pub ccplex_pg_sm_ovrd: Mmio<u32>,
    pub rst_cpu_cmplx_set: Mmio<u32>,
    pub rst_cpu_cmplx_clr: Mmio<u32>,

    pub clk_cpu_cmplx_set: Mmio<u32>,
    pub clk_cpu_cmplx_clr: Mmio<u32>,

    _0x350: [Mmio<u32>; 2],
    pub rst_dev_v: Mmio<u32>,
    pub rst_dev_w: Mmio<u32>,
    pub clk_out_enb_v: Mmio<u32>,
    pub clk_out_enb_w: Mmio<u32>,
    pub cclkg_brst_pol: Mmio<u32>,
    pub super_cclkg_div: Mmio<u32>,
    pub cclklp_brst_pol: Mmio<u32>,
    pub super_cclkp_div: Mmio<u32>,
    pub clk_cpug_cmplx: Mmio<u32>,
    pub clk_cpulp_cmplx: Mmio<u32>,
    pub cpu_softrst_ctrl: Mmio<u32>,
    pub cpu_softrst_ctrl1: Mmio<u32>,
    pub cpu_softrst_ctrl2: Mmio<u32>,
    _0x38c: [Mmio<u32>; 5],
    pub lvl2_clk_gate_ovrc: Mmio<u32>,
    pub lvl2_clk_gate_ovrd: Mmio<u32>,
    _0x3a8: [Mmio<u32>; 2],

    _0x3b0: Mmio<u32>,
    pub clk_source_mselect: Mmio<u32>,
    pub clk_source_tsensor: Mmio<u32>,
    pub clk_source_i2s4: Mmio<u32>,
    pub clk_source_i2s5: Mmio<u32>,
    pub clk_source_i2c4: Mmio<u32>,
    _0x3c8: [Mmio<u32>; 2],
    pub clk_source_ahub: Mmio<u32>,
    _0x3d4: [Mmio<u32>; 4],
    pub clk_source_hda2codec_2x: Mmio<u32>,
    pub clk_source_actmon: Mmio<u32>,
    pub clk_source_extperiph1: Mmio<u32>,
    pub clk_source_extperiph2: Mmio<u32>,
    pub clk_source_extperiph3: Mmio<u32>,
    _0x3f8: Mmio<u32>,
    pub clk_source_i2c_slow: Mmio<u32>,
    pub clk_source_sys: Mmio<u32>,
    pub clk_source_ispb: Mmio<u32>,
    _0x408: [Mmio<u32>; 2],
    pub clk_source_sor1: Mmio<u32>,
    pub clk_source_sor0: Mmio<u32>,
    _0x418: [Mmio<u32>; 2],
    pub clk_source_sata_oob: Mmio<u32>,
    pub clk_source_sata: Mmio<u32>,
    pub clk_source_hda: Mmio<u32>,
    _0x42c: Mmio<u32>,

    pub rst_dev_v_set: Mmio<u32>,
    pub rst_dev_v_clr: Mmio<u32>,
    pub rst_dev_w_set: Mmio<u32>,
    pub rst_dev_w_clr: Mmio<u32>,

    pub clk_enb_v_set: Mmio<u32>,
    pub clk_enb_v_clr: Mmio<u32>,
    pub clk_enb_w_set: Mmio<u32>,
    pub clk_enb_w_clr: Mmio<u32>,

    pub rst_cpug_cmplx_set: Mmio<u32>,
    pub rst_cpug_cmplx_clr: Mmio<u32>,
    pub rst_cpulp_cmplx_set: Mmio<u32>,
    pub rst_cpulp_cmplx_clr: Mmio<u32>,
    pub clk_cpug_cmplx_set: Mmio<u32>,
    pub clk_cpug_cmplx_clr: Mmio<u32>,
    pub clk_cpulp_cmplx_set: Mmio<u32>,
    pub clk_cpulp_cmplx_clr: Mmio<u32>,
    pub cpu_cmplx_status: Mmio<u32>,
    _0x474: Mmio<u32>,
    pub intstatus: Mmio<u32>,
    pub intmask: Mmio<u32>,
    pub utmip_pll_cfg0: Mmio<u32>,
    pub utmip_pll_cfg1: Mmio<u32>,
    pub utmip_pll_cfg2: Mmio<u32>,

    pub plle_aux: Mmio<u32>,
    pub sata_pll_cfg0: Mmio<u32>,
    pub sata_pll_cfg1: Mmio<u32>,
    pub pcie_pll_cfg0: Mmio<u32>,

    pub prog_audio_dly_clk: Mmio<u32>,
    pub audio_sync_clk_i2s0: Mmio<u32>,
    pub audio_sync_clk_i2s1: Mmio<u32>,
    pub audio_sync_clk_i2s2: Mmio<u32>,
    pub audio_sync_clk_i2s3: Mmio<u32>,
    pub audio_sync_clk_i2s4: Mmio<u32>,
    pub audio_sync_clk_spdif: Mmio<u32>,

    pub plld2_base: Mmio<u32>,
    pub plld2_misc: Mmio<u32>,
    pub utmip_pll_cfg3: Mmio<u32>,
    pub pllrefe_base: Mmio<u32>,
    pub pllrefe_misc: Mmio<u32>,
    pub pllrefe_out: Mmio<u32>,
    pub cpu_finetrim_byp: Mmio<u32>,
    pub cpu_finetrim_select: Mmio<u32>,
    pub cpu_finetrim_dr: Mmio<u32>,
    pub cpu_finetrim_df: Mmio<u32>,
    pub cpu_finetrim_f: Mmio<u32>,
    pub cpu_finetrim_r: Mmio<u32>,
    pub pllc2_base: Mmio<u32>,
    pub pllc2_misc0: Mmio<u32>,
    pub pllc2_misc1: Mmio<u32>,
    pub pllc2_misc2: Mmio<u32>,
    pub pllc2_misc3: Mmio<u32>,
    pub pllc3_base: Mmio<u32>,
    pub pllc3_misc0: Mmio<u32>,
    pub pllc3_misc1: Mmio<u32>,
    pub pllc3_misc2: Mmio<u32>,
    pub pllc3_misc3: Mmio<u32>,
    pub pllx_misc1: Mmio<u32>,
    pub pllx_misc2: Mmio<u32>,
    pub pllx_misc3: Mmio<u32>,
    pub xusbio_pll_cfg0: Mmio<u32>,
    pub xusbio_pll_cfg1: Mmio<u32>,
    pub plle_aux1: Mmio<u32>,
    pub pllp_reshift: Mmio<u32>,
    pub utmipll_hw_pwrdn_cfg0: Mmio<u32>,
    pub pllu_hw_pwrdn_cfg0: Mmio<u32>,
    pub xusb_pll_cfg0: Mmio<u32>,
    _0x538: Mmio<u32>,
    pub clk_cpu_misc: Mmio<u32>,
    pub clk_cpug_misc: Mmio<u32>,
    pub clk_cpulp_misc: Mmio<u32>,
    pub pllx_hw_ctrl_cfg: Mmio<u32>,
    pub pllx_sw_ramp_cfg: Mmio<u32>,
    pub pllx_hw_ctrl_status: Mmio<u32>,
    pub lvl2_clk_gate_ovre: Mmio<u32>,
    pub super_gr3d_clk_div: Mmio<u32>,
    pub spare_reg0: Mmio<u32>,
    pub audio_sync_clk_dmic1: Mmio<u32>,
    pub audio_sync_clk_dmic2: Mmio<u32>,

    _0x568: [Mmio<u32>; 2],
    pub plld2_ss_cfg: Mmio<u32>,
    pub plld2_ss_ctrl1: Mmio<u32>,
    pub plld2_ss_ctrl2: Mmio<u32>,
    _0x57c: [Mmio<u32>; 5],

    pub plldp_base: Mmio<u32>,
    pub plldp_misc: Mmio<u32>,
    pub plldp_ss_cfg: Mmio<u32>,
    pub plldp_ss_ctrl1: Mmio<u32>,
    pub plldp_ss_ctrl2: Mmio<u32>,
    pub pllc4_base: Mmio<u32>,
    pub pllc4_misc: Mmio<u32>,
    _0x5ac: [Mmio<u32>; 6],
    pub clk_spare0: Mmio<u32>,
    pub clk_spare1: Mmio<u32>,
    pub gpu_isob_ctrl: Mmio<u32>,
    pub pllc_misc2: Mmio<u32>,
    pub pllc_misc3: Mmio<u32>,
    pub plla_misc2: Mmio<u32>,
    _0x5dc: [Mmio<u32>; 2],
    pub pllc4_out: Mmio<u32>,
    pub pllmb_base: Mmio<u32>,
    pub pllmb_misc1: Mmio<u32>,
    pub pllx_misc4: Mmio<u32>,
    pub pllx_misc5: Mmio<u32>,
    _0x5f8: [Mmio<u32>; 2],

    pub clk_source_xusb_core_host: Mmio<u32>,
    pub clk_source_xusb_falcon: Mmio<u32>,
    pub clk_source_xusb_fs: Mmio<u32>,
    pub clk_source_xusb_core_dev: Mmio<u32>,
    pub clk_source_xusb_ss: Mmio<u32>,
    pub clk_source_cilab: Mmio<u32>,
    pub clk_source_cilcd: Mmio<u32>,
    pub clk_source_cilef: Mmio<u32>,
    pub clk_source_dsia_lp: Mmio<u32>,
    pub clk_source_dsib_lp: Mmio<u32>,
    pub clk_source_entropy: Mmio<u32>,
    pub clk_source_dvfs_ref: Mmio<u32>,
    pub clk_source_dvfs_soc: Mmio<u32>,
    _0x634: [Mmio<u32>; 3],
    pub clk_source_emc_latency: Mmio<u32>,
    pub clk_source_soc_therm: Mmio<u32>,
    _0x648: Mmio<u32>,
    pub clk_source_dmic1: Mmio<u32>,
    pub clk_source_dmic2: Mmio<u32>,
    _0x654: Mmio<u32>,
    pub clk_source_vi_sensor2: Mmio<u32>,
    pub clk_source_i2c6: Mmio<u32>,
    pub clk_source_mipibif: Mmio<u32>,
    pub clk_source_emc_dll: Mmio<u32>,
    _0x668: Mmio<u32>,
    pub clk_source_uart_fst_mipi_cal: Mmio<u32>,
    _0x670: [Mmio<u32>; 2],
    pub clk_source_vic: Mmio<u32>,

    pub pllp_outc: Mmio<u32>,
    pub pllp_misc1: Mmio<u32>,
    _0x684: [Mmio<u32>; 2],
    pub emc_div_clk_shaper_ctrl: Mmio<u32>,
    pub emc_pllc_shaper_ctrl: Mmio<u32>,

    pub clk_source_sdmmc_legacy_tm: Mmio<u32>,
    pub clk_source_nvdec: Mmio<u32>,
    pub clk_source_nvjpg: Mmio<u32>,
    pub clk_source_nvenc: Mmio<u32>,

    pub plla1_base: Mmio<u32>,
    pub plla1_misc0: Mmio<u32>,
    pub plla1_misc1: Mmio<u32>,
    pub plla1_misc2: Mmio<u32>,
    pub plla1_misc3: Mmio<u32>,
    pub audio_sync_clk_dmic3: Mmio<u32>,

    pub clk_source_dmic3: Mmio<u32>,
    pub clk_source_ape: Mmio<u32>,
    pub clk_source_qspi: Mmio<u32>,
    pub clk_source_vi_i2c: Mmio<u32>,
    pub clk_source_usb2_hsic_trk: Mmio<u32>,
    pub clk_source_pex_sata_usb_rx_byp: Mmio<u32>,
    pub clk_source_maud: Mmio<u32>,
    pub clk_source_tsecb: Mmio<u32>,

    pub clk_cpug_misc1: Mmio<u32>,
    pub aclk_burst_policy: Mmio<u32>,
    pub super_aclk_divider: Mmio<u32>,

    pub nvenc_super_clk_divider: Mmio<u32>,
    pub vi_super_clk_divider: Mmio<u32>,
    pub vic_super_clk_divider: Mmio<u32>,
    pub nvdec_super_clk_divider: Mmio<u32>,
    pub isp_super_clk_divider: Mmio<u32>,
    pub ispb_super_clk_divider: Mmio<u32>,
    pub nvjpg_super_clk_divider: Mmio<u32>,
    pub se_super_clk_divider: Mmio<u32>,
    pub tsec_super_clk_divider: Mmio<u32>,
    pub tsecb_super_clk_divider: Mmio<u32>,

    pub clk_source_uartape: Mmio<u32>,
    pub clk_cpug_misc2: Mmio<u32>,
    pub clk_source_dbgapb: Mmio<u32>,
    pub clk_ccplex_cc4_ret_clk_enb: Mmio<u32>,
    pub actmon_cpu_clk: Mmio<u32>,
    pub clk_source_emc_safe: Mmio<u32>,
    pub sdmmc2_pllc4_out0_shaper_ctrl: Mmio<u32>,
    pub sdmmc2_pllc4_out1_shaper_ctrl: Mmio<u32>,
    pub sdmmc2_pllc4_out2_shaper_ctrl: Mmio<u32>,
    pub sdmmc2_div_clk_shaper_ctrl: Mmio<u32>,
    pub sdmmc4_pllc4_out0_shaper_ctrl: Mmio<u32>,
    pub sdmmc4_pllc4_out1_shaper_ctrl: Mmio<u32>,
    pub sdmmc4_pllc4_out2_shaper_ctrl: Mmio<u32>,
    pub sdmmc4_div_clk_shaper_ctrl: Mmio<u32>,
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
