use register::mmio::ReadWrite;

use crate::{
    apb_misc::Padctl,
    clock::{Car, CLK_L_SDMMC1, CLK_L_SDMMC2, CLK_L_SDMMC4, CLK_SOURCE_FIRST, CLK_U_SDMMC3},
    timer::{get_microseconds, get_time_since, usleep},
};

/// Base address for SDMMC registers.
const SDMMC_BASE: u32 = 0x700B_0000;

const CLK_SOURCES: [u32; 4] = [0; 4];

const CLK_DIVIDERS: [u32; 4] = [0; 4];

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
    pub struct HostControl: u8 {
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
    pub struct HostControl2: u16 {
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

/// Representation of the SDMMC controllers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdmmcController {
    Sdmmc1 = 0,
    Sdmmc2,
    Sdmmc3,
    Sdmmc4,
}

enum_from_primitive! {
    /// SDMMC partition types.
    pub enum SdmmcPartition {
        InvalidPartition = -1,
        UserPartition = 0,
        Boot0Partition = 1,
        Boot1Partition = 2,
        RpmbPartition = 3,
    }
}

enum_from_primitive! {
    /// SDMMC voltage presets.
    #[derive(Clone, Copy, PartialEq)]
    pub enum SdmmcBusVoltage {
        VoltageNone = 0,
        Voltage1V8,
        Voltage3V3,
    }
}

enum_from_primitive! {
    /// SDMMC bus width presets.
    #[derive(Clone, Copy, PartialEq)]
    pub enum SdmmcBusWidth {
        Width1Bit = 0,
        Width4Bit,
        Width8Bit,
    }
}

enum_from_primitive! {
    /// SDMMC bus speed presets.
    #[derive(Copy, Clone, PartialEq)]
    pub enum SdmmcBusSpeed {
        MmcInit = 0,
        MmcLegacy,
        MmcHs,
        MmcHs200,
        MmcHs400,
        SdInit,
        SdLegacy,
        SdHs,
        UhsSdr12,
        UhsSdr25,
        UhsSdr50,
        UhsSdr104,
        UhsReserved,
        UhsDdr50,
        MmcDdr52,
    }
}

/// SDMMC dividers for the CAR.
pub enum SdmmcCarDivider {
    UhsSdr12 = 31,  // (16.5 * 2) - 2
    UhsSdr25 = 15,  // (8.5 * 2) - 2
    UhsSdr50 = 7,   // (4.5 * 2) - 2
    UhsSdr104 = 2,  // (2 * 2) - 2
    UhsDdr50 = 18,  // (5 * 2 * 2) - 2
    MmcLegacy = 30, // (16 * 2) - 2
    MmcHs = 14,     // (8 * 2) - 2
    MmcHs200 = 3,   // (2.5 * 2) - 2 (for PLLP_OUT0, same as HS400)
}

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
    pub const fn get(controller: SdmmcController) -> *const Self {
        (SDMMC_BASE + controller as u32 * 0x200) as *const _
    }
}

/// Representation of a SDMMC command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Command {
    pub opcode: u32,
    pub arg: u32,
    pub resp: [u32; 0x4],
    pub flags: u32,
}

/// Representation of a SDMMC request.
pub struct Request<'a> {
    pub data: &'a str,
    pub blksz: u32,
    pub num_blocks: u32,
    pub is_multi_block: bool,
    pub is_read: bool,
    pub is_auto_cmd12: bool,
}

/// Representation of a SDMMC device.
#[derive(Clone, Copy)]
pub struct Sdmmc<'a> {
    pub controller: SdmmcController,
    registers: &'static Registers,
    pub name: &'a str,
    pub has_sd: bool,
    pub is_clk_running: bool,
    pub is_sd_clk_enabled: bool,
    pub is_tuning_tap_val_set: bool,
    pub use_adma: bool,
    pub tap_val: u32,
    pub internal_divider: u32,
    pub resp: [u32; 0x4],
    pub resp_auto_cmd12: u32,
    pub next_dma_addr: u32,
    bus_voltage: SdmmcBusVoltage,
    bus_width: SdmmcBusWidth,
}

