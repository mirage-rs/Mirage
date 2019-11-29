//! Abstractions over miscellaneous APB registers.

use core::ops::Deref;

use register::mmio::ReadWrite;

/// Base address for PADCTL registers.
const APB_PADCTL_BASE: u32 = 0x7000_0810;

/// Representation of the PADCTL registers.
#[repr(C)]
pub(crate) struct Registers {
    pub asdbgreg: ReadWrite<u32>,
    _reserved: [ReadWrite<u32>; 0x31],
    pub sdmmc1_clk_lpbk_control: ReadWrite<u32>,
    pub sdmmc3_clk_lpbk_control: ReadWrite<u32>,
    pub emmc2_pad_cfg_control: ReadWrite<u32>,
    pub emmc4_pad_cfg_control: ReadWrite<u32>,
    _unk1: [ReadWrite<u32>; 0x6E],
    pub sdmmc1_pad_cfgpadctrl: ReadWrite<u32>,
    pub emmc2_pad_cfgpadctrl: ReadWrite<u32>,
    pub emmc2_pad_drv_type_cfgpadctrl: ReadWrite<u32>,
    pub emmc2_pad_pupd_cfgpadctrl: ReadWrite<u32>,
    _unk2: [ReadWrite<u32>; 0x3],
    pub sdmmc3_pad_cfgpadctrl: ReadWrite<u32>,
    pub emmc4_pad_cfgpadctrl: ReadWrite<u32>,
    pub emmc4_pad_drv_type_cfgpadctrl: ReadWrite<u32>,
    pub emmc4_pad_pupd_cfgpadctrl: ReadWrite<u32>,
    _unk3: [ReadWrite<u32>; 0x2E],
    pub vgpio_gpio_mux_sel: ReadWrite<u32>,
    pub qspi_sck_lpbk_control: ReadWrite<u32>,
}

impl Registers {
    /// Factory method to create a pointer to the PADCTL registers.
    #[inline]
    pub const fn get() -> *const Self {
        APB_PADCTL_BASE as *const _
    }
}

/// Representation of the Tegra210 PADCTL.
pub(crate) struct Padctl;

impl Padctl {
    /// Creates a new instance of the PADCTL.
    pub fn new() -> Self {
        Padctl
    }
}

impl Deref for Padctl {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Registers::get() }
    }
}
