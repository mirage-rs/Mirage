use register::mmio::ReadWrite;

/// Base address for SDMMC registers.
const SDMMC_BASE: u32 = 0x700B_0000;

/// Representation of the SDMMC registers.
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    pub dma_address: ReadWrite<u32>,
    pub block_size: ReadWrite<u16>,
    pub block_count: ReadWrite<u16>,
    pub argument: ReadWrite<u32>,
    pub transfer_mode: ReadWrite<u16>,
    pub command: ReadWrite<u16>,
    pub response: [ReadWrite<u32>; 0x4],
    pub buffer: ReadWrite<u32>,
    pub present_state: ReadWrite<u32>,
    pub host_control: ReadWrite<u8>,
    pub power_control: ReadWrite<u8>,
    pub block_gap_control: ReadWrite<u8>,
    pub wake_up_control: ReadWrite<u8>,
    pub clock_control: ReadWrite<u16>,
    pub timeout_control: ReadWrite<u8>,
    pub software_reset: ReadWrite<u8>,
    pub int_status: ReadWrite<u32>,
    pub int_enable: ReadWrite<u32>,
    pub signal_enable: ReadWrite<u32>,
    pub acmd12_err: ReadWrite<u16>,
    pub host_control2: ReadWrite<u16>,
    pub capabilities: ReadWrite<u32>,
    pub capabilities_1: ReadWrite<u32>,
    pub max_current: ReadWrite<u32>,
    _0x4C: ReadWrite<u32>,
    pub set_acmd12_error: ReadWrite<u16>,
    pub set_int_error: ReadWrite<u16>,
    pub adma_error: ReadWrite<u8>,
    _0x56: [ReadWrite<u8>; 0x3],
    pub adma_address: ReadWrite<u32>,
    pub upper_adma_address: ReadWrite<u32>,
    pub preset_for_init: ReadWrite<u16>,
    pub preset_for_default: ReadWrite<u16>,
    pub preset_for_high: ReadWrite<u16>,
    pub preset_for_sdr12: ReadWrite<u16>,
    pub preset_for_sdr25: ReadWrite<u16>,
    pub preset_for_sdr50: ReadWrite<u16>,
    pub preset_for_sdr104: ReadWrite<u16>,
    pub preset_for_ddr50: ReadWrite<u16>,
    _0x70: [ReadWrite<u32>; 0x23],
    pub slot_int_status: ReadWrite<u16>,
    pub host_version: ReadWrite<u16>,

    // Vendor-specific registers.
    pub vendor_clock_cntrl: ReadWrite<u32>,
    pub vendor_sys_sw_cntrl: ReadWrite<u32>,
    pub vendor_err_intr_status: ReadWrite<u32>,
    pub vendor_cap_overrides: ReadWrite<u32>,
    pub vendor_boot_cntrl: ReadWrite<u32>,
    pub vendor_boot_ack_timeout: ReadWrite<u32>,
    pub vendor_boot_dat_timeout: ReadWrite<u32>,
    pub vendor_debounce_count: ReadWrite<u32>,
    pub vendor_misc_cntrl: ReadWrite<u32>,
    pub max_current_override: ReadWrite<u32>,
    pub max_current_override_hi: ReadWrite<u32>,
    _0x12C: [ReadWrite<u32>; 0x20],
    pub vendor_io_trim_cntrl: ReadWrite<u32>,

    // SDMMC2/SDMMC4 only.
    pub vendor_dllcal_cfg: ReadWrite<u32>,
    pub vendor_dll_ctrl0: ReadWrite<u32>,
    pub vendor_dll_ctrl1: ReadWrite<u32>,
    pub vendor_dllcal_cfg_sta: ReadWrite<u32>,

    pub vendor_tuning_cntrl0: ReadWrite<u32>,
    pub vendor_tuning_cntrl1: ReadWrite<u32>,
    pub vendor_tuning_status0: ReadWrite<u32>,
    pub vendor_tuning_status1: ReadWrite<u32>,
    pub vendor_clk_gate_hysteresis_count: ReadWrite<u32>,
    pub vendor_preset_val0: ReadWrite<u32>,
    pub vendor_preset_val1: ReadWrite<u32>,
    pub vendor_preset_val2: ReadWrite<u32>,
    pub sdmemcomppadctrl: ReadWrite<u32>,
    pub auto_cal_config: ReadWrite<u32>,
    pub auto_cal_interval: ReadWrite<u32>,
    pub auto_cal_status: ReadWrite<u32>,
    pub io_spare: ReadWrite<u32>,
    pub sdmmca_mccif_fifoctrl: ReadWrite<u32>,
    pub timeout_wcoal_sdmmca: ReadWrite<u32>,
    _0x1FC: ReadWrite<u32>,
}

impl Registers {
    /// Factory method to create a pointer to a given SDMMC controller register block.
    pub const fn get(index: u32) -> *const Self {
        (SDMMC_BASE + index * 0x200) as *const _
    }
}
