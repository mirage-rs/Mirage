//! Tegra210 Fuse implementation.

use mirage_mmio::{Mmio, VolatileStorage};

use crate::{clock::Car, timer::usleep};

/// Representation of the Fuse registers.
#[repr(C)]
pub struct Fuse {
    pub ctrl: Mmio<u32>,
    pub reg_addr: Mmio<u32>,
    pub reg_read: Mmio<u32>,
    pub reg_write: Mmio<u32>,
    pub time_rd1: Mmio<u32>,
    pub time_rd2: Mmio<u32>,
    pub time_pgm1: Mmio<u32>,
    pub time_pgm2: Mmio<u32>,
    pub priv2intfc: Mmio<u32>,
    pub fusebypass: Mmio<u32>,
    pub privatekeydisable: Mmio<u32>,
    pub dis_pgm: Mmio<u32>,
    pub write_access: Mmio<u32>,
    pub pwr_good_sw: Mmio<u32>,
    _0x38: [Mmio<u32>; 0x32],
}

impl VolatileStorage for Fuse {
    unsafe fn make_ptr() -> *const Self {
        0x7000_F800 as *const _
    }
}

/// Representation of the Fuse chip.
#[repr(C)]
pub struct FuseChip {
    pub production_mode: Mmio<u32>,
    _0x4: Mmio<u32>,
    _0x8: Mmio<u32>,
    _0xc: Mmio<u32>,
    pub sku_info: Mmio<u32>,
    pub cpu_speedo_0: Mmio<u32>,
    pub cpu_iddq: Mmio<u32>,
    _0x1c: Mmio<u32>,
    _0x20: Mmio<u32>,
    _0x24: Mmio<u32>,
    pub ft_rev: Mmio<u32>,
    pub cpu_speedo_1: Mmio<u32>,
    pub cpu_speedo_2: Mmio<u32>,
    pub soc_speedo_0: Mmio<u32>,
    pub soc_speedo_1: Mmio<u32>,
    pub soc_speedo_2: Mmio<u32>,
    pub soc_iddq: Mmio<u32>,
    _0x44: Mmio<u32>,
    pub fa: Mmio<u32>,
    _0x4c: Mmio<u32>,
    _0x50: Mmio<u32>,
    _0x54: Mmio<u32>,
    _0x58: Mmio<u32>,
    _0x5c: Mmio<u32>,
    _0x60: Mmio<u32>,
    pub public_key: [Mmio<u32>; 0x8],
    pub tsensor_1: Mmio<u32>,
    pub tsensor_2: Mmio<u32>,
    _0x8c: Mmio<u32>,
    pub cp_rev: Mmio<u32>,
    _0x84: Mmio<u32>,
    pub tsensor_0: Mmio<u32>,
    pub first_bootrom_patch_size_reg: Mmio<u32>,
    pub security_mode: Mmio<u32>,
    pub private_key: [Mmio<u32>; 0x4],
    device_key: Mmio<u32>,
    _0xb8: Mmio<u32>,
    _0xbc: Mmio<u32>,
    pub reserved_sw: Mmio<u32>,
    pub vp8_enable: Mmio<u32>,
    pub reserved_odm: [Mmio<u32>; 0x8],
    _0xe8: Mmio<u32>,
    _0xec: Mmio<u32>,
    pub sku_usb_calib: Mmio<u32>,
    pub sku_direct_config: Mmio<u32>,
    _0xf8: Mmio<u32>,
    _0xfc: Mmio<u32>,
    pub vendor_code: Mmio<u32>,
    pub fab_code: Mmio<u32>,
    pub lot_code_0: Mmio<u32>,
    pub lot_code_1: Mmio<u32>,
    pub wafer_id: Mmio<u32>,
    pub x_coordinate: Mmio<u32>,
    pub y_coordinate: Mmio<u32>,
    _0x11c: Mmio<u32>,
    _0x120: Mmio<u32>,
    pub sata_calib: Mmio<u32>,
    pub gpu_iddq: Mmio<u32>,
    pub tsensor_3: Mmio<u32>,
    _0x130: Mmio<u32>,
    _0x134: Mmio<u32>,
    _0x138: Mmio<u32>,
    _0x13c: Mmio<u32>,
    _0x140: Mmio<u32>,
    _0x144: Mmio<u32>,
    pub opt_subrevision: Mmio<u32>,
    _0x14c: Mmio<u32>,
    _0x150: Mmio<u32>,
    pub tsensor_4: Mmio<u32>,
    pub tsensor_5: Mmio<u32>,
    pub tsensor_6: Mmio<u32>,
    pub tsensor_7: Mmio<u32>,
    pub opt_priv_sec_dis: Mmio<u32>,
    pub pkc_disable: Mmio<u32>,
    _0x16c: Mmio<u32>,
    _0x170: Mmio<u32>,
    _0x174: Mmio<u32>,
    _0x178: Mmio<u32>,
    _0x17c: Mmio<u32>,
    pub tsensor_common: Mmio<u32>,
    _0x184: Mmio<u32>,
    _0x188: Mmio<u32>,
    _0x18c: Mmio<u32>,
    _0x190: Mmio<u32>,
    _0x194: Mmio<u32>,
    _0x198: Mmio<u32>,
    pub debug_auth_override: Mmio<u32>,
    _0x1a0: Mmio<u32>,
    _0x1a4: Mmio<u32>,
    _0x1a8: Mmio<u32>,
    _0x1ac: Mmio<u32>,
    _0x1b0: Mmio<u32>,
    _0x1b4: Mmio<u32>,
    _0x1b8: Mmio<u32>,
    _0x1bc: Mmio<u32>,
    _0x1d0: Mmio<u32>,
    pub tsensor_8: Mmio<u32>,
    _0x1d8: Mmio<u32>,
    _0x1dc: Mmio<u32>,
    _0x1e0: Mmio<u32>,
    _0x1e4: Mmio<u32>,
    _0x1e8: Mmio<u32>,
    _0x1ec: Mmio<u32>,
    _0x1f0: Mmio<u32>,
    _0x1f4: Mmio<u32>,
    _0x1f8: Mmio<u32>,
    _0x1fc: Mmio<u32>,
    _0x200: Mmio<u32>,
    pub reserved_calib: Mmio<u32>,
    _0x208: Mmio<u32>,
    _0x20c: Mmio<u32>,
    _0x210: Mmio<u32>,
    _0x214: Mmio<u32>,
    _0x218: Mmio<u32>,
    pub tsensor_9: Mmio<u32>,
    _0x220: Mmio<u32>,
    _0x224: Mmio<u32>,
    _0x228: Mmio<u32>,
    _0x22c: Mmio<u32>,
    _0x230: Mmio<u32>,
    _0x234: Mmio<u32>,
    _0x238: Mmio<u32>,
    _0x23c: Mmio<u32>,
    _0x240: Mmio<u32>,
    _0x244: Mmio<u32>,
    _0x248: Mmio<u32>,
    _0x24c: Mmio<u32>,
    pub usb_calib_ext: Mmio<u32>,
    _0x254: Mmio<u32>,
    _0x258: Mmio<u32>,
    _0x25c: Mmio<u32>,
    _0x260: Mmio<u32>,
    _0x264: Mmio<u32>,
    _0x268: Mmio<u32>,
    _0x26c: Mmio<u32>,
    _0x270: Mmio<u32>,
    _0x274: Mmio<u32>,
    _0x278: Mmio<u32>,
    _0x27c: Mmio<u32>,
    pub spare_bit: [Mmio<u32>; 0x20],
}

