use register::mmio::ReadWrite;

/// Base address for SDMMC registers.
const SDMMC_BASE: u32 = 0x700B_0000;

bitflags! {
    /// Present state flags.
    pub struct PresentState: u32 {
        const SDHCI_CMD_INHIBIT = 0x0000_0001;
        const SDHCI_DATA_INHIBIT = 0x0000_0002;
        const SDHCI_DOING_WRITE = 0x0000_0100;
        const SDHCI_DOING_READ = 0x0000_0200;
        const SDHCI_SPACE_AVAILABLE = 0x0000_0400;
        const SDHCI_DATA_AVAILABLE = 0x0000_0800;
        const SDHCI_CARD_PRESENT = 0x0001_0000;
        const SDHCI_WRITE_PROTECT = 0x0008_0000;
        const SDHCI_DATA_LVL_MASK = 0x00F0_0000;
        const SDHCI_DATA_LVL_SHIFT = 20;
        const SDHCI_DATA_0_LVL_MASK = 0x0010_0000;
        const SDHCI_CMD_LVL = 0x0100_0000;
    }
}

bitflags! {
    /// SDHCI clock control flags.
    pub struct ClockControl: u32 {
        const SDHCI_DIVIDER_SHIFT = 8;
        const SDHCI_DIVIDER_HI_SHIFT = 6;
        const SDHCI_DIV_MASK = 0xFF;
        const SDHCI_DIV_MASK_LEN = 8;
        const SDHCI_DIV_HI_MASK = 0x300;
        const SDHCI_PROG_CLOCK_MODE = 0x0020;
        const SDHCI_CLOCK_CARD_EN = 0x0004;
        const SDHCI_CLOCK_INT_STABLE = 0x0002;
        const SDHCI_CLOCK_INT_EN = 0x0001;
    }
}

bitflags! {
    /// SDHCI host control flags.
    pub struct HostControl: u32 {
        const SDHCI_CTRL_LED = 0x01;
        const SDHCI_CTRL_4BITBUS = 0x02;
        const SDHCI_CTRL_HISPD = 0x04;
        const SDHCI_CTRL_DMA_MASK = 0x18;
        const SDHCI_CTRL_SDMA = 0x00;
        const SDHCI_CTRL_ADMA1 = 0x08;
        const SDHCI_CTRL_ADMA32 = 0x10;
        const SDHCI_CTRL_ADMA64 = 0x18;
        const SDHCI_CTRL_8BITBUS = 0x20;
        const SDHCI_CTRL_CDTEST_INS = 0x40;
        const SDHCI_CTRL_CDTEST_EN = 0x80;
    }
}

bitflags! {
    /// SDHCI host control 2 flags.
    pub struct HostControl2: u32 {
        const SDHCI_CTRL_UHS_MASK = 0x0007;
        const SDHCI_CTRL_UHS_SDR12 = 0x0000;
        const SDHCI_CTRL_UHS_SDR25 = 0x0001;
        const SDHCI_CTRL_UHS_SDR50 = 0x0002;
        const SDHCI_CTRL_UHS_SDR104 = 0x0003;
        const SDHCI_CTRL_UHS_DDR50 = 0x0004;
        const SDHCI_CTRL_HS400 = 0x0005;
        const SDHCI_CTRL_VDD_180 = 0x0008;
        const SDHCI_CTRL_DRV_TYPE_MASK = 0x0030;
        const SDHCI_CTRL_DRV_TYPE_B = 0x0000;
        const SDHCI_CTRL_DRV_TYPE_A = 0x0010;
        const SDHCI_CTRL_DRV_TYPE_C = 0x0020;
        const SDHCI_CTRL_DRV_TYPE_D = 0x0030;
        const SDHCI_CTRL_EXEC_TUNING = 0x0040;
        const SDHCI_CTRL_TUNED_CLK = 0x0080;
        const SDHCI_UHS2_IF_EN = 0x0100;
        const SDHCI_HOST_VERSION_4_EN = 0x1000;
        const SDHCI_ADDRESSING_64BIT_EN = 0x2000;
        const SDHCI_ASYNC_INTR_EN = 0x4000;
        const SDHCI_CTRL_PRESET_VAL_ENABLE = 0x8000;
    }
}

