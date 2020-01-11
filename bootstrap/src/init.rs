use mirage_libtegra::{
    clock::{Car, Clock},
    fuse,
    gpio::{Gpio, GpioConfig},
    i2c::{I2c, Device},
    mc,
    pinmux::{Pinmux, INPUT},
    pmc::Pmc,
    sdram,
    //se::SecurityEngine,
    sysctr0::Sysctr0Registers,
    sysreg::AhbRegisters,
    timer::{TimerRegisters, usleep},
    uart::Uart,
};
use mirage_mmio::{Mmio, VolatileStorage};

/// The global instance of the Security Engine.
//const SECURITY_ENGINE: SecurityEngine = SecurityEngine::new();

/// Base address for I2S registers.
const I2S_BASE: u32 = 0x702D_1000;

/// Configures the Switch oscillators.
fn config_oscillators(car: &Car, pmc: &Pmc) {
    let sysctr0 = unsafe { Sysctr0Registers::get() };
    let timer = unsafe { TimerRegisters::get() };

    // Set CLK_M_DIVISOR to 2.
    car.spare_reg0.write((car.spare_reg0.read() & 0xFFFF_FFF3) | 4);
    // Set counter frequency.
    sysctr0.CNTFID0.write(0x124F800);
    // For 19.2MHz clk_m.
    timer.TIMERUS_USEC_CFG.write(0x45F);
    // Set OSC to 38.4MHz and drive strength.
    car.osc_ctrl.write(0x5000_0071);

    // Set LP0 OSC drive strength.
    pmc.osc_edpd_over.write((pmc.osc_edpd_over.read() & 0xFFFF_FF81) | 0xE);
    pmc.osc_edpd_over.write((pmc.osc_edpd_over.read() & 0xFFBF_FFFF) | 0x400000);
    pmc.cntrl2.write((pmc.cntrl2.read() & 0xFFFF_EFFF) | 0x1000);
    // LP0 EMC2TMC_CFG_XM2COMP_PU_VREF_SEL_RANGE.
    pmc.scratch188.write((pmc.scratch188.read() & 0xFCFF_FFFF) | 0x2000000);

    // Set HCLK div to 2 and PCLK div to 1.
    car.clk_sys_rate.write(0x10);
    // Disable PLLMB.
    car.pllmb_base.write(car.pllmb_base.read() & 0xBFFF_FFFF);

    // 0x249F = 19200000 * (16 / 32.768 kHz)
    pmc.tsc_mult.write((pmc.tsc_mult.read() & 0xFFFF_0000) | 0x249F);

    // Set SCLK div to 1.
    car.clk_source_sys.write(0);
    // Set clk source to Run and PLLP_OUT2 (204MHz).
    car.sclk_brst_pol.write(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    car.super_sclk_div.write(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    car.clk_sys_rate.write(2);
}

/// Configures the GPIOs used by the Switch.
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

/// Configures and locks the PMC scratch registers.
fn config_pmc_scratch(pmc: &Pmc) {
    pmc.scratch20.write(pmc.scratch20.read() & 0xFFF3_FFFF);
    pmc.scratch190.write(pmc.scratch190.read() & 0xFFFF_FFFE);
    pmc.secure_scratch21.write(pmc.secure_scratch21.read() | 0x10);
}

fn mbist_workaround(car: &Car) {
    let i2s1_cg = unsafe { &*((I2S_BASE + 0x88) as *const Mmio<u32>) };
    let i2s1_ctrl = unsafe { &*((I2S_BASE + 0xA0) as *const Mmio<u32>) };
    let i2s2_cg = unsafe { &*((I2S_BASE + 0x188) as *const Mmio<u32>) };
    let i2s2_ctrl = unsafe { &*((I2S_BASE + 0x1A0) as *const Mmio<u32>) };
    let i2s3_cg = unsafe { &*((I2S_BASE + 0x288) as *const Mmio<u32>) };
    let i2s3_ctrl = unsafe { &*((I2S_BASE + 0x2A0) as *const Mmio<u32>) };
    let i2s4_cg = unsafe { &*((I2S_BASE + 0x388) as *const Mmio<u32>) };
    let i2s4_ctrl = unsafe { &*((I2S_BASE + 0x3A0) as *const Mmio<u32>) };
    let i2s5_cg = unsafe { &*((I2S_BASE + 0x488) as *const Mmio<u32>) };
    let i2s5_ctrl = unsafe { &*((I2S_BASE + 0x4A0) as *const Mmio<u32>) };

    car.clk_source_sor1.write((car.clk_source_sor1.read() | 0x8000) & 0xFFFF_BFFF);
    car.plld_base.write(car.plld_base.read() | 0x4080_0000);
    car.rst_dev_y_clr.write(0x40);
    car.rst_dev_x_clr.write(0x40000);
    car.rst_dev_l_clr.write(0x1800_0000);
    usleep(2);

    // Setup I2S.
    i2s1_ctrl.write(i2s1_ctrl.read() | 0x400);
    i2s1_cg.write(i2s1_cg.read() & 0xFFFF_FFFE);
    i2s2_ctrl.write(i2s2_ctrl.read() | 0x400);
    i2s2_cg.write(i2s2_cg.read() & 0xFFFF_FFFE);
    i2s3_ctrl.write(i2s3_ctrl.read() | 0x400);
    i2s3_cg.write(i2s3_cg.read() & 0xFFFF_FFFE);
    i2s4_ctrl.write(i2s4_ctrl.read() | 0x400);
    i2s4_cg.write(i2s4_cg.read() & 0xFFFF_FFFE);
    i2s5_ctrl.write(i2s5_ctrl.read() | 0x400);
    i2s5_cg.write(i2s5_cg.read() & 0xFFFF_FFFE);

    unsafe {
        let dc_com_dsc_top_ctl = &*((0x5420_0000 + 0x33E * 4) as *const Mmio<u32>);
        dc_com_dsc_top_ctl.write(dc_com_dsc_top_ctl.read() | 4);
        (*((0x5434_0000 + 0x8C) as *const Mmio<u32>)).write(0xFFFF_FFFF);
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

/// Initializes the Switch hardware in an early bootrom context.
pub fn hwinit() {
    let ahb = unsafe { AhbRegisters::get() };
    let car = unsafe { Car::get() };
    let pinmux = unsafe { Pinmux::get() };
    let pmc = unsafe { Pmc::get() };

    // TODO(Vale): Implement this.
    // Bootrom stuff that was skipped by going through RCM.
    // config_se_brom(pmc);

    ahb.AHB_SPARE_REG.write(ahb.AHB_SPARE_REG.read() & 0xFFFF_FF9F);
    pmc.scratch49.write(pmc.scratch49.read() & 0xFFFF_FFFC);

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
        (*((0x7000_0000 + 0x40) as *const Mmio<u32>)).write(0);
    }

    // Configure GPIOs.
    config_gpios(pinmux);

    #[cfg(feature = "debug_uart_port")]
    Uart::E.init(115_200);

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
        .write_byte(Device::Max77620Pwr, 0x4, 0x40)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x41, 0x60)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x43, 0x38)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x44, 0x3A)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x45, 0x38)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x4A, 0xF)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x4E, 0xC7)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x4F, 0x4F)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x50, 0x29)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x52, 0x1B)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x56, 0x22)
        .unwrap();

    // Configure SD0 voltage.
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x16, 42)
        .unwrap();

    // Configure and lock PMC scratch registers.
    // XXX: This was removed from 4.x ongoing, should this be done?
    config_pmc_scratch(pmc);

    // Set super clock burst policy to PLLP_OUT (408MHz).
    car.sclk_brst_pol.write((car.sclk_brst_pol.read() & 0xFFFF_8888) | 0x3333);

    // Initialize SDRAM.
    //sdram::init(car, pmc); --- execution gets stuck here, no panic though

    // TODO(Vale): Save SDRAM LP0 parameters.
}
