//! Low-level hardware access library for the Nintendo Switch.
//!
//! **Note:** This code is written specifically for the Switch.
//! If you decide to use it for other Tegra210 platforms, use
//! at own risk.

#![no_std]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(optimize_attribute)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate lazy_static;

extern crate paste;

extern crate register;

use core::ptr::write_bytes;

use register::mmio::ReadWrite;

use crate::gpio::*;

#[macro_use]
mod utils;

pub mod button;
pub mod clock;
pub mod fuse;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod mc;
pub mod pinmux;
pub mod pmc;
pub mod rtc;
pub mod sdram;
pub mod se;
pub mod sysctr0;
pub mod timer;
pub mod tsec;
pub mod uart;

/// Base address for I2S registers.
const I2S_BASE: u32 = 0x702D_1000;

register!(I2S1_CG, I2S_BASE + 0x88);

register!(I2S1_CTRL, I2S_BASE + 0xA0);

register!(I2S2_CG, I2S_BASE + 0x188);

register!(I2S2_CTRL, I2S_BASE + 0x1A0);

register!(I2S3_CG, I2S_BASE + 0x288);

register!(I2S3_CTRL, I2S_BASE + 0x2A0);

register!(I2S4_CG, I2S_BASE + 0x388);

register!(I2S4_CTRL, I2S_BASE + 0x3A0);

register!(I2S5_CG, I2S_BASE + 0x488);

register!(I2S5_CTRL, I2S_BASE + 0x4A0);

/// The global instance of the Security Engine.
pub const SECURITY_ENGINE: se::SecurityEngine = se::SecurityEngine::new();