bitflags! {
    /// SDHCI capabilities flags.
    pub struct Capabilities: u32 {
        const SDHCI_CAN_DO_8BIT = 0x0004_0000;
        const SDHCI_CAN_DO_ADMA2 = 0x0008_0000;
        const SDHCI_CAN_DO_ADMA1 = 0x0010_0000;
        const SDHCI_CAN_DO_HISPD = 0x0020_0000;
        const SDHCI_CAN_DO_SDMA = 0x0040_0000;
        const SDHCI_CAN_VDD_330 = 0x0100_0000;
        const SDHCI_CAN_VDD_300 = 0x0200_0000;
        const SDHCI_CAN_VDD_180 = 0x0400_0000;
        const SDHCI_CAN_64BIT = 0x1000_0000;
        const SDHCI_ASYNC_INTR = 0x2000_0000;
    }
}

bitflags! {
    /// Vendor clock control flags.
    pub struct VendorClockControl: u32 {
        const SDMMC_CLOCK_TAP_MASK = (0xFF << 16);
        const SDMMC_CLOCK_TAP_SDMMC1 = (0x04 << 16);
        const SDMMC_CLOCK_TAP_SDMMC2 = (0x00 << 16);
        const SDMMC_CLOCK_TAP_SDMMC3 = (0x03 << 16);
        const SDMMC_CLOCK_TAP_SDMMC4 = (0x00 << 16);
        const SDMMC_CLOCK_TRIM_MASK = (0xFF << 24);
        const SDMMC_CLOCK_TRIM_SDMMC1 = (0x02 << 24);
        const SDMMC_CLOCK_TRIM_SDMMC2 = (0x08 << 24);
        const SDMMC_CLOCK_TRIM_SDMMC3 = (0x03 << 24);
        const SDMMC_CLOCK_TRIM_SDMMC4 = (0x08 << 24);
        const SDMMC_CLOCK_PADPIPE_CLKEN_OVERRIDE = (1 << 3);
    }
}

bitflags! {
    /// Autocal configuration flags.
    pub struct AutocalConfiguration: u32 {
        const SDMMC_AUTOCAL_PDPU_CONFIG_MASK = 0x7F7F;
        const SDMMC_AUTOCAL_PDPU_SDMMC1_1V8 = 0x7B7B;
        const SDMMC_AUTOCAL_PDPU_SDMMC1_3V3 = 0x7D00;
        const SDMMC_AUTOCAL_PDPU_SDMMC4_1V8 = 0x0505;
        const SDMMC_AUTOCAL_START = (1 << 31);
        const SDMMC_AUTOCAL_ENABLE = (1 << 29);
    }
}

bitflags! {
    /// Autocal status flags.
    pub struct AutocalStatus: u32 {
        const SDMMC_AUTOCAL_ACTIVE = (1 << 31);
    }
}

bitflags! {
    /// Vendor tuning control 0 flags.
    pub struct VendorTuningControl0: u32 {
        const SDMMC_VENDOR_TUNING_TRIES_MASK = (0x7 << 13);
        const SDMMC_VENDOR_TUNING_TRIES_SHIFT = 13;
        const SDMMC_VENDOR_TUNING_MULTIPLIER_MASK = (0x7F << 6);
        const SDMMC_VENDOR_TUNING_MULTIPLIER_UNITY = (1 << 6);
        const SDMMC_VENDOR_TUNING_DIVIDER_MASK = (0x7 << 3);
        const SDMMC_VENDOR_TUNING_SET_BY_HW = (1 << 17);
    }
}

bitflags! {
    /// Vendor tuning control 1 flags.
    pub struct VendorTuningControl1: u32 {
        const SDMMC_VENDOR_TUNING_STEP_SIZE_SDR50_DEFAULT = (0 << 0);
        const SDMMC_VENDOR_TUNING_STEP_SIZE_SDR104_DEFAULT = (0 << 4);
    }
}

