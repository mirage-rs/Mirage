//! Pin Multiplexer (Pinmux) configurations for various I/O controllers.
//!
//! # Description
//!
//! Tegra X1 devices can be configured with different I/O functions to
//! allow use in a variety of different configurations.
//!
//! Many of the pins on Tegra X1 devices are connected to multi-purpose
//! I/O (MPIO) pads. An MPIO can operate in two modes:
//! either acting as a signal for a particular I/O controller, referred
//! to as a Special-Function I/O (SFIO); or as a software-controlled
//! general-purpose I/O function, referred to as GPIO. Each MPIO has
//! up to four SFIO functions as well as being a GPIO.
//!
//! Though each MPIO has up to 5 functions (a GPIO function and up to
//! 4 SFIO functions), a given MPIO can only act as a single function
//! at a given point in time. The Pinmux controller in Tegra X1 devices
//! includes the logic and registers to select a particular function for
//! each MPIO.
//!
//! The VGPIO controller supports 8-bit general-purpose Input/Output
//! (VGPIO) ports (port A through port D). These ports allow VGPIO signals
//! to be mapped onto unused individual functional pins which might not
//! be present in the same device. This provides a means to virtualize
//! various pins that cannot be mapped in the same package and have to be
//! moved to a companion device.
//!
//! # Implementation
//!
//! - Publicly exposed Pinmux configurations are implemented as constants.
//! These should however be irrelevant for most cases as the required
//! configurations are done internally.
//!
//! - [`Registers`] is an abstraction of the Pinmux register block that is
//! mapped to `0x70003000`. [`Registers::get`] is used to create a pointer
//! to these.
//!
//! - The public [`Pinmux`] struct is a further abstraction layer which as
//! the [`Deref`] trait implemented. Thus dereferencing a [`Pinmux`] object
//! will dereference a pointer to the [`Registers`] block to expose it for
//! read and write access. A new [`Pinmux`] object should be created using
//! the factory method [`Pinmux::new`]. **Raw writes to the Pinmux registers
//! are to be avoided since it may damage your device!**
//!
//! - The functions [`configure_uart`] and [`configure_i2c`] can be used
//! to configure UART and I²C devices for use.
//!
//! # Example
//!
//! ```
//! use mirage_libswitch::{pinmux::configure_uart, uart::Uart};
//!
//! fn main() {
//!     let device = Uart::A;
//!     configure_uart(&device);
//! }
//! ```
//!
//! [`Registers`]: struct.Registers.html
//! [`Registers::get`]: struct.Registers.html#method_get
//! [`Pinmux`]: struct.Pinmux.html
//! [`Pinmux::get`]: struct.Pinmux.html#method_get
//! [`Deref`]: https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html
//! [`Pinmux::new`]: struct.Pinmux.html#method_new
//! [`configure_uart`]: fn.configure_uart.html
//! [`configure_i2c`]: fn.configure_i2c.html

use core::ops::Deref;

use register::mmio::*;

use crate::{i2c::I2c, uart::Uart};

/// The base address for Pinmux registers.
const PINMUX_BASE: u32 = 0x7000_3000;

/// Configuration value for no pulls.
pub const PULL_NONE: u32 = (0 << 2);
/// Pull-down configuration value.
pub const PULL_DOWN: u32 = (1 << 2);
/// Pull-up configuration value.
pub const PULL_UP: u32 = (2 << 2);

/// Disables the pad’s output driver. This setting overrides all other
/// functional settings and also whether pad is selected for SFIO or
/// GPIO. Can be used when the pad direction changes or the pad is
/// assigned to a different SFIO to avoid glitches. During Cold Boot,
/// most of the pads come with this bit set to TRISTATE so that they do
/// not actively drive anything. For Normal Operation, the bit has to be
/// set to PASSTHROUGH state.Used by the Pinmux logic to drive the
/// appropriate pad control signals.
pub const TRISTATE: u32 = (1 << 4);
/// PARKING state holds control during DPD/LP0. During LP0
/// (deep sleep) entry, all pads (except a few pads in the AO region)
/// are put in the DPD state This bit is set in the Pinmux register
/// by default during Reset. In LP0 exit until this bit is cleared
/// (typically by the LP0 exit Pinmux recovery code), the pads are in
/// the DPD state, i.e., PARKED in the same value as that of LP0 entry.
pub const PARKED: u32 = (1 << 5);
/// Enables or disables input receiver. Applicable to all pads.
pub const INPUT: u32 = (1 << 6);
/// Lock control for writing to the register. Used for security purposes
/// to permanently lock the value to a pinmux register.
///
/// 0: Writes to this register are accepted.
/// 1: Writes to this register are ignored (until the next wake from Deep Sleep).
///
/// This is a sticky bit. Once software sets this bit to 1, the only
/// way to clear it is to reset the chip or enter and exit Deep Sleep.
pub const LOCK: u32 = (1 << 7);
/// Enable only one Base Drivers when set High. Typically set when
/// interfacing chips require minimal rise/fall time such as I2C.
/// Applicable to ST and DD pads.
pub const LPDR: u32 = (1 << 8);
/// Enables High Speed operation for Receiver and Transmitter.
/// Applicable to CZ pads.
pub const HSM: u32 = (1 << 9);

