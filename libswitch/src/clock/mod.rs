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
//! - The [`Car`] struct can be used to access the CAR [`Registers`]
//! by creating a reference to a object. It is only exposed within
//! the crate.
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
//! [`Registers`]: struct.Registers.html
//! [`Clock`]: struct.Clock.html
//! [`Clock::enable`]: struct.Clock.html#method.enable
//! [`Clock::disable`]: struct.Clock.html#method.disable
//! [`Clock::is_enabled`]: struct.Clock.html#method.is_enabled

use core::ops::Deref;

use register::mmio::ReadWrite;

/// Base address for clock registers.
const CLOCK_BASE: u32 = 0x6000_6000;

/// Representation of the CAR registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Registers {
    pub rst_src: ReadWrite<u32>,

    pub rst_dev_l: ReadWrite<u32>,
    pub rst_dev_h: ReadWrite<u32>,
    pub rst_dev_u: ReadWrite<u32>,

    pub clk_out_enb_l: ReadWrite<u32>,
    pub clk_out_enb_h: ReadWrite<u32>,
    pub clk_out_enb_u: ReadWrite<u32>,

    _0x1C: ReadWrite<u32>,
    pub cclk_brst_pol: ReadWrite<u32>,
    pub super_cclk_div: ReadWrite<u32>,
    pub sclk_brst_pol: ReadWrite<u32>,
    pub super_sclk_div: ReadWrite<u32>,
    pub clk_sys_rate: ReadWrite<u32>,
    pub prog_dly_clk: ReadWrite<u32>,
    pub aud_sync_clk_rate: ReadWrite<u32>,
    _0x3C: ReadWrite<u32>,
    pub cop_clk_skip_plcy: ReadWrite<u32>,
    pub clk_mask_arm: ReadWrite<u32>,
    pub misc_clk_enb: ReadWrite<u32>,
    pub clk_cpu_cmplx: ReadWrite<u32>,
    pub osc_ctrl: ReadWrite<u32>,
    pub pll_lfsr: ReadWrite<u32>,
    pub osc_freq_det: ReadWrite<u32>,
    pub osc_freq_det_stat: ReadWrite<u32>,
    _0x60: [ReadWrite<u32>; 2],
    pub plle_ss_cntl: ReadWrite<u32>,
    pub plle_misc1: ReadWrite<u32>,
    _0x70: [ReadWrite<u32>; 4],

    pub pllc_base: ReadWrite<u32>,
    pub pllc_out: ReadWrite<u32>,
    pub pllc_misc0: ReadWrite<u32>,
    pub pllc_misc1: ReadWrite<u32>,

    pub pllm_base: ReadWrite<u32>,
    pub pllm_out: ReadWrite<u32>,
    pub pllm_misc1: ReadWrite<u32>,
    pub pllm_misc2: ReadWrite<u32>,

    pub pllp_base: ReadWrite<u32>,
    pub pllp_outa: ReadWrite<u32>,
    pub pllp_outb: ReadWrite<u32>,
    pub pllp_misc: ReadWrite<u32>,

    pub plla_base: ReadWrite<u32>,
    pub plla_out: ReadWrite<u32>,
    pub plla_misc0: ReadWrite<u32>,
    pub plla_misc1: ReadWrite<u32>,

    pub pllu_base: ReadWrite<u32>,
    pub pllu_out: ReadWrite<u32>,
    pub pllu_misc1: ReadWrite<u32>,
    pub pllu_misc2: ReadWrite<u32>,

    pub plld_base: ReadWrite<u32>,
    pub plld_out: ReadWrite<u32>,
    pub plld_misc1: ReadWrite<u32>,
    pub plld_misc2: ReadWrite<u32>,

    pub pllx_base: ReadWrite<u32>,
    pub pllx_misc: ReadWrite<u32>,

    pub plle_base: ReadWrite<u32>,
    pub plle_misc: ReadWrite<u32>,
    pub plle_ss_cntl1: ReadWrite<u32>,
    pub plle_ss_cntl2: ReadWrite<u32>,

    pub lvl2_clk_gate_ovra: ReadWrite<u32>,
    pub lvl2_clk_gate_ovrb: ReadWrite<u32>,

    pub clk_source_i2s2: ReadWrite<u32>,
    pub clk_source_i2s3: ReadWrite<u32>,
    pub clk_source_spdif_out: ReadWrite<u32>,
    pub clk_source_spdif_in: ReadWrite<u32>,
    pub clk_source_pwm: ReadWrite<u32>,
    _0x114: ReadWrite<u32>,
    pub clk_source_spi2: ReadWrite<u32>,
    pub clk_source_spi3: ReadWrite<u32>,
    _0x120: ReadWrite<u32>,
    pub clk_source_i2c1: ReadWrite<u32>,
    pub clk_source_i2c5: ReadWrite<u32>,
    _0x12c: [ReadWrite<u32>; 2],
    pub clk_source_spi1: ReadWrite<u32>,
    pub clk_source_disp1: ReadWrite<u32>,
    pub clk_source_disp2: ReadWrite<u32>,
    _0x140: ReadWrite<u32>,
    pub clk_source_isp: ReadWrite<u32>,
    pub clk_source_vi: ReadWrite<u32>,
    _0x14c: ReadWrite<u32>,
    pub clk_source_sdmmc1: ReadWrite<u32>,
    pub clk_source_sdmmc2: ReadWrite<u32>,
    _0x158: [ReadWrite<u32>; 3],
    pub clk_source_sdmmc4: ReadWrite<u32>,
    _0x168: [ReadWrite<u32>; 4],
    pub clk_source_uarta: ReadWrite<u32>,
    pub clk_source_uartb: ReadWrite<u32>,
    pub clk_source_host1x: ReadWrite<u32>,
    _0x184: [ReadWrite<u32>; 5],
    pub clk_source_i2c2: ReadWrite<u32>,
    pub clk_source_emc: ReadWrite<u32>,
    pub clk_source_uartc: ReadWrite<u32>,
    _0x1a4: ReadWrite<u32>,
    pub clk_source_vi_sensor: ReadWrite<u32>,
    _0x1ac: [ReadWrite<u32>; 2],
    pub clk_source_spi4: ReadWrite<u32>,
    pub clk_source_i2c3: ReadWrite<u32>,
    pub clk_source_sdmmc3: ReadWrite<u32>,
    pub clk_source_uartd: ReadWrite<u32>,
    _0x1c4: [ReadWrite<u32>; 2],
    pub clk_source_owr: ReadWrite<u32>,
    _0x1d0: ReadWrite<u32>,
    pub clk_source_csite: ReadWrite<u32>,
    pub clk_source_i2s1: ReadWrite<u32>,
    pub clk_source_dtv: ReadWrite<u32>,
    _0x1e0: [ReadWrite<u32>; 5],
    pub clk_source_tsec: ReadWrite<u32>,
    _0x1f8: ReadWrite<u32>,

    pub clk_spare2: ReadWrite<u32>,
    _0x200: [ReadWrite<u32>; 32],

    pub clk_out_enb_x: ReadWrite<u32>,
    pub clk_enb_x_set: ReadWrite<u32>,
    pub clk_enb_x_clr: ReadWrite<u32>,

    pub rst_devices_x: ReadWrite<u32>,
    pub rst_dev_x_set: ReadWrite<u32>,
    pub rst_dev_x_clr: ReadWrite<u32>,

    pub clk_out_enb_y: ReadWrite<u32>,
    pub clk_enb_y_set: ReadWrite<u32>,
    pub clk_enb_y_clr: ReadWrite<u32>,

    pub rst_devices_y: ReadWrite<u32>,
    pub rst_dev_y_set: ReadWrite<u32>,
    pub rst_dev_y_clr: ReadWrite<u32>,

    _0x2b0: [ReadWrite<u32>; 17],
    pub dfll_base: ReadWrite<u32>,
    _0x2f8: [ReadWrite<u32>; 2],

    pub rst_dev_l_set: ReadWrite<u32>,
    pub rst_dev_l_clr: ReadWrite<u32>,
    pub rst_dev_h_set: ReadWrite<u32>,
    pub rst_dev_h_clr: ReadWrite<u32>,
    pub rst_dev_u_set: ReadWrite<u32>,
    pub rst_dev_u_clr: ReadWrite<u32>,

    _0x318: [ReadWrite<u32>; 2],

    pub clk_enb_l_set: ReadWrite<u32>,
    pub clk_enb_l_clr: ReadWrite<u32>,
    pub clk_enb_h_set: ReadWrite<u32>,
    pub clk_enb_h_clr: ReadWrite<u32>,
    pub clk_enb_u_set: ReadWrite<u32>,
    pub clk_enb_u_clr: ReadWrite<u32>,

    _0x338: ReadWrite<u32>,
    pub ccplex_pg_sm_ovrd: ReadWrite<u32>,
    pub rst_cpu_cmplx_set: ReadWrite<u32>,
    pub rst_cpu_cmplx_clr: ReadWrite<u32>,

    pub clk_cpu_cmplx_set: ReadWrite<u32>,
    pub clk_cpu_cmplx_clr: ReadWrite<u32>,

    _0x350: [ReadWrite<u32>; 2],
    pub rst_dev_v: ReadWrite<u32>,
    pub rst_dev_w: ReadWrite<u32>,
    pub clk_out_enb_v: ReadWrite<u32>,
    pub clk_out_enb_w: ReadWrite<u32>,
    pub cclkg_brst_pol: ReadWrite<u32>,
    pub super_cclkg_div: ReadWrite<u32>,
    pub cclklp_brst_pol: ReadWrite<u32>,
    pub super_cclkp_div: ReadWrite<u32>,
    pub clk_cpug_cmplx: ReadWrite<u32>,
    pub clk_cpulp_cmplx: ReadWrite<u32>,
    pub cpu_softrst_ctrl: ReadWrite<u32>,
    pub cpu_softrst_ctrl1: ReadWrite<u32>,
    pub cpu_softrst_ctrl2: ReadWrite<u32>,
    _0x38c: [ReadWrite<u32>; 5],
    pub lvl2_clk_gate_ovrc: ReadWrite<u32>,
    pub lvl2_clk_gate_ovrd: ReadWrite<u32>,
    _0x3a8: [ReadWrite<u32>; 2],

    _0x3b0: ReadWrite<u32>,
    pub clk_source_mselect: ReadWrite<u32>,
    pub clk_source_tsensor: ReadWrite<u32>,
    pub clk_source_i2s4: ReadWrite<u32>,
    pub clk_source_i2s5: ReadWrite<u32>,
    pub clk_source_i2c4: ReadWrite<u32>,
    _0x3c8: [ReadWrite<u32>; 2],
    pub clk_source_ahub: ReadWrite<u32>,
    _0x3d4: [ReadWrite<u32>; 4],
    pub clk_source_hda2codec_2x: ReadWrite<u32>,
    pub clk_source_actmon: ReadWrite<u32>,
    pub clk_source_extperiph1: ReadWrite<u32>,
    pub clk_source_extperiph2: ReadWrite<u32>,
    pub clk_source_extperiph3: ReadWrite<u32>,
    _0x3f8: ReadWrite<u32>,
    pub clk_source_i2c_slow: ReadWrite<u32>,
    pub clk_source_sys: ReadWrite<u32>,
    pub clk_source_ispb: ReadWrite<u32>,
    _0x408: [ReadWrite<u32>; 2],
    pub clk_source_sor1: ReadWrite<u32>,
    pub clk_source_sor0: ReadWrite<u32>,
    _0x418: [ReadWrite<u32>; 2],
    pub clk_source_sata_oob: ReadWrite<u32>,
    pub clk_source_sata: ReadWrite<u32>,
    pub clk_source_hda: ReadWrite<u32>,
    _0x42c: ReadWrite<u32>,

    pub rst_dev_v_set: ReadWrite<u32>,
    pub rst_dev_v_clr: ReadWrite<u32>,
    pub rst_dev_w_set: ReadWrite<u32>,
    pub rst_dev_w_clr: ReadWrite<u32>,

    pub clk_enb_v_set: ReadWrite<u32>,
    pub clk_enb_v_clr: ReadWrite<u32>,
    pub clk_enb_w_set: ReadWrite<u32>,
    pub clk_enb_w_clr: ReadWrite<u32>,

    pub rst_cpug_cmplx_set: ReadWrite<u32>,
    pub rst_cpug_cmplx_clr: ReadWrite<u32>,
    pub rst_cpulp_cmplx_set: ReadWrite<u32>,
    pub rst_cpulp_cmplx_clr: ReadWrite<u32>,
    pub clk_cpug_cmplx_set: ReadWrite<u32>,
    pub clk_cpug_cmplx_clr: ReadWrite<u32>,
    pub clk_cpulp_cmplx_set: ReadWrite<u32>,
    pub clk_cpulp_cmplx_clr: ReadWrite<u32>,
    pub cpu_cmplx_status: ReadWrite<u32>,
    _0x474: ReadWrite<u32>,
    pub intstatus: ReadWrite<u32>,
    pub intmask: ReadWrite<u32>,
    pub utmip_pll_cfg0: ReadWrite<u32>,
    pub utmip_pll_cfg1: ReadWrite<u32>,
    pub utmip_pll_cfg2: ReadWrite<u32>,

    pub plle_aux: ReadWrite<u32>,
    pub sata_pll_cfg0: ReadWrite<u32>,
    pub sata_pll_cfg1: ReadWrite<u32>,
    pub pcie_pll_cfg0: ReadWrite<u32>,

    pub prog_audio_dly_clk: ReadWrite<u32>,
    pub audio_sync_clk_i2s0: ReadWrite<u32>,
    pub audio_sync_clk_i2s1: ReadWrite<u32>,
    pub audio_sync_clk_i2s2: ReadWrite<u32>,
    pub audio_sync_clk_i2s3: ReadWrite<u32>,
    pub audio_sync_clk_i2s4: ReadWrite<u32>,
    pub audio_sync_clk_spdif: ReadWrite<u32>,

    pub plld2_base: ReadWrite<u32>,
    pub plld2_misc: ReadWrite<u32>,
    pub utmip_pll_cfg3: ReadWrite<u32>,
    pub pllrefe_base: ReadWrite<u32>,
    pub pllrefe_misc: ReadWrite<u32>,
    pub pllrefe_out: ReadWrite<u32>,
    pub cpu_finetrim_byp: ReadWrite<u32>,
    pub cpu_finetrim_select: ReadWrite<u32>,
    pub cpu_finetrim_dr: ReadWrite<u32>,
    pub cpu_finetrim_df: ReadWrite<u32>,
    pub cpu_finetrim_f: ReadWrite<u32>,
    pub cpu_finetrim_r: ReadWrite<u32>,
    pub pllc2_base: ReadWrite<u32>,
    pub pllc2_misc0: ReadWrite<u32>,
    pub pllc2_misc1: ReadWrite<u32>,
    pub pllc2_misc2: ReadWrite<u32>,
    pub pllc2_misc3: ReadWrite<u32>,
    pub pllc3_base: ReadWrite<u32>,
    pub pllc3_misc0: ReadWrite<u32>,
    pub pllc3_misc1: ReadWrite<u32>,
    pub pllc3_misc2: ReadWrite<u32>,
    pub pllc3_misc3: ReadWrite<u32>,
    pub pllx_misc1: ReadWrite<u32>,
    pub pllx_misc2: ReadWrite<u32>,
    pub pllx_misc3: ReadWrite<u32>,
    pub xusbio_pll_cfg0: ReadWrite<u32>,
    pub xusbio_pll_cfg1: ReadWrite<u32>,
    pub plle_aux1: ReadWrite<u32>,
    pub pllp_reshift: ReadWrite<u32>,
    pub utmipll_hw_pwrdn_cfg0: ReadWrite<u32>,
    pub pllu_hw_pwrdn_cfg0: ReadWrite<u32>,
    pub xusb_pll_cfg0: ReadWrite<u32>,
    _0x538: ReadWrite<u32>,
    pub clk_cpu_misc: ReadWrite<u32>,
    pub clk_cpug_misc: ReadWrite<u32>,
    pub clk_cpulp_misc: ReadWrite<u32>,
    pub pllx_hw_ctrl_cfg: ReadWrite<u32>,
    pub pllx_sw_ramp_cfg: ReadWrite<u32>,
    pub pllx_hw_ctrl_status: ReadWrite<u32>,
    pub lvl2_clk_gate_ovre: ReadWrite<u32>,
    pub super_gr3d_clk_div: ReadWrite<u32>,
    pub spare_reg0: ReadWrite<u32>,
    pub audio_sync_clk_dmic1: ReadWrite<u32>,
    pub audio_sync_clk_dmic2: ReadWrite<u32>,

    _0x568: [ReadWrite<u32>; 2],
    pub plld2_ss_cfg: ReadWrite<u32>,
    pub plld2_ss_ctrl1: ReadWrite<u32>,
    pub plld2_ss_ctrl2: ReadWrite<u32>,
    _0x57c: [ReadWrite<u32>; 5],

    pub plldp_base: ReadWrite<u32>,
    pub plldp_misc: ReadWrite<u32>,
    pub plldp_ss_cfg: ReadWrite<u32>,
    pub plldp_ss_ctrl1: ReadWrite<u32>,
    pub plldp_ss_ctrl2: ReadWrite<u32>,
    pub pllc4_base: ReadWrite<u32>,
    pub pllc4_misc: ReadWrite<u32>,
    _0x5ac: [ReadWrite<u32>; 6],
    pub clk_spare0: ReadWrite<u32>,
    pub clk_spare1: ReadWrite<u32>,
    pub gpu_isob_ctrl: ReadWrite<u32>,
    pub pllc_misc2: ReadWrite<u32>,
    pub pllc_misc3: ReadWrite<u32>,
    pub plla_misc2: ReadWrite<u32>,
    _0x5dc: [ReadWrite<u32>; 2],
    pub pllc4_out: ReadWrite<u32>,
    pub pllmb_base: ReadWrite<u32>,
    pub pllmb_misc1: ReadWrite<u32>,
    pub pllx_misc4: ReadWrite<u32>,
    pub pllx_misc5: ReadWrite<u32>,
    _0x5f8: [ReadWrite<u32>; 2],

    pub clk_source_xusb_core_host: ReadWrite<u32>,
    pub clk_source_xusb_falcon: ReadWrite<u32>,
    pub clk_source_xusb_fs: ReadWrite<u32>,
    pub clk_source_xusb_core_dev: ReadWrite<u32>,
    pub clk_source_xusb_ss: ReadWrite<u32>,
    pub clk_source_cilab: ReadWrite<u32>,
    pub clk_source_cilcd: ReadWrite<u32>,
    pub clk_source_cilef: ReadWrite<u32>,
    pub clk_source_dsia_lp: ReadWrite<u32>,
    pub clk_source_dsib_lp: ReadWrite<u32>,
    pub clk_source_entropy: ReadWrite<u32>,
    pub clk_source_dvfs_ref: ReadWrite<u32>,
    pub clk_source_dvfs_soc: ReadWrite<u32>,
    _0x634: [ReadWrite<u32>; 3],
    pub clk_source_emc_latency: ReadWrite<u32>,
    pub clk_source_soc_therm: ReadWrite<u32>,
    _0x648: ReadWrite<u32>,
    pub clk_source_dmic1: ReadWrite<u32>,
    pub clk_source_dmic2: ReadWrite<u32>,
    _0x654: ReadWrite<u32>,
    pub clk_source_vi_sensor2: ReadWrite<u32>,
    pub clk_source_i2c6: ReadWrite<u32>,
    pub clk_source_mipibif: ReadWrite<u32>,
    pub clk_source_emc_dll: ReadWrite<u32>,
    _0x668: ReadWrite<u32>,
    pub clk_source_uart_fst_mipi_cal: ReadWrite<u32>,
    _0x670: [ReadWrite<u32>; 2],
    pub clk_source_vic: ReadWrite<u32>,

    pub pllp_outc: ReadWrite<u32>,
    pub pllp_misc1: ReadWrite<u32>,
    _0x684: [ReadWrite<u32>; 2],
    pub emc_div_clk_shaper_ctrl: ReadWrite<u32>,
    pub emc_pllc_shaper_ctrl: ReadWrite<u32>,

    pub clk_source_sdmmc_legacy_tm: ReadWrite<u32>,
    pub clk_source_nvdec: ReadWrite<u32>,
    pub clk_source_nvjpg: ReadWrite<u32>,
    pub clk_source_nvenc: ReadWrite<u32>,

    pub plla1_base: ReadWrite<u32>,
    pub plla1_misc0: ReadWrite<u32>,
    pub plla1_misc1: ReadWrite<u32>,
    pub plla1_misc2: ReadWrite<u32>,
    pub plla1_misc3: ReadWrite<u32>,
    pub audio_sync_clk_dmic3: ReadWrite<u32>,

    pub clk_source_dmic3: ReadWrite<u32>,
    pub clk_source_ape: ReadWrite<u32>,
    pub clk_source_qspi: ReadWrite<u32>,
    pub clk_source_vi_i2c: ReadWrite<u32>,
    pub clk_source_usb2_hsic_trk: ReadWrite<u32>,
    pub clk_source_pex_sata_usb_rx_byp: ReadWrite<u32>,
    pub clk_source_maud: ReadWrite<u32>,
    pub clk_source_tsecb: ReadWrite<u32>,

    pub clk_cpug_misc1: ReadWrite<u32>,
    pub aclk_burst_policy: ReadWrite<u32>,
    pub super_aclk_divider: ReadWrite<u32>,

    pub nvenc_super_clk_divider: ReadWrite<u32>,
    pub vi_super_clk_divider: ReadWrite<u32>,
    pub vic_super_clk_divider: ReadWrite<u32>,
    pub nvdec_super_clk_divider: ReadWrite<u32>,
    pub isp_super_clk_divider: ReadWrite<u32>,
    pub ispb_super_clk_divider: ReadWrite<u32>,
    pub nvjpg_super_clk_divider: ReadWrite<u32>,
    pub se_super_clk_divider: ReadWrite<u32>,
    pub tsec_super_clk_divider: ReadWrite<u32>,
    pub tsecb_super_clk_divider: ReadWrite<u32>,

    pub clk_source_uartape: ReadWrite<u32>,
    pub clk_cpug_misc2: ReadWrite<u32>,
    pub clk_source_dbgapb: ReadWrite<u32>,
    pub clk_ccplex_cc4_ret_clk_enb: ReadWrite<u32>,
    pub actmon_cpu_clk: ReadWrite<u32>,
    pub clk_source_emc_safe: ReadWrite<u32>,
    pub sdmmc2_pllc4_out0_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc2_pllc4_out1_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc2_pllc4_out2_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc2_div_clk_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc4_pllc4_out0_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc4_pllc4_out1_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc4_pllc4_out2_shaper_ctrl: ReadWrite<u32>,
    pub sdmmc4_div_clk_shaper_ctrl: ReadWrite<u32>,
}

