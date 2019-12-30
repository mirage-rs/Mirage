//! Abstractions over miscellaneous APB registers.

use mirage_mmio::{Mmio, VolatileStorage};

/// Base address for PADCTL registers.
pub(crate) const APB_PADCTL_BASE: u32 = 0x7000_0810;

/// Representation of the PADCTL.
#[repr(C)]
pub struct Padctl {
    pub asdbgreg: Mmio<u32>,
    _reserved: [Mmio<u32>; 0x31],
    pub sdmmc1_clk_lpbk_control: Mmio<u32>,
    pub sdmmc3_clk_lpbk_control: Mmio<u32>,
    pub emmc2_pad_cfg_control: Mmio<u32>,
    pub emmc4_pad_cfg_control: Mmio<u32>,
    _unk1: [Mmio<u32>; 0x6E],
    pub sdmmc1_pad_cfgpadctrl: Mmio<u32>,
    pub emmc2_pad_cfgpadctrl: Mmio<u32>,
    pub emmc2_pad_drv_type_cfgpadctrl: Mmio<u32>,
    pub emmc2_pad_pupd_cfgpadctrl: Mmio<u32>,
    _unk2: [Mmio<u32>; 0x3],
    pub sdmmc3_pad_cfgpadctrl: Mmio<u32>,
    pub emmc4_pad_cfgpadctrl: Mmio<u32>,
    pub emmc4_pad_drv_type_cfgpadctrl: Mmio<u32>,
    pub emmc4_pad_pupd_cfgpadctrl: Mmio<u32>,
    _unk3: [Mmio<u32>; 0x2E],
    pub vgpio_gpio_mux_sel: Mmio<u32>,
    pub qspi_sck_lpbk_control: Mmio<u32>,
}

impl VolatileStorage for Padctl {
    unsafe fn make_ptr() -> *const Self {
        APB_PADCTL_BASE as *const _
    }
}