bitflags! {
    /// Vendor capability overrides flags.
    pub struct VendorCapabilityOverrides: u32 {
        const SDMMC_VENDOR_CAPABILITY_DQS_TRIM_MASK = (0x3F << 8);
        const SDMMC_VENDOR_CAPABILITY_DQS_TRIM_HS400 = (0x11 << 8);
    }
}

bitflags! {
    /// Timeout flags.
    pub struct Timeouts: u32 {
        const SDMMC_AUTOCAL_TIMEOUT = (10 * 1000);
        const SDMMC_TUNING_TIMEOUT = (150 * 1000);
    }
}

bitflags! {
    /// Command response flags.
    pub struct CommandResponse: u32 {
        const SDMMC_RSP_PRESENT = (1 << 0);
        const SDMMC_RSP_136 = (1 << 1);
        const SDMMC_RSP_CRC = (1 << 2);
        const SDMMC_RSP_BUSY = (1 << 3);
        const SDMMC_RSP_OPCODE = (1 << 4);
    }
}

bitflags! {
    /// Command types.
    pub struct CommandTypes: u32 {
        const SDMMC_CMD_MASK = (3 << 5);
        const SDMMC_CMD_AC = (0 << 5);
        const SDMMC_CMD_ADTC = (1 << 5);
        const SDMMC_CMD_BC = (2 << 5);
        const SDMMC_CMD_BCR = (3 << 5);
    }
}

bitflags! {
    /// SPI command response flags.
    pub struct SpiCommandResponse: u32 {
        const SDMMC_RSP_SPI_S1 = (1 << 7);
        const SDMMC_RSP_SPI_S2 = (1 << 8);
        const SDMMC_RSP_SPI_B4 = (1 << 9);
        const SDMMC_RSP_SPI_BUSY = (1 << 10);
    }
}

// Native response types for commands.
pub const SDMMC_RSP_NONE: CommandResponse = CommandResponse::empty();
pub const SDMMC_RSP_R1: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_CRC
    | CommandResponse::SDMMC_RSP_OPCODE;
pub const SDMMC_RSP_R1B: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_CRC
    | CommandResponse::SDMMC_RSP_OPCODE
    | CommandResponse::SDMMC_RSP_BUSY;
pub const SDMMC_RSP_R2: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_136
    | CommandResponse::SDMMC_RSP_CRC;
pub const SDMMC_RSP_R3: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT;
pub const SDMMC_RSP_R4: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT;
pub const SDMMC_RSP_R5: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_CRC
    | CommandResponse::SDMMC_RSP_OPCODE;
pub const SDMMC_RSP_R6: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_CRC
    | CommandResponse::SDMMC_RSP_OPCODE;
pub const SDMMC_RSP_R7: CommandResponse = CommandResponse::SDMMC_RSP_PRESENT
    | CommandResponse::SDMMC_RSP_CRC
    | CommandResponse::SDMMC_RSP_OPCODE;
pub const SDMMC_RSP_R1_NO_CRC: CommandResponse =
    CommandResponse::SDMMC_RSP_PRESENT | CommandResponse::SDMMC_RSP_OPCODE;

// SPI response types for commands.
pub const SDMMC_RSP_SPI_R1: SpiCommandResponse = SpiCommandResponse::SDMMC_RSP_SPI_S1;
pub const SDMMC_RSP_SPI_R1B: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_BUSY;
pub const SDMMC_RSP_SPI_R2: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_S2;
pub const SDMMC_RSP_SPI_R3: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_B4;
pub const SDMMC_RSP_SPI_R4: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_B4;
pub const SDMMC_RSP_SPI_R5: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_S2;
pub const SDMMC_RSP_SPI_R7: SpiCommandResponse =
    SpiCommandResponse::SDMMC_RSP_SPI_S1 | SpiCommandResponse::SDMMC_RSP_SPI_B4;

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