impl Registers {
    /// Factory method to create a pointer to the CAR registers.
    #[inline]
    pub const fn get() -> *const Self {
        CLOCK_BASE as *const _
    }
}

/// Representation of the CAR.
pub struct Car;

impl Car {
    /// Creates a new Car object.
    pub fn new() -> Self {
        Car
    }
}

impl Deref for Car {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Registers::get() }
    }
}

/// Representation of a device clock.
#[derive(Clone, Copy, Debug, PartialEq)]
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
        let reset_reg = unsafe { &(*((CLOCK_BASE + self.reset) as *const ReadWrite<u32>)) };

        let current_value = reset_reg.get();
        let mask = (1 << self.index & 0x1F) as u32;

        let new_value = if set_reset {
            current_value | mask
        } else {
            current_value & !mask
        };

        reset_reg.set(new_value);
    }

    /// Sets whether the clock should be enabled or disabled.
    fn set_enable(&self, set_enable: bool) {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };

        let current_value = enable_reg.get();
        let mask = (1 << (self.index & 0x1F)) as u32;

        let new_value = if set_enable {
            current_value | mask
        } else {
            current_value & !mask
        };

        enable_reg.set(new_value);
    }

    /// Enables the clock.
    pub fn enable(&self) {
        // Put clock into reset.
        self.set_reset(true);

        // Disable clock.
        self.disable();

        // Setup clock source if needed.
        if self.source != 0 {
            let source_reg = unsafe { &(*((CLOCK_BASE + self.source) as *const ReadWrite<u32>)) };
            source_reg.set(self.clock_divisor | (self.clock_source << 29));
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

    /// Indicates whether the clock is enabled or not.
    pub fn is_enabled(&self) -> bool {
        let enable_reg = unsafe { &(*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>)) };
        let mask = (1 << (self.index & 0x1F)) as u32;

        (enable_reg.get() & mask) == mask
    }
}
