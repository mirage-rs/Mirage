//! Low-level hardware access library for the Nintendo Switch.
//!
//! **Note:** This code is written specifically for the Switch.
//! If you decide to use it for other Tegra210 platforms, use
//! at own risk.

#![no_std]
#![feature(const_fn)]
#![feature(optimize_attribute)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate enum_primitive;

extern crate mirage_mmio;

extern crate paste;

use core::ptr::write_bytes;

use mirage_mmio::{Mmio, VolatileStorage};

use crate::{
    clock::{Car, Clock},
    fuse::FuseChip,
    gpio::{Gpio, GpioConfig},
    i2c::*,
    pinmux::*,
    pmc::Pmc,
    se::SecurityEngine,
    sysctr0::Sysctr0Registers,
    timer::usleep,
    uart::Uart,
};

pub mod apb_misc;
pub mod button;
pub mod clock;
pub mod cluster;
pub mod display;
pub mod fuse;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod mc;
pub mod pinmux;
pub mod pmc;
pub mod power;
pub mod rtc;
pub mod sdmmc;
pub mod sdram;
pub mod se;
pub mod sysctr0;
pub mod sysreg;
pub mod timer;
pub mod tsec;
pub mod uart;

/// Base address for I2S registers.
const I2S_BASE: u32 = 0x702D_1000;

const I2S1_CG: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x88) as *const _) };

const I2S1_CTRL: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0xA0) as *const _) };

const I2S2_CG: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x188) as *const _) };

const I2S2_CTRL: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x1A0) as *const _) };

const I2S3_CG: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x288) as *const _) };

const I2S3_CTRL: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x2A0) as *const _) };

const I2S4_CG: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x388) as *const _) };

const I2S4_CTRL: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x3A0) as *const _) };

const I2S5_CG: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x488) as *const _) };

const I2S5_CTRL: Mmio<u32> = unsafe { Mmio::new((I2S_BASE + 0x4A0) as *const _) };

/// The global instance of the Security Engine.
const SECURITY_ENGINE: SecurityEngine = SecurityEngine::new();