/// Enables open-drain pull-up capability to 3.3V, thereby enabling
/// High Voltage Operation. Enables 3.3V Receive. If E_IO_HV=1,
/// the pad can support 3.3V open-drain driving with I/O pull-up
/// tolerance up to 3.3V and the Receiver is adjusted to 3.3V DC
/// characteristics.
/// Default enabled for all the pads so that they can safely operate
/// without knowing whether externally it is pulled up to 3.3V or 1.8V
/// until the pins get used actively. Until that point, it can be driven
/// to High-Z or Logic 0. The platform software can read the status of
/// pull-up values and configure the E_IO_HV before actually using the pin.
/// For the PMIC I2C interface (i.e., PWR_I2C_SCL and PWR_I2C_SDA), it
/// is set at Logic 0 because this interface is needed during boot and
/// the PMIC interface typically has the pull-up at 1.8V.Applicable to DD pads.
pub const IO_HV: u32 = (1 << 10);
/// Enabling Schmitt provides better noise margin characteristics for the input.
/// Depending on driver’s logic threshold levels, this can be enabled.
/// Applicable to all pads.
pub const SCHMT: u32 = (1 << 12);

/// Representation of the Pinmux registers.
#[repr(C)]
pub struct Registers {
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

impl Registers {
    /// Factory method to create a pointer to the Pinmux registers.
    pub fn get() -> *const Self {
        PINMUX_BASE as *const _
    }
}

/// Pinmux abstraction.
pub struct Pinmux;

impl Deref for Pinmux {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Registers::get() }
    }
}

impl Pinmux {
    /// Creates a new Pinmux object.
    pub fn new() -> Self {
        Pinmux
    }
}

/// Configures an UART device.
pub fn configure_uart(pinmux: &Pinmux, uart: &Uart) {
    match uart {
        &Uart::A => {
            pinmux.uart1_tx.set(0);
            pinmux.uart1_rx.set(INPUT | PULL_UP);
            pinmux.uart1_rts.set(0);
            pinmux.uart1_cts.set(INPUT | PULL_DOWN);
        }
        &Uart::B => {
            pinmux.uart2_tx.set(0);
            pinmux.uart2_rx.set(INPUT | PULL_UP);
            pinmux.uart2_rts.set(0);
            pinmux.uart2_cts.set(INPUT | PULL_DOWN);
        }
        &Uart::C => {
            pinmux.uart3_tx.set(0);
            pinmux.uart3_rx.set(INPUT | PULL_UP);
            pinmux.uart3_rts.set(0);
            pinmux.uart3_cts.set(INPUT | PULL_DOWN);
        }
        &Uart::D => {
            pinmux.uart4_tx.set(0);
            pinmux.uart4_rx.set(INPUT | PULL_UP);
            pinmux.uart4_rts.set(0);
            pinmux.uart4_cts.set(INPUT | PULL_DOWN);
        }
        &Uart::E => {
            pinmux.dap1_fs.set(0);
            pinmux.dap1_din.set(INPUT | PULL_UP);
            pinmux.dap1_dout.set(0);
            pinmux.dap1_sclk.set(INPUT | PULL_DOWN);
        }
    }
}

/// Configures an I²C device.
pub fn configure_i2c(pinmux: &Pinmux, device: &I2c) {
    match device {
        &I2c::C1 => {
            pinmux.gen1_i2c_scl.set(INPUT);
            pinmux.gen1_i2c_sda.set(INPUT);
        }
        &I2c::C2 => {
            pinmux.gen2_i2c_scl.set(INPUT);
            pinmux.gen2_i2c_sda.set(INPUT);
        }
        &I2c::C3 => {
            pinmux.gen3_i2c_scl.set(INPUT);
            pinmux.gen3_i2c_sda.set(INPUT);
        }
        &I2c::C4 => {
            pinmux.cam_i2c_scl.set(INPUT);
            pinmux.cam_i2c_sda.set(INPUT);
        }
        &I2c::C5 => {
            pinmux.pwr_i2c_scl.set(INPUT);
            pinmux.pwr_i2c_sda.set(INPUT);
        }
        &I2c::C6 => {
            // Unused on the Switch.
            // TODO(Vale): Nonetheless, figure this out.
        }
    }
}
