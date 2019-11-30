//! Nvidia Tegra210 CPU cluster driver.

use register::mmio::ReadWrite;

use crate::{
    clock::{Car, Clock},
    i2c::{Error, I2c, MAX77620_PWR_I2C_ADDR, MAX77621_CPU_I2C_ADDR},
    pmc::Pmc,
    sysreg::*,
    timer::usleep,
};

/// Base address for Flow Control registers.
const FLOW_CTLR_BASE: u32 = 0x6000_7000;

register!(HALT_COP_EVENTS_0, FLOW_CTLR_BASE + 0x004);

register!(RAM_REPAIR_0, FLOW_CTLR_BASE + 0x040);

register!(FLOW_DBG_QUAL_0, FLOW_CTLR_BASE + 0x050);

register!(L2FLUSH_CONTROL_0, FLOW_CTLR_BASE + 0x094);

register!(BPMP_CLUSTER_CONTROL_0, FLOW_CTLR_BASE + 0x098);

fn try_enable_power() -> Result<(), Error> {
    let mut value = I2c::C5.read_byte(MAX77620_PWR_I2C_ADDR, 0x40)?;

    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x40, value & 0xDF)?;
    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x3B, 0x9)?;

    // Enable power.
    I2c::C5.write_byte(MAX77621_CPU_I2C_ADDR, 0x2, 0x20)?;
    I2c::C5.write_byte(MAX77621_CPU_I2C_ADDR, 0x3, 0x8D)?;
    I2c::C5.write_byte(MAX77621_CPU_I2C_ADDR, 0, 0xB7)?;
    I2c::C5.write_byte(MAX77621_CPU_I2C_ADDR, 0x1, 0xB7)
}

fn enable_power() {
    try_enable_power().unwrap();
}

fn enable_pmc_partition(partition: u32, toggle: u32) -> Result<(), ()> {
    let pmc = &Pmc::new();

    // Check if the partition has already been turned on.
    if pmc.pwrgate_status.get() & partition != 0 {
        return Ok(());
    }

    let mut i = 5001;
    while pmc.pwrgate_toggle.get() & 0x100 != 0 {
        usleep(1);
        i -= 1;

        if i < 1 {
            return Err(());
        }
    }

    pmc.pwrgate_toggle.set(toggle | 0x100);

    i = 5001;
    while i > 0 {
        if pmc.pwrgate_status.get() & partition != 0 {
            break;
        }

        usleep(1);
        i -= 1;
    }

    Ok(())
}

/// Boots the CPU0 of the device.
pub fn boot_cpu0(entry: u32) {
    let car = &Car::new();

    // Set ACTIVE_CLUSTER to FAST.
    BPMP_CLUSTER_CONTROL_0.set(BPMP_CLUSTER_CONTROL_0.get() & 0xFFFF_FFFE);

    enable_power();

    if car.pllx_base.get() & 0x4000_0000 == 0 {
        car.pllx_misc3.set(car.pllx_misc3.get() & 0xFFFF_FFF7);
        usleep(2);
        car.pllx_base.set(0x8040_4E02);
        car.pllx_base.set(0x404E02);
        car.pllx_misc
            .set((car.pllx_misc.get() & 0xFFFB_FFFF) | 0x40000);
        car.pllx_base.set(0x4040_4E02);
    }

    while car.pllx_base.get() & 0x8000000 == 0 {
        // Wait.
    }

    // Configure MSELECT source and enable clock.
    car.clk_source_mselect
        .set((car.clk_source_mselect.get() & 0x1FFF_FF00) | 6);
    car.clk_out_enb_v
        .set((car.clk_out_enb_v.get() & 0xFFFF_FFF7) | 8);

    // Configure initial CPU clock frequency and enable clock.
    car.cclk_brst_pol.set(0x2000_8888);
    car.super_cclk_div.set(0x8000_0000);
    car.clk_enb_v_set.set(1);

    Clock::CORESIGHT.enable();

    // CAR2PMC_CPU_ACK_WIDTH should be set to 0.
    car.cpu_softrst_ctrl2
        .set(car.cpu_softrst_ctrl2.get() & 0xFFFF_F000);

    // Enable CPU rail.
    enable_pmc_partition(1, 0).unwrap();

    // Enable cluster 0 non-CPU.
    enable_pmc_partition(0x8000, 15).unwrap();

    // Enable CE0.
    enable_pmc_partition(0x4000, 14).unwrap();

    // Request and wait for RAM repair.
    RAM_REPAIR_0.set(1);
    while RAM_REPAIR_0.get() & 2 == 0 {
        // Wait.
    }

    unsafe {
        (*((EXCEPTION_VECTOR_BASE + 0x100) as *const ReadWrite<u32>)).set(0);
    }

    // Set reset vector.
    SB_AA64_RESET_LOW_0.set(entry | 1);
    SB_AA64_RESET_HIGH_0.set(0);

    // Non-secure reset vector write disable.
    SB_CSR_0.set(2);
    SB_CSR_0.get();

    // Set CPU_STRICT_TZ_APERTURE_CHECK.
    // TODO(Vale): Should we do this?

    // Clear MSELECT reset.
    car.rst_dev_v.set(car.rst_dev_v.get() & 0xFFFF_FFF7);

    // Clear NONCPU reset.
    car.rst_cpug_cmplx_clr.set(0x2000_0000);

    // Clear CPU0 reset.
    car.rst_cpug_cmplx_clr.set(0x4101_0001);
}