fn config_oscillators(car: &clock::Car, pmc: &pmc::Pmc) {
    // Set CLK_M_DIVISOR to 2.
    car.spare_reg0.set((car.spare_reg0.get() & 0xFFFF_FFF3) | 4);
    // Set counter frequency.
    sysctr0::CNTFID0.set(19200000);
    // For 19.2MHz clk_m.
    timer::TIMERUS_USEC_CFG.set(0x45F);

    // Set OSC to 38.4MHz and drive strength.
    car.osc_ctrl.set(0x5000_0071);

    // // Set LP0 OSC drive strength.
    pmc.osc_edpd_over
        .set((pmc.osc_edpd_over.get() & 0xFFFF_FF81) | 0xE);
    pmc.osc_edpd_over
        .set((pmc.osc_edpd_over.get() & 0xFFBF_FFFF) | 0x400000);
    pmc.cntrl2.set((pmc.cntrl2.get() & 0xFFFF_EFFF) | 0x1000);
    // LP0 EMC2TMC_CFG_XM2COMP_PU_VREF_SEL_RANGE.
    pmc.scratch188
        .set((pmc.scratch188.get() & 0xFCFF_FFFF) | 0x2000000);

    // // Set HCLK div to 2 and PCLK div to 1.
    car.clk_sys_rate.set(0x10);
    // Disable PLLMB.
    car.pllmb_base.set(car.pllmb_base.get() & 0xBFFF_FFFF);

    pmc.tsc_mult
        .set((pmc.tsc_mult.get() & 0xFFFF_0000) | 0x249F); //0x249F = 19200000 * (16 / 32.768 kHz)

    // Set SCLK div to 1.
    car.clk_source_sys.set(0);
    // Set clk source to Run and PLLP_OUT2 (204MHz).
    car.sclk_brst_pol.set(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    car.super_sclk_div.set(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    car.clk_sys_rate.set(2);
}

fn config_gpios(pinmux: &pinmux::Pinmux) {
    pinmux.uart2_tx.set(0);
    pinmux.uart3_tx.set(0);

    // Set Joy-Con IsAttached direction.
    pinmux.pe6.set(pinmux::INPUT);
    pinmux.ph6.set(pinmux::INPUT);

    // Set pin mode for Joy-Con IsAttached and UART_B/C TX pins.
    gpio!(G, 0).set_mode(gpio::GpioMode::GPIO);
    gpio!(D, 1).set_mode(gpio::GpioMode::GPIO);

    // Set Joy-Con IsAttached mode.
    gpio!(E, 6).set_mode(gpio::GpioMode::GPIO);
    gpio!(H, 6).set_mode(gpio::GpioMode::GPIO);

    // Enable input logic for Joy-Con IsAttached and UART_B/C TX pins.
    gpio!(G, 0).config(gpio::GpioConfig::Input);
    gpio!(D, 1).config(gpio::GpioConfig::Input);
    gpio!(E, 6).config(gpio::GpioConfig::Input);
    gpio!(H, 6).config(gpio::GpioConfig::Input);

    pinmux::configure_i2c(pinmux, &i2c::I2c::C1);
    pinmux::configure_i2c(pinmux, &i2c::I2c::C5);
    pinmux::configure_uart(pinmux, &uart::Uart::A);

    // Configure Volume Up/Down as inputs.
    gpio::Gpio::BUTTON_VOL_UP.config(gpio::GpioConfig::Input);
    gpio::Gpio::BUTTON_VOL_DOWN.config(gpio::GpioConfig::Input);
}

fn config_pmc_scratch(pmc: &pmc::Pmc) {
    pmc.scratch20.set(pmc.scratch20.get() & 0xFFF3_FFFF);
    pmc.scratch190.set(pmc.scratch190.get() & 0xFFFF_FFFE);
    pmc.secure_scratch21.set(pmc.secure_scratch21.get() | 0x10);
}

fn mbist_workaround(car: &clock::Car) {
    car.clk_source_sor1
        .set((car.clk_source_sor1.get() | 0x8000) & 0xFFFF_BFFF);
    car.plld_base.set(car.plld_base.get() | 0x4080_0000);
    car.rst_dev_y_clr.set(0x40);
    car.rst_dev_x_clr.set(0x40000);
    car.rst_dev_l_clr.set(0x1800_0000);
    timer::usleep(2);

    // Setup I2S.
    I2S1_CTRL.set(I2S1_CTRL.get() | 0x400);
    I2S1_CG.set(I2S1_CG.get() & 0xFFFF_FFFE);
    I2S2_CTRL.set(I2S2_CTRL.get() | 0x400);
    I2S2_CG.set(I2S2_CG.get() & 0xFFFF_FFFE);
    I2S3_CTRL.set(I2S3_CTRL.get() | 0x400);
    I2S3_CG.set(I2S3_CG.get() & 0xFFFF_FFFE);
    I2S4_CTRL.set(I2S4_CTRL.get() | 0x400);
    I2S4_CG.set(I2S4_CG.get() & 0xFFFF_FFFE);
    I2S5_CTRL.set(I2S5_CTRL.get() | 0x400);
    I2S5_CG.set(I2S5_CG.get() & 0xFFFF_FFFE);

    unsafe {
        let dc_com_dsc_top_ctl = &*((0x5420_0000 + 0x33E * 4) as *const ReadWrite<u32>);
        dc_com_dsc_top_ctl.set(dc_com_dsc_top_ctl.get() | 4);
        (*((0x5434_0000 + 0x8C) as *const ReadWrite<u32>)).set(0xFFFF_FFFF);
    }
    timer::usleep(2);

    // Set devices in reset.
    car.rst_dev_y_set.set(0x40);
    car.rst_dev_l_set.set(0x1800_0000);
    car.rst_dev_x_set.set(0x40000);

    // Clock out enables.
    car.clk_out_enb_h.set(0xC0);
    car.clk_out_enb_l.set(0x8000_0130);
    car.clk_out_enb_u.set(0x1F00200);
    car.clk_out_enb_v.set(0x8040_0808);
    car.clk_out_enb_w.set(0x4020_00FC);
    car.clk_out_enb_x.set(0x2300_0780);
    car.clk_out_enb_y.set(0x300);

    // LVL2 clock gate overrides.
    car.lvl2_clk_gate_ovra.set(0);
    car.lvl2_clk_gate_ovrb.set(0);
    car.lvl2_clk_gate_ovrc.set(0);
    car.lvl2_clk_gate_ovrd.set(0);
    car.lvl2_clk_gate_ovre.set(0);

    // Configure clock sources.
    car.plld_base.set(car.plld_base.get() & 0x1F7F_FFFF);
    car.clk_source_sor1
        .set(car.clk_source_sor1.get() & 0xFFFF_3FFF);
    car.clk_source_vi
        .set((car.clk_source_vi.get() & 0x1FFF_FFFF) | 0x8000_0000);
    car.clk_source_host1x
        .set((car.clk_source_host1x.get() & 0x1FFF_FFFF) | 0x8000_0000);
    car.clk_source_nvenc
        .set((car.clk_source_nvenc.get() & 0x1FFF_FFFF) | 0x8000_0000);
}

fn config_se_brom(pmc: &pmc::Pmc) {
    let fuse_chip = unsafe { &*fuse::FuseChip::get() };

    // Bootrom part we skipped.
    let sbk = [
        fuse_chip.private_key[0].get() as u8,
        fuse_chip.private_key[1].get() as u8,
        fuse_chip.private_key[2].get() as u8,
        fuse_chip.private_key[3].get() as u8,
    ];
    SECURITY_ENGINE.set_aes_keyslot(0xE, &sbk);

    SECURITY_ENGINE.lock_sbk();

    // Without this, TZRAM will behave weirdly later on.
    unsafe {
        write_bytes(0x7C010000 as *mut u32, 0, 0x10000);
    }

    pmc.crypto_op.set(0);

    SECURITY_ENGINE.lock_ssk();

    // Clear the boot reason to avoid problems later.
    pmc.scratch200.set(0);
    pmc.reset_status.set(0);
}

/// Initializes the Switch hardware in an early bootrom context.
pub fn hardware_init() {
    let car = &clock::Car::new();
    let pinmux = &pinmux::Pinmux::new();
    let pmc = &pmc::Pmc::new();

    // Bootrom stuff that was skipped by going through RCM.
    config_se_brom(pmc);

    unsafe {
        let ahb_spare_reg_0 = &*((0x6000_C000 + 0x110) as *const ReadWrite<u32>);
        ahb_spare_reg_0.set(ahb_spare_reg_0.get() & 0xFFFF_FF9F);
    }
    pmc.scratch49
        .set(((pmc.scratch49.get() >> 1) << 1) & 0xFFFF_FFFD);

    // Apply the memory built-in self test workaround.
    mbist_workaround(car);

    // Reboot SE.
    clock::Clock::SE.enable();

    // Initialize the fuse driver.
    fuse::init();

    // Initialize the memory controller.
    mc::enable_mc();

    // Configure oscillators.
    config_oscillators(car, pmc);

    // Disable pinmux tristate input clamping.
    unsafe {
        (*((0x7000_0000 + 0x40) as *const ReadWrite<u32>)).set(0);
    }

    // Configure GPIOs.
    config_gpios(pinmux);

    // Reboot CL-DVFS.
    clock::Clock::CL_DVFS.enable();

    // Reboot I2C1.
    clock::Clock::I2C_1.enable();

    // Reboot I2C5.
    clock::Clock::I2C_5.enable();

    // Reboot TZRAM.
    clock::Clock::TZRAM.enable();

    // Initialize I2C 1.
    i2c::I2c::C1.init();

    // Initialize I2C 5.
    i2c::I2c::C5.init();

    // Configure the PMIC.
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x4, 0x40)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x41, 0x60)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x43, 0x38)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x44, 0x3A)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x45, 0x38)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x4A, 0xF)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x4E, 0xC7)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x4F, 0x4F)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x50, 0x29)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x52, 0x1B)
        .unwrap();
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x56, 0x22)
        .unwrap();

    // Configure SD0 voltage.
    i2c::I2c::C5
        .write_byte(i2c::MAX77620_PWR_I2C_ADDR, 0x16, 42)
        .unwrap();

    // Configure and lock PMC scratch registers.
    config_pmc_scratch(pmc);

    // Set super clock burst policy.
    car.sclk_brst_pol
        .set((car.sclk_brst_pol.get() & 0xFFFF_8888) | 0x3333);

    // Initialize SDRAM.
    sdram::init(car, pmc);

    // TODO(Vale): Save SDRAM LP0 parameters.
}
