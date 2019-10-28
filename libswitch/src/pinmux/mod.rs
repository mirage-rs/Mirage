//! Pin Multiplexer (Pinmux) configurations for various I/O controllers.

use register::mmio::*;

use crate::i2c::I2cDevice;
use crate::uart::UartDevice;

const PINMUX_BASE: u32 = 0x7000_3000;

pub const PINMUX_PULL_NONE: u32 = (0 << 2);
pub const PINMUX_PULL_DOWN: u32 = (1 << 2);
pub const PINMUX_PULL_UP: u32 = (2 << 2);

pub const PINMUX_TRISTATE: u32 = (1 << 4);
pub const PINMUX_PARKED: u32 = (1 << 5);
pub const PINMUX_INPUT: u32 = (1 << 6);
pub const PINMUX_LOCK: u32 = (1 << 7);
pub const PINMUX_LPDR: u32 = (1 << 8);
pub const PINMUX_HSM: u32 = (1 << 9);

#[repr(C)]
pub struct PinmuxRegisters {
    pub sdmmc1_clk: ReadWrite<u32>,
    pub sdmmc1_cmd: ReadWrite<u32>,
    pub sdmmc1_dat3: ReadWrite<u32>,
    pub sdmmc1_dat2: ReadWrite<u32>,
    pub sdmmc1_dat1: ReadWrite<u32>,
    pub sdmmc1_dat0: ReadWrite<u32>,
    _r18: ReadWrite<u32>,
    pub sdmmc3_clk: ReadWrite<u32>,
    pub sdmmc3_cmd: ReadWrite<u32>,
    pub sdmmc3_dat0: ReadWrite<u32>,
    pub sdmmc3_dat1: ReadWrite<u32>,
    pub sdmmc3_dat2: ReadWrite<u32>,
    pub sdmmc3_dat3: ReadWrite<u32>,
    _r34: ReadWrite<u32>,
    pub pex_l0_rst_n: ReadWrite<u32>,
    pub pex_l0_clkreq_n: ReadWrite<u32>,
    pub pex_wake_n: ReadWrite<u32>,
    pub pex_l1_rst_n: ReadWrite<u32>,
    pub pex_l1_clkreq_n: ReadWrite<u32>,
    pub sata_led_active: ReadWrite<u32>,
    pub spi1_mosi: ReadWrite<u32>,
    pub spi1_miso: ReadWrite<u32>,
    pub spi1_sck: ReadWrite<u32>,
    pub spi1_cs0: ReadWrite<u32>,
    pub spi1_cs1: ReadWrite<u32>,
    pub spi2_mosi: ReadWrite<u32>,
    pub spi2_miso: ReadWrite<u32>,
    pub spi2_sck: ReadWrite<u32>,
    pub spi2_cs0: ReadWrite<u32>,
    pub spi2_cs1: ReadWrite<u32>,
    pub spi4_mosi: ReadWrite<u32>,
    pub spi4_miso: ReadWrite<u32>,
    pub spi4_sck: ReadWrite<u32>,
    pub spi4_cs0: ReadWrite<u32>,
    pub qspi_sck: ReadWrite<u32>,
    pub qspi_cs_n: ReadWrite<u32>,
    pub qspi_io0: ReadWrite<u32>,
    pub qspi_io1: ReadWrite<u32>,
    pub qspi_io2: ReadWrite<u32>,
    pub qspi_io3: ReadWrite<u32>,
    _ra0: ReadWrite<u32>,
    pub dmic1_clk: ReadWrite<u32>,
    pub dmic1_dat: ReadWrite<u32>,
    pub dmic2_clk: ReadWrite<u32>,
    pub dmic2_dat: ReadWrite<u32>,
    pub dmic3_clk: ReadWrite<u32>,
    pub dmic3_dat: ReadWrite<u32>,
    pub gen1_i2c_scl: ReadWrite<u32>,
    pub gen1_i2c_sda: ReadWrite<u32>,
    pub gen2_i2c_scl: ReadWrite<u32>,
    pub gen2_i2c_sda: ReadWrite<u32>,
    pub gen3_i2c_scl: ReadWrite<u32>,
    pub gen3_i2c_sda: ReadWrite<u32>,
    pub cam_i2c_scl: ReadWrite<u32>,
    pub cam_i2c_sda: ReadWrite<u32>,
    pub pwr_i2c_scl: ReadWrite<u32>,
    pub pwr_i2c_sda: ReadWrite<u32>,
    pub uart1_tx: ReadWrite<u32>,
    pub uart1_rx: ReadWrite<u32>,
    pub uart1_rts: ReadWrite<u32>,
    pub uart1_cts: ReadWrite<u32>,
    pub uart2_tx: ReadWrite<u32>,
    pub uart2_rx: ReadWrite<u32>,
    pub uart2_rts: ReadWrite<u32>,
    pub uart2_cts: ReadWrite<u32>,
    pub uart3_tx: ReadWrite<u32>,
    pub uart3_rx: ReadWrite<u32>,
    pub uart3_rts: ReadWrite<u32>,
    pub uart3_cts: ReadWrite<u32>,
    pub uart4_tx: ReadWrite<u32>,
    pub uart4_rx: ReadWrite<u32>,
    pub uart4_rts: ReadWrite<u32>,
    pub uart4_cts: ReadWrite<u32>,
    pub dap1_fs: ReadWrite<u32>,
    pub dap1_din: ReadWrite<u32>,
    pub dap1_dout: ReadWrite<u32>,
    pub dap1_sclk: ReadWrite<u32>,
    pub dap2_fs: ReadWrite<u32>,
    pub dap2_din: ReadWrite<u32>,
    pub dap2_dout: ReadWrite<u32>,
    pub dap2_sclk: ReadWrite<u32>,
    pub dap4_fs: ReadWrite<u32>,
    pub dap4_din: ReadWrite<u32>,
    pub dap4_dout: ReadWrite<u32>,
    pub dap4_sclk: ReadWrite<u32>,
    pub cam1_mclk: ReadWrite<u32>,
    pub cam2_mclk: ReadWrite<u32>,
    pub jtag_rtck: ReadWrite<u32>,
    pub clk_32k_in: ReadWrite<u32>,
    pub clk_32k_out: ReadWrite<u32>,
    pub batt_bcl: ReadWrite<u32>,
    pub clk_req: ReadWrite<u32>,
    pub cpu_pwr_req: ReadWrite<u32>,
    pub pwr_int_n: ReadWrite<u32>,
    pub shutdown: ReadWrite<u32>,
    pub core_pwr_req: ReadWrite<u32>,
    pub aud_mclk: ReadWrite<u32>,
    pub dvfs_pwm: ReadWrite<u32>,
    pub dvfs_clk: ReadWrite<u32>,
    pub gpio_x1_aud: ReadWrite<u32>,
    pub gpio_x3_aud: ReadWrite<u32>,
    pub pcc7: ReadWrite<u32>,
    pub hdmi_cec: ReadWrite<u32>,
    pub hdmi_int_dp_hpd: ReadWrite<u32>,
    pub spdif_out: ReadWrite<u32>,
    pub spdif_in: ReadWrite<u32>,
    pub usb_vbus_en0: ReadWrite<u32>,
    pub usb_vbus_en1: ReadWrite<u32>,
    pub dp_hpd0: ReadWrite<u32>,
    pub wifi_en: ReadWrite<u32>,
    pub wifi_rst: ReadWrite<u32>,
    pub wifi_wake_ap: ReadWrite<u32>,
    pub ap_wake_bt: ReadWrite<u32>,
    pub bt_rst: ReadWrite<u32>,
    pub bt_wake_ap: ReadWrite<u32>,
    pub ap_wake_nfc: ReadWrite<u32>,
    pub nfc_en: ReadWrite<u32>,
    pub nfc_int: ReadWrite<u32>,
    pub gps_en: ReadWrite<u32>,
    pub gps_rst: ReadWrite<u32>,
    pub cam_rst: ReadWrite<u32>,
    pub cam_af_en: ReadWrite<u32>,
    pub cam_flash_en: ReadWrite<u32>,
    pub cam1_pwdn: ReadWrite<u32>,
    pub cam2_pwdn: ReadWrite<u32>,
    pub cam1_strobe: ReadWrite<u32>,
    pub lcd_te: ReadWrite<u32>,
    pub lcd_bl_pwm: ReadWrite<u32>,
    pub lcd_bl_en: ReadWrite<u32>,
    pub lcd_rst: ReadWrite<u32>,
    pub lcd_gpio1: ReadWrite<u32>,
    pub lcd_gpio2: ReadWrite<u32>,
    pub ap_ready: ReadWrite<u32>,
    pub touch_rst: ReadWrite<u32>,
    pub touch_clk: ReadWrite<u32>,
    pub modem_wake_ap: ReadWrite<u32>,
    pub touch_int: ReadWrite<u32>,
    pub motion_int: ReadWrite<u32>,
    pub als_prox_int: ReadWrite<u32>,
    pub temp_alert: ReadWrite<u32>,
    pub button_power_on: ReadWrite<u32>,
    pub button_vol_up: ReadWrite<u32>,
    pub button_vol_down: ReadWrite<u32>,
    pub button_slide_sw: ReadWrite<u32>,
    pub button_home: ReadWrite<u32>,
    pub pa6: ReadWrite<u32>,
    pub pe6: ReadWrite<u32>,
    pub pe7: ReadWrite<u32>,
    pub ph6: ReadWrite<u32>,
    pub pk0: ReadWrite<u32>,
    pub pk1: ReadWrite<u32>,
    pub pk2: ReadWrite<u32>,
    pub pk3: ReadWrite<u32>,
    pub pk4: ReadWrite<u32>,
    pub pk5: ReadWrite<u32>,
    pub pk6: ReadWrite<u32>,
    pub pk7: ReadWrite<u32>,
    pub pl0: ReadWrite<u32>,
    pub pl1: ReadWrite<u32>,
    pub pz0: ReadWrite<u32>,
    pub pz1: ReadWrite<u32>,
    pub pz2: ReadWrite<u32>,
    pub pz3: ReadWrite<u32>,
    pub pz4: ReadWrite<u32>,
    pub pz5: ReadWrite<u32>,
}