impl VolatileStorage for FuseChip {
    unsafe fn make_ptr() -> *const Self {
        0x7000_F900 as *const _
    }
}

/// Initializes the fuse driver.
pub fn init() {
    make_registers_visible(true);
    disable_secondary_private_key();
    disable_programming();
}

/// Whether or not the fuse registers should be made visible.
pub fn make_registers_visible(make_visible: bool) {
    let car = unsafe { Car::get() };

    car.misc_clk_enb.write(
        (car.misc_clk_enb.read() & 0xEFFF_FFFF) | ((if make_visible { 1 } else { 0 } & 1) << 28),
    );
}

/// Disables all fuse programming.
pub fn disable_programming() {
    let fuse = unsafe { Fuse::get() };

    fuse.dis_pgm.write(1);
}

pub fn disable_secondary_private_key() {
    let fuse = unsafe { Fuse::get() };

    fuse.privatekeydisable.write(0x10);
}

/// Wait for the fuse driver to enter an idle state.
pub fn wait_idle() {
    let fuse = unsafe { Fuse::get() };

    // Wait for STATE_IDLE.
    let mut ctrl = 0;
    while (ctrl & 0xF0000) != 0x40000 {
        usleep(1);
        ctrl = fuse.ctrl.read();
    }
}

/// Reads a fuse from the hardware array.
pub fn hardware_read(address: u32) -> u32 {
    let fuse = unsafe { Fuse::get() };
    wait_idle();

    // Program the target address.
    fuse.reg_addr.write(address);

    // Enable read operation in control register.
    let mut ctrl = fuse.ctrl.read();
    ctrl &= !0x3;
    ctrl |= 0x1; // Set FUSE_READ command.
    fuse.ctrl.write(ctrl);

    wait_idle();

    fuse.reg_read.read()
}

