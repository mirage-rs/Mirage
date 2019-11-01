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
pub mod sysctr0;
pub mod timer;
pub mod tsec;
pub mod uart;

fn config_oscillators() {
    let pmc = &pmc::Pmc::new();

    // Set CLK_M_DIVISOR to 2.
    clock::SPARE_REG0.set((clock::SPARE_REG0.get() & 0xFFFF_FFF3) | 4);
    // Set counter frequency.
    sysctr0::CNTFID0.set(19200000);
    // For 19.2MHz clk_m.
    timer::TIMERUS_USEC_CFG.set(0x45F);

    // Set OSC to 38.4MHz and drive strength.
    clock::OSC_CTRL.set(0x5000_0071);

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
    clock::CLK_SYSTEM_RATE.set(0x10);
    // Disable PLLMB.
    clock::PLLMB_BASE.set(clock::PLLMB_BASE.get() & 0xBFFF_FFFF);

    pmc.tsc_mult
        .set((pmc.tsc_mult.get() & 0xFFFF_0000) | 0x249F); //0x249F = 19200000 * (16 / 32.768 kHz)

    // Set SCLK div to 1.
    clock::CLK_SOURCE_SYS.set(0);
    // Set clk source to Run and PLLP_OUT2 (204MHz).
    clock::SCLK_BURST_POLICY.set(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    clock::SCLK_DIVIDER.set(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    clock::CLK_SYSTEM_RATE.set(2);
}

fn config_gpios() {
    let pinmux = &pinmux::Pinmux::new();

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

    pinmux::configure_i2c(i2c::I2cDevice::I1);
    pinmux::configure_i2c(i2c::I2cDevice::I5);
    pinmux::configure_uart(pinmux, &uart::Uart::A);

    // Configure Volume Up/Down as inputs.
    gpio::GpioPin::BUTTON_VOL_UP.config(gpio::GpioConfig::Input);
    gpio::GpioPin::BUTTON_VOL_DOWN.config(gpio::GpioConfig::Input);
}

fn config_pmc_scratch() {
    let pmc = &pmc::Pmc::new();

    pmc.scratch20.set(pmc.scratch20.get() & 0xFFF3_FFFF);
    pmc.scratch190.set(pmc.scratch190.get() & 0xFFFF_FFFE);
    pmc.secure_sratch21.set(pmc.secure_scratch21.get() | 0x10);
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
