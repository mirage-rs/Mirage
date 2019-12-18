//! Tegra210 Fuse implementation.

use mirage_mmio::{BlockMmio, VolatileStorage};

use crate::{clock::Car, timer::usleep};

/// Representation of the Fuse registers.
#[repr(C)]
pub struct Fuse {
    pub ctrl: BlockMmio<u32>,
    pub reg_addr: BlockMmio<u32>,
    pub reg_read: BlockMmio<u32>,
    pub reg_write: BlockMmio<u32>,
    pub time_rd1: BlockMmio<u32>,
    pub time_rd2: BlockMmio<u32>,
    pub time_pgm1: BlockMmio<u32>,
    pub time_pgm2: BlockMmio<u32>,
    pub priv2intfc: BlockMmio<u32>,
    pub fusebypass: BlockMmio<u32>,
    pub privatekeydisable: BlockMmio<u32>,
    pub dis_pgm: BlockMmio<u32>,
    pub write_access: BlockMmio<u32>,
    pub pwr_good_sw: BlockMmio<u32>,
    _0x38: [BlockMmio<u32>; 0x32],
}

impl VolatileStorage for Fuse {
    unsafe fn make_ptr() -> *const Self {
        0x7000_F800 as *const _
    }
}

/// Representation of the Fuse chip.
#[repr(C)]
pub struct FuseChip {
    pub production_mode: BlockMmio<u32>,
    _0x4: BlockMmio<u32>,
    _0x8: BlockMmio<u32>,
    _0xc: BlockMmio<u32>,
    pub sku_info: BlockMmio<u32>,
    pub cpu_speedo_0: BlockMmio<u32>,
    pub cpu_iddq: BlockMmio<u32>,
    _0x1c: BlockMmio<u32>,
    _0x20: BlockMmio<u32>,
    _0x24: BlockMmio<u32>,
    pub ft_rev: BlockMmio<u32>,
    pub cpu_speedo_1: BlockMmio<u32>,
    pub cpu_speedo_2: BlockMmio<u32>,
    pub soc_speedo_0: BlockMmio<u32>,
    pub soc_speedo_1: BlockMmio<u32>,
    pub soc_speedo_2: BlockMmio<u32>,
    pub soc_iddq: BlockMmio<u32>,
    _0x44: BlockMmio<u32>,
    pub fa: BlockMmio<u32>,
    _0x4c: BlockMmio<u32>,
    _0x50: BlockMmio<u32>,
    _0x54: BlockMmio<u32>,
    _0x58: BlockMmio<u32>,
    _0x5c: BlockMmio<u32>,
    _0x60: BlockMmio<u32>,
    pub public_key: [BlockMmio<u32>; 0x8],
    pub tsensor_1: BlockMmio<u32>,
    pub tsensor_2: BlockMmio<u32>,
    _0x8c: BlockMmio<u32>,
    pub cp_rev: BlockMmio<u32>,
    _0x84: BlockMmio<u32>,
    pub tsensor_0: BlockMmio<u32>,
    pub first_bootrom_patch_size_reg: BlockMmio<u32>,
    pub security_mode: BlockMmio<u32>,
    pub private_key: [BlockMmio<u32>; 0x4],
    device_key: BlockMmio<u32>,
    _0xb8: BlockMmio<u32>,
    _0xbc: BlockMmio<u32>,
    pub reserved_sw: BlockMmio<u32>,
    pub vp8_enable: BlockMmio<u32>,
    pub reserved_odm: [BlockMmio<u32>; 0x8],
    _0xe8: BlockMmio<u32>,
    _0xec: BlockMmio<u32>,
    pub sku_usb_calib: BlockMmio<u32>,
    pub sku_direct_config: BlockMmio<u32>,
    _0xf8: BlockMmio<u32>,
    _0xfc: BlockMmio<u32>,
    pub vendor_code: BlockMmio<u32>,
    pub fab_code: BlockMmio<u32>,
    pub lot_code_0: BlockMmio<u32>,
    pub lot_code_1: BlockMmio<u32>,
    pub wafer_id: BlockMmio<u32>,
    pub x_coordinate: BlockMmio<u32>,
    pub y_coordinate: BlockMmio<u32>,
    _0x11c: BlockMmio<u32>,
    _0x120: BlockMmio<u32>,
    pub sata_calib: BlockMmio<u32>,
    pub gpu_iddq: BlockMmio<u32>,
    pub tsensor_3: BlockMmio<u32>,
    _0x130: BlockMmio<u32>,
    _0x134: BlockMmio<u32>,
    _0x138: BlockMmio<u32>,
    _0x13c: BlockMmio<u32>,
    _0x140: BlockMmio<u32>,
    _0x144: BlockMmio<u32>,
    pub opt_subrevision: BlockMmio<u32>,
    _0x14c: BlockMmio<u32>,
    _0x150: BlockMmio<u32>,
    pub tsensor_4: BlockMmio<u32>,
    pub tsensor_5: BlockMmio<u32>,
    pub tsensor_6: BlockMmio<u32>,
    pub tsensor_7: BlockMmio<u32>,
    pub opt_priv_sec_dis: BlockMmio<u32>,
    pub pkc_disable: BlockMmio<u32>,
    _0x16c: BlockMmio<u32>,
    _0x170: BlockMmio<u32>,
    _0x174: BlockMmio<u32>,
    _0x178: BlockMmio<u32>,
    _0x17c: BlockMmio<u32>,
    pub tsensor_common: BlockMmio<u32>,
    _0x184: BlockMmio<u32>,
    _0x188: BlockMmio<u32>,
    _0x18c: BlockMmio<u32>,
    _0x190: BlockMmio<u32>,
    _0x194: BlockMmio<u32>,
    _0x198: BlockMmio<u32>,
    pub debug_auth_override: BlockMmio<u32>,
    _0x1a0: BlockMmio<u32>,
    _0x1a4: BlockMmio<u32>,
    _0x1a8: BlockMmio<u32>,
    _0x1ac: BlockMmio<u32>,
    _0x1b0: BlockMmio<u32>,
    _0x1b4: BlockMmio<u32>,
    _0x1b8: BlockMmio<u32>,
    _0x1bc: BlockMmio<u32>,
    _0x1d0: BlockMmio<u32>,
    pub tsensor_8: BlockMmio<u32>,
    _0x1d8: BlockMmio<u32>,
    _0x1dc: BlockMmio<u32>,
    _0x1e0: BlockMmio<u32>,
    _0x1e4: BlockMmio<u32>,
    _0x1e8: BlockMmio<u32>,
    _0x1ec: BlockMmio<u32>,
    _0x1f0: BlockMmio<u32>,
    _0x1f4: BlockMmio<u32>,
    _0x1f8: BlockMmio<u32>,
    _0x1fc: BlockMmio<u32>,
    _0x200: BlockMmio<u32>,
    pub reserved_calib: BlockMmio<u32>,
    _0x208: BlockMmio<u32>,
    _0x20c: BlockMmio<u32>,
    _0x210: BlockMmio<u32>,
    _0x214: BlockMmio<u32>,
    _0x218: BlockMmio<u32>,
    pub tsensor_9: BlockMmio<u32>,
    _0x220: BlockMmio<u32>,
    _0x224: BlockMmio<u32>,
    _0x228: BlockMmio<u32>,
    _0x22c: BlockMmio<u32>,
    _0x230: BlockMmio<u32>,
    _0x234: BlockMmio<u32>,
    _0x238: BlockMmio<u32>,
    _0x23c: BlockMmio<u32>,
    _0x240: BlockMmio<u32>,
    _0x244: BlockMmio<u32>,
    _0x248: BlockMmio<u32>,
    _0x24c: BlockMmio<u32>,
    pub usb_calib_ext: BlockMmio<u32>,
    _0x254: BlockMmio<u32>,
    _0x258: BlockMmio<u32>,
    _0x25c: BlockMmio<u32>,
    _0x260: BlockMmio<u32>,
    _0x264: BlockMmio<u32>,
    _0x268: BlockMmio<u32>,
    _0x26c: BlockMmio<u32>,
    _0x270: BlockMmio<u32>,
    _0x274: BlockMmio<u32>,
    _0x278: BlockMmio<u32>,
    _0x27c: BlockMmio<u32>,
    pub spare_bit: [BlockMmio<u32>; 0x20],
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