fn config_oscillators(car: &Car, pmc: &Pmc) {
    let sysctr0 = unsafe { Sysctr0Registers::get() };

    // Set CLK_M_DIVISOR to 2.
    car.spare_reg0.write((car.spare_reg0.read() & 0xFFFF_FFF3) | 4);
    // Set counter frequency.
    sysctr0.CNTFID0.write(19200000);
    // For 19.2MHz clk_m.
    timer::TIMERUS_USEC_CFG.write(0x45F);

    // Set OSC to 38.4MHz and drive strength.
    car.osc_ctrl.write(0x5000_0071);

    // // Set LP0 OSC drive strength.
    pmc.osc_edpd_over.write((pmc.osc_edpd_over.read() & 0xFFFF_FF81) | 0xE);
    pmc.osc_edpd_over.write((pmc.osc_edpd_over.read() & 0xFFBF_FFFF) | 0x400000);
    pmc.cntrl2.write((pmc.cntrl2.read() & 0xFFFF_EFFF) | 0x1000);
    // LP0 EMC2TMC_CFG_XM2COMP_PU_VREF_SEL_RANGE.
    pmc.scratch188.write((pmc.scratch188.read() & 0xFCFF_FFFF) | 0x2000000);

    // // Set HCLK div to 2 and PCLK div to 1.
    car.clk_sys_rate.write(0x10);
    // Disable PLLMB.
    car.pllmb_base.write(car.pllmb_base.read() & 0xBFFF_FFFF);

    pmc.tsc_mult.write((pmc.tsc_mult.read() & 0xFFFF_0000) | 0x249F); //0x249F = (16 / 32.768 kHz)

    // Set SCLK div to 1.
    car.clk_source_sys.write(0);
    // Set clk source to Run and PLLP_OUT2 (204MHz).
    car.sclk_brst_pol.write(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    car.super_sclk_div.write(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    car.clk_sys_rate.write(2);
}

fn config_gpios(pinmux: &Pinmux) {
    pinmux.uart2_tx.write(0);
    pinmux.uart3_tx.write(0);

    // Set Joy-Con IsAttached direction.
    pinmux.pe6.write(INPUT);
    pinmux.ph6.write(INPUT);

    // Enable input logic for Joy-Con IsAttached and UART_B/C TX pins.
    gpio!(G, 0).config(GpioConfig::Input);
    gpio!(D, 1).config(GpioConfig::Input);
    gpio!(E, 6).config(GpioConfig::Input);
    gpio!(H, 6).config(GpioConfig::Input);

    pinmux.configure_i2c(&I2c::C1);
    pinmux.configure_i2c(&I2c::C5);
    pinmux.configure_uart(&Uart::A);

    // Configure Volume Up/Down as inputs.
    Gpio::BUTTON_VOL_UP.config(GpioConfig::Input);
    Gpio::BUTTON_VOL_DOWN.config(GpioConfig::Input);
}

fn config_pmc_scratch(pmc: &Pmc) {
    pmc.scratch20.write(pmc.scratch20.read() & 0xFFF3_FFFF);
    pmc.scratch190.write(pmc.scratch190.read() & 0xFFFF_FFFE);
    pmc.secure_scratch21.write(pmc.secure_scratch21.read() | 0x10);
}

fn mbist_workaround(car: &Car) {
    car.clk_source_sor1.write((car.clk_source_sor1.read() | 0x8000) & 0xFFFF_BFFF);
    car.plld_base.write(car.plld_base.read() | 0x4080_0000);
    car.rst_dev_y_clr.write(0x40);
    car.rst_dev_x_clr.write(0x40000);
    car.rst_dev_l_clr.write(0x1800_0000);
    usleep(2);

    // Setup I2S.
    I2S1_CTRL.write(I2S1_CTRL.read() | 0x400);
    I2S1_CG.write(I2S1_CG.read() & 0xFFFF_FFFE);
    I2S2_CTRL.write(I2S2_CTRL.read() | 0x400);
    I2S2_CG.write(I2S2_CG.read() & 0xFFFF_FFFE);
    I2S3_CTRL.write(I2S3_CTRL.read() | 0x400);
    I2S3_CG.write(I2S3_CG.read() & 0xFFFF_FFFE);
    I2S4_CTRL.write(I2S4_CTRL.read() | 0x400);
    I2S4_CG.write(I2S4_CG.read() & 0xFFFF_FFFE);
    I2S5_CTRL.write(I2S5_CTRL.read() | 0x400);
    I2S5_CG.write(I2S5_CG.read() & 0xFFFF_FFFE);

    unsafe {
        let dc_com_dsc_top_ctl = Mmio::new((0x5420_0000 + 0x33E * 4) as *const u32);
        dc_com_dsc_top_ctl.write(dc_com_dsc_top_ctl.read() | 4);
        Mmio::new((0x5434_0000 + 0x8C) as *const u32).write(0xFFFF_FFFF);
    }
    usleep(2);

    // Set devices in reset.
    car.rst_dev_y_set.write(0x40);
    car.rst_dev_l_set.write(0x1800_0000);
    car.rst_dev_x_set.write(0x40000);

    // Clock out enables.
    car.clk_out_enb_h.write(0xC0);
    car.clk_out_enb_l.write(0x8000_0130);
    car.clk_out_enb_u.write(0x1F00200);
    car.clk_out_enb_v.write(0x8040_0808);
    car.clk_out_enb_w.write(0x4020_00FC);
    car.clk_out_enb_x.write(0x2300_0780);
    car.clk_out_enb_y.write(0x300);

    // LVL2 clock gate overrides.
    car.lvl2_clk_gate_ovra.write(0);
    car.lvl2_clk_gate_ovrb.write(0);
    car.lvl2_clk_gate_ovrc.write(0);
    car.lvl2_clk_gate_ovrd.write(0);
    car.lvl2_clk_gate_ovre.write(0);

    // Configure clock sources.
    car.plld_base.write(car.plld_base.read() & 0x1F7F_FFFF);
    car.clk_source_sor1.write(car.clk_source_sor1.read() & 0xFFFF_3FFF);
    car.clk_source_vi.write((car.clk_source_vi.read() & 0x1FFF_FFFF) | 0x8000_0000);
    car.clk_source_host1x.write((car.clk_source_host1x.read() & 0x1FFF_FFFF) | 0x8000_0000);
    car.clk_source_nvenc.write((car.clk_source_nvenc.read() & 0x1FFF_FFFF) | 0x8000_0000);
}

fn config_se_brom(pmc: &Pmc) {
    let fuse_chip = unsafe { FuseChip::get() };

    // Bootrom part we skipped.
    // TODO(Vale): Do the private_key parts even fit an u8?
    let sbk = [
        fuse_chip.private_key[0].read() as u8,
        fuse_chip.private_key[1].read() as u8,
        fuse_chip.private_key[2].read() as u8,
        fuse_chip.private_key[3].read() as u8,
    ];
    SECURITY_ENGINE.set_aes_keyslot(0xE, &sbk);

    SECURITY_ENGINE.lock_sbk();

    // Without this, TZRAM will behave weirdly later on.
    unsafe {
        write_bytes(0x7C010000 as *mut u32, 0, 0x10000);
    }

    pmc.crypto_op.write(0);
    SECURITY_ENGINE.config_brom();

    SECURITY_ENGINE.lock_ssk();

    // Clear the boot reason to avoid problems later.
    pmc.scratch200.write(0);
    pmc.reset_status.write(0);
}

/// Initializes the Switch hardware in an early bootrom context.
pub fn hardware_init() {
    let car = unsafe { Car::get() };
    let pinmux = unsafe { Pinmux::get() };
    let pmc = unsafe { Pmc::get() };

    // Bootrom stuff that was skipped by going through RCM.
    config_se_brom(pmc);

    unsafe {
        let ahb_spare_reg = Mmio::new((0x6000_C000 + 0x110) as *const u32);
        ahb_spare_reg.write(ahb_spare_reg.read() & 0xFFFF_FF9F);
    }
    pmc.scratch49.write(((pmc.scratch49.read() >> 1) << 1) & 0xFFFF_FFFD);

    // Apply the memory built-in self test workaround.
    mbist_workaround(car);

    // Reboot SE.
    Clock::SE.enable();

    // Initialize the fuse driver.
    fuse::init();

    // Initialize the memory controller.
    mc::enable_mc();

    // Configure oscillators.
    config_oscillators(car, pmc);

    // Disable pinmux tristate input clamping.
    unsafe {
        Mmio::new((0x7000_0000 + 0x40) as *const u32).write(0);
    }

    // Configure GPIOs.
    config_gpios(pinmux);

    // Reboot CL-DVFS.
    Clock::CL_DVFS.enable();

    // Reboot TZRAM.
    Clock::TZRAM.enable();

    // Initialize I2C 1.
    I2c::C1.init();

    // Initialize I2C 5.
    I2c::C5.init();

    // Configure the PMIC.
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x4, 0x40)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x41, 0x60)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x43, 0x38)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x44, 0x3A)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x45, 0x38)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x4A, 0xF)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x4E, 0xC7)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x4F, 0x4F)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x50, 0x29)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x52, 0x1B)
        .unwrap();
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x56, 0x22)
        .unwrap();

    // Configure SD0 voltage.
    I2c::C5
        .write_byte(MAX77620_PWR_I2C_ADDR, 0x16, 42)
        .unwrap();

    // Configure and lock PMC scratch registers.
    config_pmc_scratch(pmc);

    // Set super clock burst policy.
    car.sclk_brst_pol.write((car.sclk_brst_pol.read() & 0xFFFF_8888) | 0x3333);

    // Initialize SDRAM.
    sdram::init(car, pmc);

    // TODO(Vale): Save SDRAM LP0 parameters.
}
