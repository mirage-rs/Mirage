//! Synchronous Dynamic Random Access Memory initialization code for Tegra210 devices.
//!
//! # Implementation
//!
//! - [`get_parameters`] is to be used for retrieving SDRAM configuration parameters.
//!
//! - The [`init`] function initializes the SDRAM and calls  [`config_sdram`] which
//! does the actual dirty job of writing SDRAM parameters to the respective registers
//! to configure it.
//!
//! [`get_parameters`]: fn.get_parameters.html
//! [`init`]: fn.init.html
//! [`config_sdram`]: fn.config_sdram.html

use core::{mem::transmute_copy, ptr::write_volatile};

use register::mmio::ReadWrite;

use self::{config::DRAM_CONFIG, params::Parameters};
use crate::{
    clock::Car,
    fuse::read_reserved_odm,
    i2c::{I2c, MAX77620_PWR_I2C_ADDR},
    pmc::Pmc,
    timer::{get_microseconds, usleep},
};

mod config;
mod params;

/// Retrieves the SDRAM ID.
#[inline]
fn get_sdram_id() -> usize {
    ((read_reserved_odm(4) & 0x38) >> 3) as usize
}

/// Configures the SDRAM.
fn config_sdram(car: &Car, pmc: &Pmc, params: &mut Parameters) {
    pmc.io_dpd3_req
        .set((((4 * params.emc_pmc_scratch1 >> 2) + 0x8000_0000) ^ 0xFFFF) & 0xC000_FFFF);
    usleep(params.pmc_io_dpd3_req_wait);
    let req = (4 * params.emc_pmc_scratch2 >> 2) + 0x8000_0000;
    pmc.io_dpd4_req.set((req >> 16 << 16) ^ 0x3FFF_0000);
    usleep(params.pmc_io_dpd4_req_wait);
    pmc.io_dpd4_req.set((req ^ 0xFFFF) & 0xC000_FFFF);
    usleep(params.pmc_io_dpd4_req_wait);
    pmc.weak_bias.set(0);
    usleep(1);

    car.pllm_misc1.set(params.pllm_setup_control);
    car.pllm_misc2.set(0);
    car.pllm_base.set(
        (params.pllm_feedback_divider << 8)
            | params.pllm_input_divider
            | 0x4000_0000
            | ((params.pllm_post_divider & 0xFFFF) << 20),
    );

    let mut timeout = false;
    let wait_end = get_microseconds() + 300;

    while car.pllm_base.get() & 0x8000000 == 0 && !timeout {
        if get_microseconds() >= wait_end {
            timeout = true;
        }
    }

    if !timeout {
        usleep(10);
    }

    car.clk_source_emc.set(
        ((params.mc_emem_arb_misc0 >> 11) & 0x10000) | (params.emc_clock_source & 0xFFFE_FFFF),
    );

    if params.emc_clock_source_dll != 0 {
        car.clk_source_emc_dll.set(params.emc_clock_source_dll);
    }

    if params.clear_clock2_mc1 != 0 {
        car.clk_enb_w_clr.set(0x4000_0000);
    }

    car.clk_enb_h_set.set(0x2000001);
    car.clk_enb_x_set.set(0x4000);
    car.rst_dev_h_clr.set(0x2000001);

    unsafe {
        (*((0x7001B000 + 3124) as *const ReadWrite<u32>)).set(params.emc_pmacro_vttgen_ctrl0);
        (*((0x7001B000 + 3128) as *const ReadWrite<u32>)).set(params.emc_pmacro_vttgen_ctrl1);
        (*((0x7001B000 + 3312) as *const ReadWrite<u32>)).set(params.emc_pmacro_vttgen_ctrl2);
        (*((0x7001B000 + 40) as *const ReadWrite<u32>)).set(1);

        usleep(1);

        (*((0x7001B000 + 8) as *const ReadWrite<u32>))
            .set((params.emc_dbg_write_mux << 1) | params.emc_dbg);

        if params.emc_bct_spare2 != 0 {
            write_volatile(
                &mut params.emc_bct_spare2 as *mut u32,
                params.emc_bct_spare3,
            );
        }

        (*((0x7001B000 + 1412) as *const ReadWrite<u32>)).set(params.emc_fbio_cfg7);
        (*((0x7001B000 + 896) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd0_0);
        (*((0x7001B000 + 900) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd0_1);
        (*((0x7001B000 + 904) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd0_2);
        (*((0x7001B000 + 908) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd1_0);
        (*((0x7001B000 + 912) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd1_1);
        (*((0x7001B000 + 916) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd1_2);
        (*((0x7001B000 + 920) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd2_0);
        (*((0x7001B000 + 924) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd2_1);
        (*((0x7001B000 + 928) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd2_2);
        (*((0x7001B000 + 932) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd3_0);
        (*((0x7001B000 + 936) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd3_1);
        (*((0x7001B000 + 940) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_cmd3_2);
        (*((0x7001B000 + 944) as *const ReadWrite<u32>)).set(params.emc_cmd_mapping_byte);
        (*((0x7001B000 + 3200) as *const ReadWrite<u32>)).set(params.emc_pmacro_brick_mapping0);
        (*((0x7001B000 + 3204) as *const ReadWrite<u32>)).set(params.emc_pmacro_brick_mapping1);
        (*((0x7001B000 + 3208) as *const ReadWrite<u32>)).set(params.emc_pmacro_brick_mapping2);
        (*((0x7001B000 + 816) as *const ReadWrite<u32>))
            .set((params.emc_pmacro_brick_ctrl_rfu1 & 0x1120112) | 0x1EED_1EED);
        (*((0x7001B000 + 1520) as *const ReadWrite<u32>)).set(params.emc_config_sample_delay);
        (*((0x7001B000 + 1480) as *const ReadWrite<u32>)).set(params.emc_fbio_cfg8);
        (*((0x7001B000 + 1028) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank0_byte0);
        (*((0x7001B000 + 1032) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank0_byte1);
        (*((0x7001B000 + 1036) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank0_byte2);
        (*((0x7001B000 + 1040) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank0_byte3);
        (*((0x7001B000 + 1048) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank1_byte0);
        (*((0x7001B000 + 1052) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank1_byte1);
        (*((0x7001B000 + 1056) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank1_byte2);
        (*((0x7001B000 + 1060) as *const ReadWrite<u32>)).set(params.emc_swizzle_rank1_byte3);

        if params.emc_bct_spare6 != 0 {
            write_volatile(
                &mut params.emc_bct_spare6 as *mut u32,
                params.emc_bct_spare7,
            );
        }

        (*((0x7001B000 + 780) as *const ReadWrite<u32>)).set(params.emc_xm2_comp_pad_ctrl);
        (*((0x7001B000 + 1400) as *const ReadWrite<u32>)).set(params.emc_xm2_comp_pad_ctrl2);
        (*((0x7001B000 + 756) as *const ReadWrite<u32>)).set(params.emc_xm2_comp_pad_ctrl3);
        (*((0x7001B000 + 1112) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config2);
        (*((0x7001B000 + 1116) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config3);
        (*((0x7001B000 + 1456) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config4);
        (*((0x7001B000 + 1460) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config5);
        (*((0x7001B000 + 1484) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config6);
        (*((0x7001B000 + 1396) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config7);
        (*((0x7001B000 + 732) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config8);
        (*((0x7001B000 + 3144) as *const ReadWrite<u32>)).set(params.emc_pmacro_rx_term);
        (*((0x7001B000 + 3184) as *const ReadWrite<u32>)).set(params.emc_pmacro_dq_tx_drive);
        (*((0x7001B000 + 3188) as *const ReadWrite<u32>)).set(params.emc_pmacro_ca_tx_drive);
        (*((0x7001B000 + 3148) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_tx_drive);
        (*((0x7001B000 + 3192) as *const ReadWrite<u32>)).set(params.emc_pmacro_auto_cal_common);
        (*((0x7001B000 + 1124) as *const ReadWrite<u32>)).set(params.emc_auto_cal_channel);
        (*((0x7001B000 + 3140) as *const ReadWrite<u32>)).set(params.emc_pmacro_zcrtl);
        (*((0x7001B000 + 1508) as *const ReadWrite<u32>)).set(params.emc_dll_cfg0);
        (*((0x7001B000 + 1512) as *const ReadWrite<u32>)).set(params.emc_dll_cfg1);
        (*((0x7001B000 + 712) as *const ReadWrite<u32>)).set(params.emc_cfg_dig_dll_1);
        (*((0x7001B000 + 1416) as *const ReadWrite<u32>)).set(params.emc_data_brlshft0);
        (*((0x7001B000 + 1420) as *const ReadWrite<u32>)).set(params.emc_data_brlshft1);
        (*((0x7001B000 + 1428) as *const ReadWrite<u32>)).set(params.emc_dqs_brlshft0);
        (*((0x7001B000 + 1432) as *const ReadWrite<u32>)).set(params.emc_dqs_brlshft1);
        (*((0x7001B000 + 1436) as *const ReadWrite<u32>)).set(params.emc_cmd_brlshft0);
        (*((0x7001B000 + 1440) as *const ReadWrite<u32>)).set(params.emc_cmd_brlshft1);
        (*((0x7001B000 + 1444) as *const ReadWrite<u32>)).set(params.emc_cmd_brlshft2);
        (*((0x7001B000 + 1448) as *const ReadWrite<u32>)).set(params.emc_cmd_brlshft3);
        (*((0x7001B000 + 1452) as *const ReadWrite<u32>)).set(params.emc_quse_brlshft0);
        (*((0x7001B000 + 1464) as *const ReadWrite<u32>)).set(params.emc_quse_brlshft1);
        (*((0x7001B000 + 1468) as *const ReadWrite<u32>)).set(params.emc_quse_brlshft2);
        (*((0x7001B000 + 1476) as *const ReadWrite<u32>)).set(params.emc_quse_brlshft3);
        (*((0x7001B000 + 816) as *const ReadWrite<u32>))
            .set((params.emc_pmacro_brick_ctrl_rfu1 & 0x1BF01BF) | 0x1E40_1E40);
        (*((0x7001B000 + 3136) as *const ReadWrite<u32>)).set(params.emc_pmacro_pad_cfg_ctrl);
        (*((0x7001B000 + 792) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_brick_ctrl_fdpd);
        (*((0x7001B000 + 820) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_brick_ctrl_rfu2 & 0xFF7F_FF7F);
        (*((0x7001B000 + 796) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_data_brick_ctrl_fdpd);
        (*((0x7001B000 + 3132) as *const ReadWrite<u32>)).set(params.emc_pmacro_bg_bias_ctrl0);
        (*((0x7001B000 + 3156) as *const ReadWrite<u32>)).set(params.emc_pmacro_data_pad_rx_ctrl);
        (*((0x7001B000 + 3152) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_pad_rx_ctrl);
        (*((0x7001B000 + 3172) as *const ReadWrite<u32>)).set(params.emc_pmacro_data_pad_tx_ctrl);
        (*((0x7001B000 + 3164) as *const ReadWrite<u32>)).set(params.emc_pmacro_data_rx_term_mode);
        (*((0x7001B000 + 3160) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_rx_term_mode);
        (*((0x7001B000 + 3168) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_pad_tx_ctrl);
        (*((0x7001B000 + 1180) as *const ReadWrite<u32>)).set(params.emc_cfg3);
        (*((0x7001B000 + 1824) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd0);
        (*((0x7001B000 + 1828) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd1);
        (*((0x7001B000 + 1832) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd2);
        (*((0x7001B000 + 1836) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd3);
        (*((0x7001B000 + 1840) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd4);
        (*((0x7001B000 + 1844) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_pwrd5);
        (*((0x7001B000 + 1856) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src0);
        (*((0x7001B000 + 1860) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src1);
        (*((0x7001B000 + 1864) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src2);
        (*((0x7001B000 + 1868) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src3);
        (*((0x7001B000 + 1872) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src4);
        (*((0x7001B000 + 1876) as *const ReadWrite<u32>)).set(params.emc_pmacro_tx_sel_clk_src5);
        (*((0x7001B000 + 1888) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_bypass);
        (*((0x7001B000 + 1904) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_pwrd0);
        (*((0x7001B000 + 1908) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_pwrd1);
        (*((0x7001B000 + 1912) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_pwrd2);
        (*((0x7001B000 + 1920) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_ctrl0);
        (*((0x7001B000 + 1924) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_ctrl1);
        (*((0x7001B000 + 1928) as *const ReadWrite<u32>)).set(params.emc_pmacro_cmd_ctrl2);
        (*((0x7001B000 + 3040) as *const ReadWrite<u32>)).set(params.emc_pmacro_ib_vref_dq_0);
        (*((0x7001B000 + 3044) as *const ReadWrite<u32>)).set(params.emc_pmacro_ib_vref_dq_1);
        (*((0x7001B000 + 3056) as *const ReadWrite<u32>)).set(params.emc_pmacro_ib_vref_dqs_0);
        (*((0x7001B000 + 3060) as *const ReadWrite<u32>)).set(params.emc_pmacro_ib_vref_dqs_1);
        (*((0x7001B000 + 3316) as *const ReadWrite<u32>)).set(params.emc_pmacro_ib_rxrt);
        (*((0x7001B000 + 1536) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_0);
        (*((0x7001B000 + 1540) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_1);
        (*((0x7001B000 + 1544) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_2);
        (*((0x7001B000 + 1548) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_3);
        (*((0x7001B000 + 1552) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_4);
        (*((0x7001B000 + 1556) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank0_5);
        (*((0x7001B000 + 1568) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_0);
        (*((0x7001B000 + 1572) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_1);
        (*((0x7001B000 + 1576) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_2);
        (*((0x7001B000 + 1580) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_3);
        (*((0x7001B000 + 1584) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_4);
        (*((0x7001B000 + 1588) as *const ReadWrite<u32>)).set(params.emc_pmacro_quse_ddll_rank1_5);
        (*((0x7001B000 + 816) as *const ReadWrite<u32>)).set(params.emc_pmacro_brick_ctrl_rfu1);
        (*((0x7001B000 + 1600) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_0);
        (*((0x7001B000 + 1604) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_1);
        (*((0x7001B000 + 1608) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_2);
        (*((0x7001B000 + 1612) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_3);
        (*((0x7001B000 + 1616) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_4);
        (*((0x7001B000 + 1620) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank0_5);
        (*((0x7001B000 + 1632) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_0);
        (*((0x7001B000 + 1636) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_1);
        (*((0x7001B000 + 1640) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_2);
        (*((0x7001B000 + 1644) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_3);
        (*((0x7001B000 + 1648) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_4);
        (*((0x7001B000 + 1652) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dq_rank1_5);
        (*((0x7001B000 + 1664) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_0);
        (*((0x7001B000 + 1668) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_1);
        (*((0x7001B000 + 1672) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_2);
        (*((0x7001B000 + 1676) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_3);
        (*((0x7001B000 + 1680) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_4);
        (*((0x7001B000 + 1684) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank0_5);
        (*((0x7001B000 + 1696) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_0);
        (*((0x7001B000 + 1700) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_1);
        (*((0x7001B000 + 1704) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_2);
        (*((0x7001B000 + 1708) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_3);
        (*((0x7001B000 + 1712) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_4);
        (*((0x7001B000 + 1716) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ob_ddll_long_dqs_rank1_5);
        (*((0x7001B000 + 1728) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank0_0);
        (*((0x7001B000 + 1732) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank0_1);
        (*((0x7001B000 + 1736) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank0_2);
        (*((0x7001B000 + 1740) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank0_3);
        (*((0x7001B000 + 1760) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank1_0);
        (*((0x7001B000 + 1764) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank1_1);
        (*((0x7001B000 + 1768) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank1_2);
        (*((0x7001B000 + 1772) as *const ReadWrite<u32>))
            .set(params.emc_pmacro_ib_ddll_long_dqs_rank1_3);
        (*((0x7001B000 + 3072) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_long_cmd_0);
        (*((0x7001B000 + 3076) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_long_cmd_1);
        (*((0x7001B000 + 3080) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_long_cmd_2);
        (*((0x7001B000 + 3084) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_long_cmd_3);
        (*((0x7001B000 + 3088) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_long_cmd_4);
        (*((0x7001B000 + 3104) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_short_cmd_0);
        (*((0x7001B000 + 3108) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_short_cmd_1);
        (*((0x7001B000 + 3112) as *const ReadWrite<u32>)).set(params.emc_pmacro_ddll_short_cmd_2);
        (*((0x7001B000 + 3176) as *const ReadWrite<u32>))
            .set((params.emc_pmacro_common_pad_tx_ctrl & 1) | 0xE);

        if params.emc_bct_spare4 != 0 {
            write_volatile(
                &mut params.emc_bct_spare4 as *mut u32,
                params.emc_bct_spare5,
            );
        }

        (*((0x7001B000 + 40) as *const ReadWrite<u32>)).set(1);
        (*((0x70019000 + 1608) as *const ReadWrite<u32>)).set(params.mc_video_protect_bom);
        (*((0x70019000 + 2424) as *const ReadWrite<u32>)).set(params.mc_video_protect_bom_adr_hi);
        (*((0x70019000 + 1612) as *const ReadWrite<u32>)).set(params.mc_video_protect_size_mb);
        (*((0x70019000 + 1048) as *const ReadWrite<u32>)).set(params.mc_video_protect_vpr_override);
        (*((0x70019000 + 1424) as *const ReadWrite<u32>))
            .set(params.mc_video_protect_vpr_override1);
        (*((0x70019000 + 2436) as *const ReadWrite<u32>))
            .set(params.mc_video_protect_gpu_override0);
        (*((0x70019000 + 2440) as *const ReadWrite<u32>))
            .set(params.mc_video_protect_gpu_override1);
        (*((0x70019000 + 84) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg);
        (*((0x70019000 + 88) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_dev0);
        (*((0x70019000 + 92) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_dev1);
        (*((0x70019000 + 96) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_channel_mask);
        (*((0x70019000 + 100) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_bank_mask0);
        (*((0x70019000 + 104) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_bank_mask1);
        (*((0x70019000 + 108) as *const ReadWrite<u32>)).set(params.mc_emem_adr_cfg_bank_mask2);
        (*((0x70019000 + 80) as *const ReadWrite<u32>)).set(params.mc_emem_cfg);
        (*((0x70019000 + 1648) as *const ReadWrite<u32>)).set(params.mc_sec_carveout_bom);
        (*((0x70019000 + 2516) as *const ReadWrite<u32>)).set(params.mc_sec_carveout_adr_hi);
        (*((0x70019000 + 1652) as *const ReadWrite<u32>)).set(params.mc_sec_carveout_size_mb);
        (*((0x70019000 + 2464) as *const ReadWrite<u32>)).set(params.mc_mts_carveout_bom);
        (*((0x70019000 + 2472) as *const ReadWrite<u32>)).set(params.mc_mts_carveout_adr_hi);
        (*((0x70019000 + 2468) as *const ReadWrite<u32>)).set(params.mc_mts_carveout_size_mb);
        (*((0x70019000 + 144) as *const ReadWrite<u32>)).set(params.mc_emem_arb_cfg);
        (*((0x70019000 + 148) as *const ReadWrite<u32>)).set(params.mc_emem_arb_outstanding_req);
        (*((0x70019000 + 1776) as *const ReadWrite<u32>)).set(params.emc_emem_arb_refpb_hp_ctrl);
        (*((0x70019000 + 1780) as *const ReadWrite<u32>)).set(params.emc_emem_arb_refpb_bank_ctrl);
        (*((0x70019000 + 152) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rcd);
        (*((0x70019000 + 156) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rp);
        (*((0x70019000 + 160) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rc);
        (*((0x70019000 + 164) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_ras);
        (*((0x70019000 + 168) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_faw);
        (*((0x70019000 + 172) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rrd);
        (*((0x70019000 + 176) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rap2pre);
        (*((0x70019000 + 180) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_wap2pre);
        (*((0x70019000 + 184) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_r2r);
        (*((0x70019000 + 188) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_w2w);
        (*((0x70019000 + 1732) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_ccdmw);
        (*((0x70019000 + 192) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_r2w);
        (*((0x70019000 + 196) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_w2r);
        (*((0x70019000 + 1728) as *const ReadWrite<u32>)).set(params.mc_emem_arb_timing_rfcpb);
        (*((0x70019000 + 208) as *const ReadWrite<u32>)).set(params.mc_emem_arb_da_turns);
        (*((0x70019000 + 212) as *const ReadWrite<u32>)).set(params.mc_emem_arb_da_covers);
        (*((0x70019000 + 216) as *const ReadWrite<u32>)).set(params.mc_emem_arb_misc0);
        (*((0x70019000 + 220) as *const ReadWrite<u32>)).set(params.mc_emem_arb_misc1);
        (*((0x70019000 + 200) as *const ReadWrite<u32>)).set(params.mc_emem_arb_misc2);
        (*((0x70019000 + 224) as *const ReadWrite<u32>)).set(params.mc_emem_arb_ring1_throttle);
        (*((0x70019000 + 232) as *const ReadWrite<u32>)).set(params.mc_emem_arb_override);
        (*((0x70019000 + 2408) as *const ReadWrite<u32>)).set(params.mc_emem_arb_override1);
        (*((0x70019000 + 236) as *const ReadWrite<u32>)).set(params.mc_emem_arb_rsv);
        (*((0x70019000 + 2524) as *const ReadWrite<u32>)).set(params.mc_da_cfg0);
        (*((0x70019000 + 252) as *const ReadWrite<u32>)).set(1);
        (*((0x70019000 + 244) as *const ReadWrite<u32>)).set(params.mc_clken_override);
        (*((0x70019000 + 256) as *const ReadWrite<u32>)).set(params.mc_stat_control);
        (*((0x7001B000 + 16) as *const ReadWrite<u32>)).set(params.emc_adr_cfg);
        (*((0x7001B000 + 320) as *const ReadWrite<u32>)).set(params.emc_clken_override);
        (*((0x7001B000 + 1792) as *const ReadWrite<u32>)).set(params.emc_pmacro_auto_cal_cfg0);
        (*((0x7001B000 + 1796) as *const ReadWrite<u32>)).set(params.emc_pmacro_auto_cal_cfg1);
        (*((0x7001B000 + 1800) as *const ReadWrite<u32>)).set(params.emc_pmacro_auto_cal_cfg2);
        (*((0x7001B000 + 760) as *const ReadWrite<u32>)).set(params.emc_auto_cal_vref_sel0);
        (*((0x7001B000 + 768) as *const ReadWrite<u32>)).set(params.emc_auto_cal_vref_sel1);
        (*((0x7001B000 + 680) as *const ReadWrite<u32>)).set(params.emc_auto_cal_interval);
        (*((0x7001B000 + 676) as *const ReadWrite<u32>)).set(params.emc_auto_cal_config);

        usleep(params.emc_auto_cal_wait);

        if params.emc_bct_spare8 != 0 {
            write_volatile(
                &mut params.emc_bct_spare8 as *mut u32,
                params.emc_bct_spare9,
            );
        }

        (*((0x7001B000 + 696) as *const ReadWrite<u32>)).set(params.emc_cfg2);
        (*((0x7001B000 + 1376) as *const ReadWrite<u32>)).set(params.emc_cfg_pipe);
        (*((0x7001B000 + 1372) as *const ReadWrite<u32>)).set(params.emc_cfg_pipe1);
        (*((0x7001B000 + 1364) as *const ReadWrite<u32>)).set(params.emc_cfg_pipe2);
        (*((0x7001B000 + 240) as *const ReadWrite<u32>)).set(params.emc_cmd_q);
        (*((0x7001B000 + 244) as *const ReadWrite<u32>)).set(params.emc_mc2emc_q);
        (*((0x7001B000 + 200) as *const ReadWrite<u32>)).set(params.emc_mrs_wait_cnt);
        (*((0x7001B000 + 196) as *const ReadWrite<u32>)).set(params.emc_mrs_wait_cnt2);
        (*((0x7001B000 + 260) as *const ReadWrite<u32>)).set(params.emc_fbio_cfg5);
        (*((0x7001B000 + 44) as *const ReadWrite<u32>)).set(params.emc_rc);
        (*((0x7001B000 + 48) as *const ReadWrite<u32>)).set(params.emc_rfc);
        (*((0x7001B000 + 1424) as *const ReadWrite<u32>)).set(params.emc_rfc_pb);
        (*((0x7001B000 + 1408) as *const ReadWrite<u32>)).set(params.emc_ref_ctrl2);
        (*((0x7001B000 + 192) as *const ReadWrite<u32>)).set(params.emc_rfc_slr);
        (*((0x7001B000 + 52) as *const ReadWrite<u32>)).set(params.emc_ras);
        (*((0x7001B000 + 56) as *const ReadWrite<u32>)).set(params.emc_rp);
        (*((0x7001B000 + 172) as *const ReadWrite<u32>)).set(params.emc_tppd);
        (*((0x7001B000 + 324) as *const ReadWrite<u32>)).set(params.emc_r2r);
        (*((0x7001B000 + 328) as *const ReadWrite<u32>)).set(params.emc_w2w);
        (*((0x7001B000 + 60) as *const ReadWrite<u32>)).set(params.emc_r2w);
        (*((0x7001B000 + 64) as *const ReadWrite<u32>)).set(params.emc_w2r);
        (*((0x7001B000 + 68) as *const ReadWrite<u32>)).set(params.emc_r2p);
        (*((0x7001B000 + 72) as *const ReadWrite<u32>)).set(params.emc_w2p);
        (*((0x7001B000 + 1472) as *const ReadWrite<u32>)).set(params.emc_ccdmw);
        (*((0x7001B000 + 76) as *const ReadWrite<u32>)).set(params.emc_rd_rcd);
        (*((0x7001B000 + 80) as *const ReadWrite<u32>)).set(params.emc_wr_rcd);
        (*((0x7001B000 + 84) as *const ReadWrite<u32>)).set(params.emc_rrd);
        (*((0x7001B000 + 88) as *const ReadWrite<u32>)).set(params.emc_rext);
        (*((0x7001B000 + 184) as *const ReadWrite<u32>)).set(params.emc_wext);
        (*((0x7001B000 + 92) as *const ReadWrite<u32>)).set(params.emc_wdv);
        (*((0x7001B000 + 1248) as *const ReadWrite<u32>)).set(params.emc_wdv_chk);
        (*((0x7001B000 + 1176) as *const ReadWrite<u32>)).set(params.emc_wsv);
        (*((0x7001B000 + 1172) as *const ReadWrite<u32>)).set(params.emc_wev);
        (*((0x7001B000 + 720) as *const ReadWrite<u32>)).set(params.emc_wdv_mask);
        (*((0x7001B000 + 1168) as *const ReadWrite<u32>)).set(params.emc_ws_duration);
        (*((0x7001B000 + 1164) as *const ReadWrite<u32>)).set(params.emc_we_duration);
        (*((0x7001B000 + 96) as *const ReadWrite<u32>)).set(params.emc_quse);
        (*((0x7001B000 + 1384) as *const ReadWrite<u32>)).set(params.emc_quse_width);
        (*((0x7001B000 + 1128) as *const ReadWrite<u32>)).set(params.emc_ibdly);
        (*((0x7001B000 + 1132) as *const ReadWrite<u32>)).set(params.emc_obdly);
        (*((0x7001B000 + 332) as *const ReadWrite<u32>)).set(params.emc_einput);
        (*((0x7001B000 + 336) as *const ReadWrite<u32>)).set(params.emc_einput_duration);
        (*((0x7001B000 + 340) as *const ReadWrite<u32>)).set(params.emc_puterm_extra);
        (*((0x7001B000 + 1388) as *const ReadWrite<u32>)).set(params.emc_puterm_width);
        (*((0x7001B000 + 3176) as *const ReadWrite<u32>)).set(params.emc_pmacro_common_pad_tx_ctrl);
        (*((0x7001B000 + 8) as *const ReadWrite<u32>)).set(params.emc_dbg);
        (*((0x7001B000 + 100) as *const ReadWrite<u32>)).set(params.emc_qrst);
        (*((0x7001B000 + 1064) as *const ReadWrite<u32>)).set(0);
        (*((0x7001B000 + 104) as *const ReadWrite<u32>)).set(params.emc_qsafe);
        (*((0x7001B000 + 108) as *const ReadWrite<u32>)).set(params.emc_rdv);
        (*((0x7001B000 + 716) as *const ReadWrite<u32>)).set(params.emc_rdv_mask);
        (*((0x7001B000 + 728) as *const ReadWrite<u32>)).set(params.emc_rdv_early);
        (*((0x7001B000 + 724) as *const ReadWrite<u32>)).set(params.emc_rdv_early_mask);
        (*((0x7001B000 + 1380) as *const ReadWrite<u32>)).set(params.emc_qpop);
        (*((0x7001B000 + 112) as *const ReadWrite<u32>)).set(params.emc_refresh);
        (*((0x7001B000 + 116) as *const ReadWrite<u32>)).set(params.emc_burst_refresh_num);
        (*((0x7001B000 + 988) as *const ReadWrite<u32>)).set(params.emc_prerefresh_req_cnt);
        (*((0x7001B000 + 120) as *const ReadWrite<u32>)).set(params.emc_pdex2wr);
        (*((0x7001B000 + 124) as *const ReadWrite<u32>)).set(params.emc_pdex2rd);
        (*((0x7001B000 + 128) as *const ReadWrite<u32>)).set(params.emc_pchg2pden);
        (*((0x7001B000 + 132) as *const ReadWrite<u32>)).set(params.emc_act2pden);
        (*((0x7001B000 + 136) as *const ReadWrite<u32>)).set(params.emc_ar2pden);
        (*((0x7001B000 + 140) as *const ReadWrite<u32>)).set(params.emc_rw2pden);
        (*((0x7001B000 + 284) as *const ReadWrite<u32>)).set(params.emc_cke2pden);
        (*((0x7001B000 + 280) as *const ReadWrite<u32>)).set(params.emc_pdex2che);
        (*((0x7001B000 + 180) as *const ReadWrite<u32>)).set(params.emc_pdex2mrr);
        (*((0x7001B000 + 144) as *const ReadWrite<u32>)).set(params.emc_txsr);
        (*((0x7001B000 + 996) as *const ReadWrite<u32>)).set(params.emc_txsr_dll);
        (*((0x7001B000 + 148) as *const ReadWrite<u32>)).set(params.emc_tcke);
        (*((0x7001B000 + 344) as *const ReadWrite<u32>)).set(params.emc_tckesr);
        (*((0x7001B000 + 348) as *const ReadWrite<u32>)).set(params.emc_tpd);
        (*((0x7001B000 + 152) as *const ReadWrite<u32>)).set(params.emc_tfaw);
        (*((0x7001B000 + 156) as *const ReadWrite<u32>)).set(params.emc_trpab);
        (*((0x7001B000 + 160) as *const ReadWrite<u32>)).set(params.emc_tclkstable);
        (*((0x7001B000 + 164) as *const ReadWrite<u32>)).set(params.emc_tclkstop);
        (*((0x7001B000 + 168) as *const ReadWrite<u32>)).set(params.emc_trefbw);
        (*((0x7001B000 + 176) as *const ReadWrite<u32>)).set(params.emc_odt_write);
        (*((0x7001B000 + 700) as *const ReadWrite<u32>)).set(params.emc_cfg_dig_dll);
        (*((0x7001B000 + 704) as *const ReadWrite<u32>)).set(params.emc_cfg_dig_dll_period);
        (*((0x7001B000 + 256) as *const ReadWrite<u32>)).set(params.emc_fbio_spare & 0xFFFF_FFFD);
        (*((0x7001B000 + 288) as *const ReadWrite<u32>)).set(params.emc_cfg_rsv);
        (*((0x7001B000 + 1088) as *const ReadWrite<u32>)).set(params.emc_pmc_scratch1);
        (*((0x7001B000 + 1092) as *const ReadWrite<u32>)).set(params.emc_pmc_scratch2);
        (*((0x7001B000 + 1096) as *const ReadWrite<u32>)).set(params.emc_pmc_scratch3);
        (*((0x7001B000 + 292) as *const ReadWrite<u32>)).set(params.emc_acpd_control);
        (*((0x7001B000 + 1152) as *const ReadWrite<u32>)).set(params.emc_txdsrvttgen);
        (*((0x7001B000 + 12) as *const ReadWrite<u32>)).set((params.emc_cfg & 0xE) | 0x3C00000);

        if params.boot_rom_patch_control & 0x8000_0000 != 0 {
            (*((4 * (params.boot_rom_patch_control + 0x1C00_0000)) as *const ReadWrite<u32>))
                .set(params.boot_rom_patch_data);
            (*((0x70019000 + 252) as *const ReadWrite<u32>)).set(1);
        }

        pmc.io_dpd3_req
            .set(((4 * params.emc_pmc_scratch1 >> 2) + 0x4000_0000) & 0xCFFF_0000);
        usleep(params.pmc_io_dpd3_req_wait);

        if params.emc_auto_cal_interval == 0 {
            (*((0x7001B000 + 676) as *const ReadWrite<u32>))
                .set(params.emc_auto_cal_config | 0x200);
        }

        (*((0x7001B000 + 820) as *const ReadWrite<u32>)).set(params.emc_pmacro_brick_ctrl_rfu2);

        if params.emc_zcal_warm_cold_boot_enables & 1 != 0 {
            if params.memory_type == 2 {
                (*((0x7001B000 + 740) as *const ReadWrite<u32>)).set(8 * params.emc_zcal_wait_cnt);
            }

            if params.memory_type == 3 {
                (*((0x7001B000 + 740) as *const ReadWrite<u32>)).set(params.emc_zcal_wait_cnt);
                (*((0x7001B000 + 744) as *const ReadWrite<u32>)).set(params.emc_zcal_mrw_cmd);
            }
        }

        (*((0x7001B000 + 40) as *const ReadWrite<u32>)).set(1);
        usleep(params.emc_timing_control_wait);
        pmc.ddr_cntrl.set(pmc.ddr_cntrl.get() & 0xFFF8_007F);
        usleep(params.pmc_ddr_ctrl_wait);

        if params.memory_type == 2 {
            (*((0x7001B000 + 36) as *const ReadWrite<u32>))
                .set((params.emc_pin_gpio_enable << 16) | (params.emc_pin_gpio << 12));
            usleep(params.emc_pin_extra_wait + 200);
            (*((0x7001B000 + 36) as *const ReadWrite<u32>))
                .set(((params.emc_pin_gpio_enable << 16) | (params.emc_pin_gpio << 12)) + 256);
            usleep(params.emc_pin_extra_wait + 500);
        }

        if params.memory_type == 3 {
            (*((0x7001B000 + 36) as *const ReadWrite<u32>))
                .set((params.emc_pin_gpio_enable << 16) | (params.emc_pin_gpio << 12));
            usleep(params.emc_pin_extra_wait + 200);
            (*((0x7001B000 + 36) as *const ReadWrite<u32>))
                .set(((params.emc_pin_gpio_enable << 16) | (params.emc_pin_gpio << 12)) + 256);
            usleep(params.emc_pin_extra_wait + 2000);
        }

        (*((0x7001B000 + 36) as *const ReadWrite<u32>))
            .set(((params.emc_pin_gpio_enable << 16) | (params.emc_pin_gpio << 12)) + 0x101);
        usleep(params.emc_pin_program_wait);

        if params.memory_type != 3 {
            (*((0x7001B000 + 220) as *const ReadWrite<u32>)).set((params.emc_dev_select << 30) + 1);
        }

        if params.memory_type == 1 {
            usleep(params.emc_pin_extra_wait + 200);
        }

        if params.memory_type == 3 {
            if params.emc_bct_spare10 != 0 {
                write_volatile(
                    &mut params.emc_bct_spare10 as *mut u32,
                    params.emc_bct_spare11,
                );
            }

            (*((0x7001B000 + 308) as *const ReadWrite<u32>)).set(params.emc_mrw2);
            (*((0x7001B000 + 232) as *const ReadWrite<u32>)).set(params.emc_mrw1);
            (*((0x7001B000 + 312) as *const ReadWrite<u32>)).set(params.emc_mrw3);
            (*((0x7001B000 + 316) as *const ReadWrite<u32>)).set(params.emc_mrw4);
            (*((0x7001B000 + 1188) as *const ReadWrite<u32>)).set(params.emc_mrw6);
            (*((0x7001B000 + 1220) as *const ReadWrite<u32>)).set(params.emc_mrw14);
            (*((0x7001B000 + 1196) as *const ReadWrite<u32>)).set(params.emc_mrw8);
            (*((0x7001B000 + 1212) as *const ReadWrite<u32>)).set(params.emc_mrw12);
            (*((0x7001B000 + 1200) as *const ReadWrite<u32>)).set(params.emc_mrw9);
            (*((0x7001B000 + 1216) as *const ReadWrite<u32>)).set(params.emc_mrw13);

            if params.emc_zcal_warm_cold_boot_enables & 1 != 0 {
                (*((0x7001B000 + 748) as *const ReadWrite<u32>)).set(params.emc_zcal_init_dev0);
                usleep(params.emc_zcal_init_wait);
                (*((0x7001B000 + 748) as *const ReadWrite<u32>)).set(params.emc_zcal_init_dev0 ^ 3);

                if params.emc_dev_select & 2 == 0 {
                    (*((0x7001B000 + 748) as *const ReadWrite<u32>)).set(params.emc_zcal_init_dev1);
                    usleep(params.emc_zcal_init_wait);
                    (*((0x7001B000 + 748) as *const ReadWrite<u32>))
                        .set(params.emc_zcal_init_dev1 ^ 3);
                }
            }
        }

        pmc.ddr_cfg.set(params.pmc_ddr_cfg);

        if (params.memory_type - 1) <= 2 {
            (*((0x7001B000 + 736) as *const ReadWrite<u32>)).set(params.emc_zcal_interval);
            (*((0x7001B000 + 740) as *const ReadWrite<u32>)).set(params.emc_zcal_wait_cnt);
            (*((0x7001B000 + 744) as *const ReadWrite<u32>)).set(params.emc_zcal_mrw_cmd);
        }

        if params.emc_bct_spare12 != 0 {
            write_volatile(
                &mut params.emc_bct_spare12 as *mut u32,
                params.emc_bct_spare13,
            );
        }

        (*((0x7001B000 + 40) as *const ReadWrite<u32>)).set(1);

        if params.emc_extra_refresh_num != 0 {
            (*((0x7001B000 + 212) as *const ReadWrite<u32>)).set(
                ((1 << params.emc_extra_refresh_num << 8) - 0xFD) | (params.emc_pin_gpio << 30),
            );
        }

        (*((0x7001B000 + 32) as *const ReadWrite<u32>)).set(params.emc_dev_select | 0x80000000);
        (*((0x7001B000 + 992) as *const ReadWrite<u32>)).set(params.emc_dyn_self_ref_control);
        (*((0x7001B000 + 1524) as *const ReadWrite<u32>)).set(params.emc_cfg_update);
        (*((0x7001B000 + 12) as *const ReadWrite<u32>)).set(params.emc_cfg);
        (*((0x7001B000 + 784) as *const ReadWrite<u32>)).set(params.emc_fdpd_ctrl_dq);
        (*((0x7001B000 + 788) as *const ReadWrite<u32>)).set(params.emc_fdpd_ctrl_cmd);
        (*((0x7001B000 + 984) as *const ReadWrite<u32>)).set(params.emc_sel_dpd_ctrl);
        (*((0x7001B000 + 256) as *const ReadWrite<u32>)).set(params.emc_fbio_spare | 2);
        (*((0x7001B000 + 40) as *const ReadWrite<u32>)).set(1);
        (*((0x7001B000 + 1368) as *const ReadWrite<u32>)).set(params.emc_cfg_pipe_clk);
        (*((0x7001B000 + 1240) as *const ReadWrite<u32>)).set(params.emc_fdpd_ctrl_cmd_no_ramp);

        let ahb_arbitration_xbar_ctrl_0 = &*((0x6000C000 + 0xE0) as *const ReadWrite<u32>);
        ahb_arbitration_xbar_ctrl_0.set(
            (ahb_arbitration_xbar_ctrl_0.get() & 0xFFFE_FFFF)
                | ((params.ahb_arbitration_xbar_ctrl_meminit_done & 0xFFFF) << 16),
        );

        (*((0x70019000 + 1616) as *const ReadWrite<u32>)).set(params.mc_video_protect_write_access);
        (*((0x70019000 + 1656) as *const ReadWrite<u32>))
            .set(params.mc_sec_carveout_protect_write_access);
        (*((0x70019000 + 2476) as *const ReadWrite<u32>)).set(params.mc_mts_carveout_reg_ctrl);
        (*((0x70019000 + 1636) as *const ReadWrite<u32>)).set(1);
    }
}

/// Retrieves the SDRAM parameters.
pub fn get_parameters() -> Parameters {
    // TODO(Vale): LZ77 compression of the config values.
    let parameters: Parameters = unsafe { transmute_copy(&DRAM_CONFIG[get_sdram_id()]) };

    parameters
}

/// Initializes and configures the SDRAM.
pub fn init(car: &Car, pmc: &Pmc) {
    let mut params = get_parameters();

    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x22, 5).unwrap();
    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x17, 40).unwrap();

    pmc.vddp_sel.set(params.pmc_vddp_sel);
    usleep(params.pmc_vddp_sel_wait);

    pmc.ddr_pwr.set(pmc.ddr_pwr.get());
    pmc.no_iopower.set(params.pmc_no_io_power);
    pmc.reg_short.set(params.pmc_reg_short);
    pmc.ddr_cntrl.set(params.pmc_ddr_ctrl);

    if params.emc_bct_spare0 != 0 {
        unsafe {
            write_volatile(
                &mut params.emc_bct_spare0 as *mut u32,
                params.emc_bct_spare0,
            );
        }
    }

    config_sdram(car, pmc, &mut params);
}