/// Gets the appropriate maximum clock frequency for the SDCLK.
fn get_sdclk_frequency(bus_speed: SdmmcBusSpeed) -> u32 {
    match bus_speed {
        SdmmcBusSpeed::MmcInit => 26_000,
        SdmmcBusSpeed::MmcLegacy => 26_000,
        SdmmcBusSpeed::MmcHs => 52_000,
        SdmmcBusSpeed::MmcHs200 => 200_000,
        SdmmcBusSpeed::MmcHs400 => 200_000,
        SdmmcBusSpeed::UhsSdr104 => 200_000,
        SdmmcBusSpeed::SdInit => 25_000,
        SdmmcBusSpeed::SdLegacy => 25_000,
        SdmmcBusSpeed::UhsSdr12 => 25_000,
        SdmmcBusSpeed::SdHs => 50_000,
        SdmmcBusSpeed::UhsSdr25 => 50_000,
        SdmmcBusSpeed::UhsSdr50 => 100_000,
        SdmmcBusSpeed::UhsDdr50 => 40_800,
        SdmmcBusSpeed::MmcDdr52 => 200_000,
        _ => 0,
    }
}

/// Gets the appropriate clock divider for the SDCLK.
fn get_sdclk_divider(bus_speed: SdmmcBusSpeed) -> u32 {
    match bus_speed {
        SdmmcBusSpeed::MmcInit => 66,
        SdmmcBusSpeed::MmcLegacy => 1,
        SdmmcBusSpeed::MmcHs => 1,
        SdmmcBusSpeed::MmcHs200 => 1,
        SdmmcBusSpeed::MmcHs400 => 1,
        SdmmcBusSpeed::UhsSdr104 => 1,
        SdmmcBusSpeed::SdInit => 1,
        SdmmcBusSpeed::SdLegacy => 1,
        SdmmcBusSpeed::UhsSdr12 => 1,
        SdmmcBusSpeed::SdHs => 1,
        SdmmcBusSpeed::UhsSdr25 => 1,
        SdmmcBusSpeed::UhsSdr50 => 1,
        SdmmcBusSpeed::UhsDdr50 => 1,
        SdmmcBusSpeed::MmcDdr52 => 2,
        _ => 0,
    }
}