impl PinmuxRegisters {
    pub fn get() -> *const Self {
        PINMUX_BASE as *const PinmuxRegisters
    }
}

/// Configures an UART device.
pub fn configure_uart(device: UartDevice) {
    let value = match device {
        UartDevice::A => 0,
        UartDevice::B => 1,
        UartDevice::C => 2,
        UartDevice::D => 3,
        UartDevice::E => 4,
    };

    let tx_reg = unsafe { &(*((PINMUX_BASE + 0xE4 + 0x10 * value) as *const WriteOnly<u32>)) };
    let rx_reg = unsafe { &(*((PINMUX_BASE + 0xE8 + 0x10 * value) as *const WriteOnly<u32>)) };
    let rts_reg = unsafe { &(*((PINMUX_BASE + 0xEC + 0x10 * value) as *const WriteOnly<u32>)) };
    let cts_reg = unsafe { &(*((PINMUX_BASE + 0xF0 + 0x10 * value) as *const WriteOnly<u32>)) };

    tx_reg.set(0);
    rx_reg.set(PINMUX_INPUT | PINMUX_PULL_UP);
    rts_reg.set(0);
    cts_reg.set(PINMUX_INPUT | PINMUX_PULL_DOWN);
}

/// Configures an I²C device.
pub fn configure_i2c(device: I2cDevice) {
    let value = match device {
        I2cDevice::I1 => 0,
        I2cDevice::I2 => 1,
        I2cDevice::I3 => 2,
        I2cDevice::I4 => 3,
        I2cDevice::I5 => 4,
        I2cDevice::I6 => 5,
    };

    let scl_reg = unsafe { &(*((PINMUX_BASE + 0xBC + 8 * value) as *const WriteOnly<u32>)) };
    let sda_reg = unsafe { &(*((PINMUX_BASE + 0xC0 + 8 * value) as *const WriteOnly<u32>)) };

    scl_reg.set(PINMUX_INPUT);
    sda_reg.set(PINMUX_INPUT);
}
