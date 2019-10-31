//! Low-level hardware access library for the Nintendo Switch.
//!
//! **Note:** This code is written specifically for the Switch.
//! If you decide to use it for other Tegra210 platforms, use
//! at own risk.

#![no_std]
#![feature(optimize_attr)]

#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate enum_primitive;

extern crate register;

use register::mmio::ReadWrite;

pub mod button;
pub mod clock;
pub mod fuse;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod pinmux;
pub mod pmc;
pub mod timer;
pub mod tsec;
pub mod uart;

fn config_oscillators() {
    let pmc_registers = pmc::PmcRegisters::get();

    unsafe {
        clock::SPARE_REG0.set((clock::SPARE_REG0.get() & 0xFFFF_FFF3) | 4);

        let sysctr0_cntfid0_0_reg = &(*((0x700F_0000 + 0x20) as *const ReadWrite<u32>));
        sysctr0_cntfid0_0_reg.set(19200000);
        let timerus_usec_cfg_0_reg = &(*(0x6000_5014 as *const ReadWrite<u32>));
        timerus_usec_cfg_0_reg.set(0x45F);

        clock::OSC_CTRL.set(0x5000_0071);
        let pmc_osc_edpd_over_reg = &((*pmc_registers).osc_edpd_over);
        pmc_osc_edpd_over_reg.set((pmc_osc_edpd_over_reg.get() & 0xFFFF_FF81) | 0xE);
        pmc_osc_edpd_over_reg.set((pmc_osc_edpd_over_reg.get() & 0xFFBF_FFFF) | 0x400000);
        let pmc_cntrl2_reg = &((*pmc_registers).cntrl2);
        pmc_cntrl2_reg.set((pmc_cntrl2_reg.get() & 0xFFFF_EFFF) | 0x1000);
        let pmc_scratch188_reg = &((*pmc_registers).scratch188);
        pmc_scratch188_reg.set((pmc_scratch188_reg.get() & 0xFCFF_FFFF) | 0x2000000);
        clock::CLK_SYSTEM_RATE.set(0x10);
        clock::PLLMB_BASE.set(clock::PLLMB_BASE.get() & 0xBFFF_FFFF);
        let pmc_tsc_mult_reg = &((*pmc_registers).tsc_mult);
        pmc_tsc_mult_reg.set((pmc_tsc_mult_reg.get() & 0xFFFF_0000) | 0x249F);
        clock::CLK_SOURCE_SYS.set(0);
        clock::SCLK_BURST_POLICY.set(0x2000_4444);
        clock::SCLK_DIVIDER.set(0x8000_0000);
        clock::CLK_SYSTEM_RATE.set(2);
    }
}

fn config_gpios() {
    let pinmux_registers = pinmux::PinmuxRegisters::get();

    let pinmux_uart2_tx_reg = unsafe { &((*pinmux_registers).uart2_tx) };
    pinmux_uart2_tx_reg.set(0);
    let pinmux_uart3_tx_reg = unsafe { &((*pinmux_registers).uart3_tx) };
    pinmux_uart3_tx_reg.set(0);

    // Set Joy-Con IsAttached direction.
    let pinmux_pe6_reg = unsafe { &((*pinmux_registers).pe6) };
    pinmux_pe6_reg.set(pinmux::PINMUX_INPUT);
    let pinmux_ph6_reg = unsafe { &((*pinmux_registers).ph6) };
    pinmux_ph6_reg.set(pinmux::PINMUX_INPUT);

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

    pinmux::configure_i2c(i2c::I2cDevice::I1);
    pinmux::configure_i2c(i2c::I2cDevice::I5);
    pinmux::configure_uart(uart::Uart::A);

    // Configure Volume Up/Down as inputs.
    gpio::GpioPin::BUTTON_VOL_UP.config(gpio::GpioConfig::Input);
    gpio::GpioPin::BUTTON_VOL_DOWN.config(gpio::GpioConfig::Input);
}

fn config_pmc_scratch() {
    let pmc_registers = pmc::PmcRegisters::get();

    unsafe {
        let pmc_scratch20_reg = &((*pmc_registers).scratch20);
        pmc_scratch20_reg.set(pmc_scratch20_reg.get() & 0xFFF3_FFFF);
        let pmc_scratch190_reg = &((*pmc_registers).scratch190);
        pmc_scratch190_reg.set(pmc_scratch190_reg.get() & 0xFFFF_FFFE);
        let pmc_secure_scratch21_reg = &((*pmc_registers).secure_scratch21);
        pmc_secure_scratch21_reg.set(pmc_secure_scratch21_reg.get() | 0x10);
    }
}

fn mbist_workaround() {
    unimplemented!();
}

fn config_se_brom() {
    unimplemented!();
}

/// Initializes the Switch hardware.
pub fn hardware_init() {
    unimplemented!();
}