/// Writes a fuse to the hardware array.
pub fn hardware_write(address: u32, value: u32) {
    let fuse = unsafe { Fuse::get() };
    wait_idle();

    // Program the target address and value.
    fuse.reg_addr.write(address);
    fuse.reg_write.write(value);

    // Enable write operation in control register.
    let mut ctrl = fuse.ctrl.read();
    ctrl &= !0x3;
    ctrl |= 0x2; // Set FUSE_WRITE command.
    fuse.ctrl.write(ctrl);

    wait_idle();
}

/// Senses the fuse hardware array into the shadow cache.
pub fn hardware_sense() {
    let fuse = unsafe { Fuse::get() };
    wait_idle();

    // Enable sense operation in control register.
    let mut ctrl = fuse.ctrl.read();
    ctrl &= !0x3;
    ctrl |= 0x3; // Set FUSE_SENSE command.
    fuse.ctrl.write(ctrl);

    wait_idle();
}

/// Reads the SKU info register from the shadow cache.
pub fn read_sku_info() -> u32 {
    let fuse_chip = unsafe { FuseChip::get() };

    fuse_chip.sku_info.read()
}

/// Reads the bootrom patch version from a register in the shadow cache.
pub fn read_bootrom_patch_version() -> u32 {
    let fuse_chip = unsafe { FuseChip::get() };

    fuse_chip.soc_speedo_1.read()
}

/// Reads a spare bit register from the shadow cache.
pub fn read_spare_bit(index: usize) -> u32 {
    let fuse_chip = unsafe { FuseChip::get() };

    if index < 32 {
        return fuse_chip.spare_bit[index].read();
    } else {
        return 0;
    }
}

/// Reads a reserved ODM register from the shadow cache.
pub fn read_reserved_odm(index: usize) -> u32 {
    let fuse_chip = unsafe { FuseChip::get() };

    if index < 8 {
        return fuse_chip.reserved_odm[index].read();
    } else {
        return 0;
    }
}

/// Retrieves the Device ID from the shadow cache.
pub fn get_device_id() -> u64 {
    let fuse_chip = unsafe { FuseChip::get() };
    let mut device_id = 0;

    let y_coord = fuse_chip.y_coordinate.read() & 0x1FF;
    let x_coord = fuse_chip.x_coordinate.read() & 0x1FF;
    let wafer_id = fuse_chip.wafer_id.read() & 0x3F;
    let lot_code = fuse_chip.lot_code_0.read();
    let fab_code = fuse_chip.fab_code.read() & 0x3F;

    let mut derived_lot_code = 0;
    for i in 0..5 {
        derived_lot_code = (derived_lot_code * 0x24) + ((lot_code >> (24 - 6 * i)) & 0x3F);
    }
    derived_lot_code &= 0x03FF_FFFF;

    device_id |= (y_coord as u64) << 0;
    device_id |= (x_coord as u64) << 9;
    device_id |= (wafer_id as u64) << 18;
    device_id |= (derived_lot_code as u64) << 24;
    device_id |= (fab_code as u64) << 50;

    device_id
}
