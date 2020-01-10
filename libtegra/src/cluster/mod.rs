//! Nvidia Tegra210 CPU cluster driver.

use mirage_mmio::{Mmio, VolatileStorage};

use crate::{
    clock::{Car, Clock},
    i2c::{Error, I2c, Device},
    pmc::Pmc,
    sysreg::{SbRegisters, EXCEPTION_VECTOR_BASE},
    timer::usleep,
};

/// Base address for Flow Control registers.
pub(crate) const FLOW_CTLR_BASE: u32 = 0x6000_7000;

fn try_enable_power() -> Result<(), Error> {
    let value = I2c::C5.read_byte(Device::Max77620Pwr, 0x40)?;

    I2c::C5.write_byte(Device::Max77620Pwr, 0x40, value & 0xDF)?;
    I2c::C5.write_byte(Device::Max77620Pwr, 0x3B, 0x9)?;

    // Enable power.
    I2c::C5.write_byte(Device::Max77621Cpu, 0x2, 0x20)?;
    I2c::C5.write_byte(Device::Max77621Cpu, 0x3, 0x8D)?;
    I2c::C5.write_byte(Device::Max77621Cpu, 0, 0xB7)?;
    I2c::C5.write_byte(Device::Max77621Cpu, 0x1, 0xB7)
}

fn enable_power() {
    try_enable_power().unwrap();
}

fn enable_pmc_partition(partition: u32, toggle: u32) -> Result<(), ()> {
    let pmc = unsafe { Pmc::get() };

    // Check if the partition has already been turned on.
    if pmc.pwrgate_status.read() & partition != 0 {
        return Ok(());
    }

    let mut i = 5001;
    while pmc.pwrgate_toggle.read() & 0x100 != 0 {
        usleep(1);
        i -= 1;

        if i < 1 {
            return Err(());
        }
    }

    pmc.pwrgate_toggle.write(toggle | 0x100);

    i = 5001;
    while i > 0 {
        if pmc.pwrgate_status.read() & partition != 0 {
            break;
        }

        usleep(1);
        i -= 1;
    }

    Ok(())
}

/// Boots the CPU0 of the device.
pub fn boot_cpu0(entry: u32) {
    let car = unsafe { Car::get() };
    let sb = unsafe { SbRegisters::get() };

    let ram_repair = unsafe {
        &*((FLOW_CTLR_BASE + 0x040) as *const Mmio<u32>)
    };

    let bpmp_cluster_control = unsafe {
        &*((FLOW_CTLR_BASE + 0x098) as *const Mmio<u32>)
    };

    // Set ACTIVE_CLUSTER to FAST.
    bpmp_cluster_control.write(bpmp_cluster_control.read() & 0xFFFF_FFFE);

    enable_power();

    if car.pllx_base.read() & 0x4000_0000 == 0 {
        car.pllx_misc3.write(car.pllx_misc3.read() & 0xFFFF_FFF7);
        usleep(2);
        car.pllx_base.write(0x8040_4E02);
        car.pllx_base.write(0x404E02);
        car.pllx_misc.write((car.pllx_misc.read() & 0xFFFB_FFFF) | 0x40000);
        car.pllx_base.write(0x4040_4E02);
    }

    while car.pllx_base.read() & 0x8000000 == 0 {
        // Wait.
    }

    // Configure MSELECT source and enable clock.
    car.clk_source_mselect.write((car.clk_source_mselect.read() & 0x1FFF_FF00) | 6);
    car.clk_out_enb_v.write((car.clk_out_enb_v.read() & 0xFFFF_FFF7) | 8);

    // Configure initial CPU clock frequency and enable clock.
    car.cclk_brst_pol.write(0x2000_8888);
    car.super_cclk_div.write(0x8000_0000);
    car.clk_enb_v_set.write(1);

    Clock::CORESIGHT.enable();

    // CAR2PMC_CPU_ACK_WIDTH should be set to 0.
    car.cpu_softrst_ctrl2.write(car.cpu_softrst_ctrl2.read() & 0xFFFF_F000);

    // Enable CPU rail.
    enable_pmc_partition(1, 0).unwrap();

    // Enable cluster 0 non-CPU.
    enable_pmc_partition(0x8000, 15).unwrap();

    // Enable CE0.
    enable_pmc_partition(0x4000, 14).unwrap();

    // Request and wait for RAM repair.
    ram_repair.write(1);
    while ram_repair.read() & 2 == 0 {
        // Wait.
    }

    unsafe {
        (*((EXCEPTION_VECTOR_BASE + 0x100) as *const Mmio<u32>)).write(0);
    }

    // Set reset vector.
    sb.AA64_RESET_LOW.write(entry | 1);
    sb.AA64_RESET_HIGH.write(0);

    // Non-secure reset vector write disable.
    sb.CSR.write(2);
    sb.CSR.read();

    // Set CPU_STRICT_TZ_APERTURE_CHECK.
    // TODO(Vale): Should we do this?

    // Clear MSELECT reset.
    car.rst_dev_v.write(car.rst_dev_v.read() & 0xFFFF_FFF7);

    // Clear NONCPU reset.
    car.rst_cpug_cmplx_clr.write(0x2000_0000);

    // Clear CPU0 reset.
    car.rst_cpug_cmplx_clr.write(0x4101_0001);
}