impl<'a> Sdmmc<'a> {
    /// Checks if the SDMMC device clock is held in reset.
    fn is_clk_reset(&self) -> bool {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => (car.rst_dev_l.get() & CLK_L_SDMMC1) != 0,
            SdmmcController::Sdmmc2 => (car.rst_dev_l.get() & CLK_L_SDMMC2) != 0,
            SdmmcController::Sdmmc3 => (car.rst_dev_u.get() & CLK_U_SDMMC3) != 0,
            SdmmcController::Sdmmc4 => (car.rst_dev_l.get() & CLK_L_SDMMC4) != 0,
        }
    }

    /// Puts the SDMMC device clock into reset.
    fn clk_set_reset(&self) {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => {
                car.rst_dev_l_set.set(CLK_L_SDMMC1);
            },
            SdmmcController::Sdmmc2 => {
                car.rst_dev_l_set.set(CLK_L_SDMMC2);
            },
            SdmmcController::Sdmmc3 => {
                car.rst_dev_u_set.set(CLK_U_SDMMC3);
            },
            SdmmcController::Sdmmc4 => {
                car.rst_dev_l_set.set(CLK_L_SDMMC4);
            },
        }
    }

    /// Takes the SDMMC device clock out of reset.
    fn clk_clear_reset(&self) {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => {
                car.rst_dev_l_clr.set(CLK_L_SDMMC1);
            },
            SdmmcController::Sdmmc2 => {
                car.rst_dev_l_clr.set(CLK_L_SDMMC2);
            },
            SdmmcController::Sdmmc3 => {
                car.rst_dev_u_clr.set(CLK_U_SDMMC3);
            },
            SdmmcController::Sdmmc4 => {
                car.rst_dev_l_clr.set(CLK_L_SDMMC4);
            },
        }
    }

    /// Checks if the SDMMC device clock is enabled.
    fn is_clk_enabled(&self) -> bool {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => (car.clk_out_enb_l.get() & CLK_L_SDMMC1) != 0,
            SdmmcController::Sdmmc2 => (car.clk_out_enb_l.get() & CLK_L_SDMMC2) != 0,
            SdmmcController::Sdmmc3 => (car.clk_out_enb_u.get() & CLK_U_SDMMC3) != 0,
            SdmmcController::Sdmmc4 => (car.clk_out_enb_l.get() & CLK_L_SDMMC4) != 0,
        }
    }

    /// Enables the SDMMC device clock.
    fn clk_set_enabled(&self) {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => {
                car.clk_enb_l_set.set(CLK_L_SDMMC1);
            },
            SdmmcController::Sdmmc2 => {
                car.clk_enb_l_set.set(CLK_L_SDMMC2);
            },
            SdmmcController::Sdmmc3 => {
                car.clk_enb_u_set.set(CLK_U_SDMMC3);
            },
            SdmmcController::Sdmmc4 => {
                car.clk_enb_l_set.set(CLK_L_SDMMC4);
            },
        }
    }

    /// Disables the SDMMC device clock.
    fn clk_set_disabled(&self) {
        let car = &Car::new();

        match self.controller {
            SdmmcController::Sdmmc1 => {
                car.clk_enb_l_clr.set(CLK_L_SDMMC1);
            },
            SdmmcController::Sdmmc2 => {
                car.clk_enb_l_clr.set(CLK_L_SDMMC2);
            },
            SdmmcController::Sdmmc3 => {
                car.clk_enb_u_clr.set(CLK_U_SDMMC3);
            },
            SdmmcController::Sdmmc4 => {
                car.clk_enb_l_clr.set(CLK_L_SDMMC4);
            },
        }
    }

    /// Sets the device clock source and CAR divider.
    fn clk_set_source(&self, frequency: u32) -> Result<u32, ()> {
        let car = &Car::new();

        let mut car_divider;
        let mut out_frequency;

        match frequency {
            25_000 => {
                out_frequency = 24_728;
                car_divider = SdmmcCarDivider::UhsSdr12;
            },
            26_000 => {
                out_frequency = 25_500;
                car_divider = SdmmcCarDivider::MmcLegacy;
            },
            40_800 => {
                out_frequency = 40_800;
                car_divider = SdmmcCarDivider::UhsDdr50;
            },
            50_000 => {
                out_frequency = 48000;
                car_divider = SdmmcCarDivider::UhsSdr25;
            },
            52_000 => {
                out_frequency = 51_000;
                car_divider = SdmmcCarDivider::MmcHs;
            },
            100_000 => {
                out_frequency = 90_667;
                car_divider = SdmmcCarDivider::UhsSdr50;
            },
            200_000 => {
                out_frequency = 163_200;
                car_divider = SdmmcCarDivider::MmcHs200;
            },
            208_000 => {
                out_frequency = 204_000;
                car_divider = SdmmcCarDivider::UhsSdr104;
            },
            _ => {
                return Err(());
            },
        }

        CLK_SOURCES[self.controller as usize] = frequency;
        CLK_DIVIDERS[self.controller as usize] = out_frequency;

        match self.controller {
            SdmmcController::Sdmmc1 => {
                car.clk_source_sdmmc1
                    .set(CLK_SOURCE_FIRST | car_divider as u32);
            },
            SdmmcController::Sdmmc2 => {
                car.clk_source_sdmmc2
                    .set(CLK_SOURCE_FIRST | car_divider as u32);
            },
            SdmmcController::Sdmmc3 => {
                car.clk_source_sdmmc3
                    .set(CLK_SOURCE_FIRST | car_divider as u32);
            },
            SdmmcController::Sdmmc4 => {
                car.clk_source_sdmmc4
                    .set(CLK_SOURCE_FIRST | car_divider as u32);
            },
        }

        Ok(out_frequency)
    }

    /// Adjusts the device clock source value.
    fn clk_adjust_source(&self, source: u32) -> u32 {
        let mut value = 0;

        if CLK_SOURCES[self.controller as usize] == source {
            value = CLK_DIVIDERS[self.controller as usize];
        } else {
            let was_already_enabled = self.is_clk_enabled();

            // Clock was already enabled, disable it.
            if was_already_enabled {
                self.clk_set_disabled();
            }

            value = self.clk_set_source(source).unwrap();

            // Clock was already enabled, enable it back.
            if was_already_enabled {
                self.clk_set_enabled();
            }

            // Dummy read for value refreshing.
            self.is_clk_reset();
        }

        value
    }

    /// Enables the SD clock, if possible.
    fn enable_sd_clock(&mut self) {
        if self.has_sd && self.registers.clock_control.get() & (1 << 2) == 0 {
            self.registers
                .clock_control
                .set(self.registers.clock_control.get() | (1 << 2));
        }

        self.is_sd_clk_enabled = true;
    }

    /// Disables the SD clock.
    fn disable_sd_clock(&mut self) {
        self.registers
            .clock_control
            .set(self.registers.clock_control.get() & !(1 << 2));

        self.is_sd_clk_enabled = false;
    }

    /// Automatically enables or disables the SD clock.
    fn adjust_sd_clock(&mut self) {
        if !self.has_sd && self.registers.clock_control.get() & (1 << 2) != 0 {
            self.disable_sd_clock();
        } else if self.is_sd_clk_enabled && self.registers.clock_control.get() & (1 << 2) == 0 {
            self.enable_sd_clock();
        }
    }

    /// Returns the clock control value. Used for dummy reads.
    fn get_sd_clock_control(&self) -> u16 {
        self.registers.clock_control.get()
    }

    /// Starts the SDMMC clock.
    fn clk_start(&self, source: u32) {
        // Clock was already enabled. Disable it.
        if self.is_clk_enabled() {
            self.clk_set_disabled();
        }

        // Put the device clock into reset.
        self.clk_set_reset();

        // Configure the device clock source.
        let clk_divider = self.clk_set_source(source).unwrap();

        // Enable the device clock.
        self.clk_set_enabled();

        // Dummy read for value refreshing.
        self.is_clk_reset();

        // Synchronize.
        usleep((100_000 + clk_divider - 1) / clk_divider);

        // Take the device clock out of reset.
        self.clk_clear_reset();

        // Dummy read for value refreshing.
        self.is_clk_reset();
    }

    /// Stops the SDMMC clock.
    fn clk_stop(&self) {
        // Put the device clock in reset.
        self.clk_set_reset();

        // Disable the device clock.
        self.clk_set_disabled();

        // Dummy read for value refreshing.
        self.is_clk_reset();
    }

    /// Configures the clock trimming.
    fn vendor_clock_cntrl_config(&self) {
        // Clear the I/O conditioning constants.
        self.registers
            .vendor_clock_cntrl
            .set(self.registers.vendor_clock_cntrl.get() & !0xFFFF_0000);

        // Enable the PADPIPE clock.
        self.registers
            .vendor_clock_cntrl
            .set(self.registers.vendor_clock_cntrl.get() | (1 << 3));

        // Set the appropriate trim value.
        match self.controller {
            SdmmcController::Sdmmc1 => {
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() | (0x02 << 24));
            },
            SdmmcController::Sdmmc2 => {
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() | (0x08 << 24));
            },
            SdmmcController::Sdmmc3 => {
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() | (0x03 << 24));
            },
            SdmmcController::Sdmmc4 => {
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() | (0x08 << 24));
            },
        }
    }

    /// Configures automatic calibration.
    fn autocal_config(&self, voltage: SdmmcBusVoltage) -> Result<(), ()> {
        match self.controller {
            SdmmcController::Sdmmc1 | SdmmcController::Sdmmc3 => match voltage {
                SdmmcBusVoltage::Voltage1V8 => {
                    self.registers
                        .auto_cal_config
                        .set(self.registers.auto_cal_config.get() & !0x7F7F);
                    self.registers
                        .auto_cal_config
                        .set(self.registers.auto_cal_config.get() | 0x7B7B);
                },
                SdmmcBusVoltage::Voltage3V3 => {
                    self.registers
                        .auto_cal_config
                        .set(self.registers.auto_cal_config.get() & !0x7F7F);
                    self.registers
                        .auto_cal_config
                        .set(self.registers.auto_cal_config.get() | 0x7D00);
                },
                _ => {
                    // uSD does not support requested voltage.
                    return Err(());
                },
            },
            SdmmcController::Sdmmc2 | SdmmcController::Sdmmc4 => {
                if voltage != SdmmcBusVoltage::Voltage1V8 {
                    // eMMC can only run at 1V8.
                    return Err(());
                }

                self.registers
                    .auto_cal_config
                    .set(self.registers.auto_cal_config.get() & !0x7F7F);
                self.registers
                    .auto_cal_config
                    .set(self.registers.auto_cal_config.get() | 0x0505);
            },
        }

        Ok(())
    }

    /// Runs automatic calibration.
    fn autocal_run(&mut self, voltage: SdmmcBusVoltage) {
        let padctl = &Padctl::new();
        let mut restart_sd_clock = false;

        // SD clock is enabled, disable it and restart later.
        if self.is_sd_clk_enabled {
            restart_sd_clock = true;
            self.disable_sd_clock();
        }

        // Set PAD_E_INPUT_OR_E_PWRD.
        if self.registers.sdmemcomppadctrl.get() & 0x8000_0000 == 0 {
            self.registers
                .sdmemcomppadctrl
                .set(self.registers.sdmemcomppadctrl.get() | 0x8000_0000);

            // Force a register read to refresh the clock control value.
            self.get_sd_clock_control();

            // Delay.
            usleep(1);
        }

        // Start automatic calibration.
        self.registers.auto_cal_config.set(
            self.registers.auto_cal_config.get()
                & (AutocalConfiguration::SDMMC_AUTOCAL_START
                    | AutocalConfiguration::SDMMC_AUTOCAL_ENABLE)
                    .bits(),
        );

        // Force a register read to refresh the clock control value.
        self.get_sd_clock_control();

        // Delay.
        usleep(1);

        // Get current time.
        let timebase = get_microseconds();

        // Wait until the autocal is complete.
        while self.registers.auto_cal_status.get() & AutocalStatus::SDMMC_AUTOCAL_ACTIVE.bits() != 0
        {
            // Ensure we haven't timed out.
            if get_time_since(timebase) > Timeouts::SDMMC_AUTOCAL_TIMEOUT.bits() {
                // Autocal timed out.

                // Force a register read to refresh the clock control value.
                self.get_sd_clock_control();

                // Upon timeout, fall back to the standard values.
                match self.controller {
                    SdmmcController::Sdmmc1 => {
                        let mut drvup;
                        let mut drvdn;

                        if voltage == SdmmcBusVoltage::Voltage3V3 {
                            drvup = 0x12;
                            drvdn = 0x12;
                        } else {
                            drvup = 0x11;
                            drvdn = 0x15;
                        }

                        let mut value = padctl.sdmmc1_pad_cfgpadctrl.get();

                        value &= !((0x7F << 20) | (0x7F << 12));
                        value |= drvup << 20;
                        value |= drvdn << 12;

                        padctl.sdmmc1_pad_cfgpadctrl.set(value);
                    },
                    SdmmcController::Sdmmc4 => {
                        let mut value = padctl.emmc4_pad_cfgpadctrl.get();

                        value &= !((0x3F << 8) | (0x3F << 2));
                        value |= 0x10 << 8;
                        value |= 0x10 << 2;

                        padctl.emmc4_pad_cfgpadctrl.set(value);
                    },
                    _ => {},
                }

                // Manually clear the autocal enable bit.
                self.registers.auto_cal_config.set(
                    self.registers.auto_cal_config.get()
                        & !AutocalConfiguration::SDMMC_AUTOCAL_ENABLE.bits(),
                );
            }
        }

        // Clear PAD_E_INPUT_OR_E_PWRD (relevant for eMMC only).
        self.registers
            .sdmemcomppadctrl
            .set(self.registers.sdmemcomppadctrl.get() & !0x8000_0000);

        // If requested, enable the SD clock.
        if restart_sd_clock {
            self.enable_sd_clock();
        }
    }

    /// Enables the internal clock.
    fn internal_clk_enable(&mut self) -> Result<(), ()> {
        // Enable the internal clock.
        self.registers
            .clock_control
            .set(self.registers.clock_control.get() | (1 << 0));

        // Force a register read to refresh the clock control value.
        self.get_sd_clock_control();

        // Program a timeout of 2000 milliseconds.
        let timebase = get_microseconds();
        let mut is_timeout = false;

        // Wait for the clock to stabilize.
        while !is_timeout && self.registers.clock_control.get() & (1 << 1) == 0 {
            // Keep checking if timeout expired.
            is_timeout = get_time_since(timebase) > 2_000_000;
        }

        // Clock failed to stabilize.
        if is_timeout {
            // Clock never stabilized.
            return Err(());
        }

        // Configure clock control and host control 2.
        self.registers.host_control2.set(
            self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_PRESET_VAL_ENABLE.bits(),
        );
        self.registers
            .host_control
            .set(self.registers.host_control.get() & !(1 << 5));
        self.registers
            .host_control2
            .set(self.registers.host_control2.get() | HostControl2::SDHCI_HOST_VERSION_4_EN.bits());

        // Ensure 64-bit addressing is supported.
        if self.registers.capabilities.get() & Capabilities::SDHCI_CAN_64BIT.bits() == 0 {
            // 64-bit addressing is unsupported.
            return Err(());
        }

        // Enable 64-bit addressing.
        self.registers.host_control2.set(
            self.registers.host_control2.get() | HostControl2::SDHCI_ADDRESSING_64BIT_EN.bits(),
        );

        // Use SDMA by default.
        self.registers
            .host_control
            .set(self.registers.host_control.get() & !HostControl::SDHCI_CTRL_DMA_MASK.bits());

        // Change to ADMA if possible.
        if self.registers.capabilities.get() & Capabilities::SDHCI_CAN_DO_ADMA2.bits() != 0 {
            self.use_adma = true;
        }

        // Set the timeout to be the maximum value.
        self.registers
            .timeout_control
            .set(self.registers.timeout_control.get() & 0xF0);
        self.registers
            .timeout_control
            .set(self.registers.timeout_control.get() | 0x0E);

        Ok(())
    }

    /// Gets the bus width.
    pub fn get_bus_width(&self) -> SdmmcBusWidth {
        self.bus_width
    }

    /// Sets the bus width.
    pub fn set_bus_width(&mut self, bus_width: SdmmcBusWidth) {
        match bus_width {
            SdmmcBusWidth::Width1Bit => {
                self.registers.host_control.set(
                    self.registers.host_control.get()
                        & !(HostControl::SDHCI_CTRL_4BITBUS | HostControl::SDHCI_CTRL_8BITBUS)
                            .bits(),
                );
            },
            SdmmcBusWidth::Width4Bit => {
                self.registers.host_control.set(
                    self.registers.host_control.get() | HostControl::SDHCI_CTRL_4BITBUS.bits(),
                );
                self.registers.host_control.set(
                    self.registers.host_control.get() & !HostControl::SDHCI_CTRL_8BITBUS.bits(),
                );
            },
            SdmmcBusWidth::Width8Bit => {
                self.registers.host_control.set(
                    self.registers.host_control.get() | HostControl::SDHCI_CTRL_8BITBUS.bits(),
                );
            },
        }

        self.bus_width = bus_width;
    }

    /// Gets the voltage.
    pub fn get_voltage(&self) -> SdmmcBusVoltage {
        self.bus_voltage
    }

    /// Sets the voltage.
    pub fn set_voltage(&mut self, voltage: SdmmcBusVoltage) {
        match voltage {
            SdmmcBusVoltage::VoltageNone => {
                self.registers
                    .power_control
                    .set(self.registers.power_control.get() & !(1 << 0));
            },
            SdmmcBusVoltage::Voltage1V8 => {
                self.registers
                    .power_control
                    .set(self.registers.power_control.get() | (5 << 1));
                self.registers
                    .power_control
                    .set(self.registers.power_control.get() | (1 << 0));
            },
            SdmmcBusVoltage::Voltage3V3 => {
                self.registers
                    .power_control
                    .set(self.registers.power_control.get() | (7 << 1));
                self.registers
                    .power_control
                    .set(self.registers.power_control.get() | (1 << 0));
            },
        }

        self.bus_voltage = voltage;
    }

    fn tap_config(&mut self, bus_speed: SdmmcBusSpeed) {
        if bus_speed == SdmmcBusSpeed::MmcHs400 {
            // Clear and set DQS_TRIM_VAL (used in HS400).
            self.registers
                .vendor_cap_overrides
                .set(self.registers.vendor_cap_overrides.get() & !0x3F00);
            self.registers
                .vendor_cap_overrides
                .set(self.registers.vendor_cap_overrides.get() | 0x2800);
        }

        // Clear TAP_VAL_UPDATED_BY_HW.
        self.registers
            .vendor_tuning_cntrl0
            .set(self.registers.vendor_tuning_cntrl0.get() & !0x20000);

        if bus_speed == SdmmcBusSpeed::MmcHs400 {
            // We must have obtained the tap value from the tuning procedure here.
            if self.is_tuning_tap_val_set {
                // Clear and set the tap value.
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() & !0xFF0000);
                self.registers
                    .vendor_clock_cntrl
                    .set(self.registers.vendor_clock_cntrl.get() | (self.tap_val << 16));
            }
        } else {
            // Use the recommended values.
            match self.controller {
                SdmmcController::Sdmmc1 => {
                    self.tap_val = 4;
                },
                SdmmcController::Sdmmc2 | SdmmcController::Sdmmc4 => {
                    self.tap_val = 0;
                },
                SdmmcController::Sdmmc3 => {
                    self.tap_val = 3;
                },
            }

            // Clear and set the tap values.
            self.registers
                .vendor_clock_cntrl
                .set(self.registers.vendor_clock_cntrl.get() & !0xFF0000);
            self.registers
                .vendor_clock_cntrl
                .set(self.registers.vendor_clock_cntrl.get() | (self.tap_val << 16));
        }
    }

    fn dllcal_run(&mut self) -> Result<(), ()> {
        let mut shutdown_sd_clock = false;

        // SD clock is disabled, enable it.
        if !self.is_sd_clk_enabled {
            shutdown_sd_clock = true;
            self.enable_sd_clock();
        }

        // Set the CALIBRATE bit.
        self.registers
            .vendor_dllcal_cfg
            .set(self.registers.vendor_dllcal_cfg.get() | 0x8000_0000);

        // Force a register read to refresh the clock control value.
        self.get_sd_clock_control();

        // Program a timeout of 5 milliseconds.
        let mut timebase = get_microseconds();
        let mut is_timeout = false;

        // Wait for CALIBRATE to be cleared.
        while !is_timeout && self.registers.vendor_dllcal_cfg.get() & 0x8000_0000 != 0 {
            // Keep checking if timeout expired.
            is_timeout = get_time_since(timebase) > 5_000;
        }

        // Calibration failed.
        if is_timeout {
            // DLLCAL failed.
            return Err(());
        }

        // Program a timeout of 10 milliseconds.
        timebase = get_microseconds();
        is_timeout = false;

        // Wait for DLL_CAL_ACTIVE to be cleared.
        while !is_timeout && self.registers.vendor_dllcal_cfg_sta.get() & 0x8000_0000 != 0 {
            // Keep checking if timeout expired.
            is_timeout = get_time_since(timebase) > 10_000;
        }

        // Calibration failed.
        if is_timeout {
            // DLLCAL failed.
            return Err(());
        }

        // If requested, disable the SD clock.
        if shutdown_sd_clock {
            self.disable_sd_clock();
        }

        Ok(())
    }

    /// Sets the bus speed.
    pub fn set_bus_speed(&mut self, bus_speed: SdmmcBusSpeed) -> Result<(), ()> {
        let mut restart_sd_clock = false;

        // SD clock is enabled, disable it and restart later.
        if self.is_sd_clk_enabled {
            restart_sd_clock = true;
            self.disable_sd_clock();
        }

        // Configure tap values as necessary.
        self.tap_config(bus_speed);

        // Set the appropriate host speed.
        match bus_speed {
            // 400kHz initialization mode and a few others.
            SdmmcBusSpeed::MmcInit
            | SdmmcBusSpeed::MmcLegacy
            | SdmmcBusSpeed::SdInit
            | SdmmcBusSpeed::SdLegacy => {
                self.registers
                    .host_control
                    .set(self.registers.host_control.get() & !HostControl::SDHCI_CTRL_HISPD.bits());
                self.registers.host_control2.set(
                    self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_VDD_180.bits(),
                );
            },

            // 50MHz high speed (SD) and 52MHz high speed (MMC).
            SdmmcBusSpeed::SdHs | SdmmcBusSpeed::MmcHs | SdmmcBusSpeed::UhsSdr25 => {
                self.registers
                    .host_control
                    .set(self.registers.host_control.get() | HostControl::SDHCI_CTRL_HISPD.bits());
                self.registers.host_control2.set(
                    self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_VDD_180.bits(),
                );
            },

            // 200MHz UHS-I (SD) and other modes due to errata.
            SdmmcBusSpeed::MmcHs200
            | SdmmcBusSpeed::UhsSdr104
            | SdmmcBusSpeed::UhsDdr50
            | SdmmcBusSpeed::UhsSdr50
            | SdmmcBusSpeed::MmcDdr52 => {
                self.registers.host_control2.set(
                    self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_UHS_MASK.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_UHS_SDR104.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_VDD_180.bits(),
                );
            },

            // 200MHz single-data rate (MMC).
            SdmmcBusSpeed::MmcHs400 => {
                self.registers.host_control2.set(
                    self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_UHS_MASK.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_HS400.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_VDD_180.bits(),
                );
            },

            // 25Mhz default speed (SD).
            SdmmcBusSpeed::UhsSdr12 => {
                self.registers.host_control2.set(
                    self.registers.host_control2.get() & !HostControl2::SDHCI_CTRL_UHS_MASK.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_UHS_SDR12.bits(),
                );
                self.registers.host_control2.set(
                    self.registers.host_control2.get() | HostControl2::SDHCI_CTRL_VDD_180.bits(),
                );
            },

            _ => {
                // Switching to unsupported speed.
                return Err(());
            },
        }

        // Force a register read to refresh the clock control value.
        self.get_sd_clock_control();

        // Get the clock's frequency and divider.
        let frequency = get_sdclk_frequency(bus_speed);
        let divider = get_sdclk_divider(bus_speed);

        // Adjust the CAR side of the clock.
        let out_frequency = self.clk_adjust_source(frequency);

        // Save the internal divider value.
        self.internal_divider = (out_frequency + divider - 1) / divider;

        let divider_low = divider >> 1;
        let mut divider_high = 0;

        if divider_low > 0xFF {
            divider_high = divider_low >> 8;
        }

        // Set the clock control divider values.
        self.registers
            .clock_control
            .set(self.registers.clock_control.get() & !((0x300 | 0xFF) << 6));
        self.registers.clock_control.set(
            self.registers.clock_control.get() | ((divider_high << 6) | (divider_low << 8)) as u16,
        );

        // If requested, enable the SD clock.
        if restart_sd_clock {
            self.enable_sd_clock();
        }

        // Run DLLCAL for HS400 only.
        if bus_speed == SdmmcBusSpeed::MmcHs400 {
            self.dllcal_run()?;
        }

        Ok(())
    }
}
