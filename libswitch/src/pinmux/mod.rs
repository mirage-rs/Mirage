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
//! [`Registers::get`]: struct.Registers.html#method.get
//! [`Pinmux`]: struct.Pinmux.html
//! [`Pinmux::get`]: struct.Pinmux.html#method.get
//! [`Deref`]: https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html
//! [`Pinmux::new`]: struct.Pinmux.html#method.new
//! [`configure_uart`]: fn.configure_uart.html
//! [`configure_i2c`]: fn.configure_i2c.html

use mirage_mmio::{BlockMmio, VolatileStorage};

use crate::{i2c::I2c, uart::Uart};

/// The base address for Pinmux registers.
pub(crate) const PINMUX_BASE: u32 = 0x7000_3000;

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
pub struct Pinmux {
    pub sdmmc1_clk: BlockMmio<u32>,
    pub sdmmc1_cmd: BlockMmio<u32>,
    pub sdmmc1_dat3: BlockMmio<u32>,
    pub sdmmc1_dat2: BlockMmio<u32>,
    pub sdmmc1_dat1: BlockMmio<u32>,
    pub sdmmc1_dat0: BlockMmio<u32>,
    _r18: BlockMmio<u32>,
    pub sdmmc3_clk: BlockMmio<u32>,
    pub sdmmc3_cmd: BlockMmio<u32>,
    pub sdmmc3_dat0: BlockMmio<u32>,
    pub sdmmc3_dat1: BlockMmio<u32>,
    pub sdmmc3_dat2: BlockMmio<u32>,
    pub sdmmc3_dat3: BlockMmio<u32>,
    _r34: BlockMmio<u32>,
    pub pex_l0_rst_n: BlockMmio<u32>,
    pub pex_l0_clkreq_n: BlockMmio<u32>,
    pub pex_wake_n: BlockMmio<u32>,
    pub pex_l1_rst_n: BlockMmio<u32>,
    pub pex_l1_clkreq_n: BlockMmio<u32>,
    pub sata_led_active: BlockMmio<u32>,
    pub spi1_mosi: BlockMmio<u32>,
    pub spi1_miso: BlockMmio<u32>,
    pub spi1_sck: BlockMmio<u32>,
    pub spi1_cs0: BlockMmio<u32>,
    pub spi1_cs1: BlockMmio<u32>,
    pub spi2_mosi: BlockMmio<u32>,
    pub spi2_miso: BlockMmio<u32>,
    pub spi2_sck: BlockMmio<u32>,
    pub spi2_cs0: BlockMmio<u32>,
    pub spi2_cs1: BlockMmio<u32>,
    pub spi4_mosi: BlockMmio<u32>,
    pub spi4_miso: BlockMmio<u32>,
    pub spi4_sck: BlockMmio<u32>,
    pub spi4_cs0: BlockMmio<u32>,
    pub qspi_sck: BlockMmio<u32>,
    pub qspi_cs_n: BlockMmio<u32>,
    pub qspi_io0: BlockMmio<u32>,
    pub qspi_io1: BlockMmio<u32>,
    pub qspi_io2: BlockMmio<u32>,
    pub qspi_io3: BlockMmio<u32>,
    _ra0: BlockMmio<u32>,
    pub dmic1_clk: BlockMmio<u32>,
    pub dmic1_dat: BlockMmio<u32>,
    pub dmic2_clk: BlockMmio<u32>,
    pub dmic2_dat: BlockMmio<u32>,
    pub dmic3_clk: BlockMmio<u32>,
    pub dmic3_dat: BlockMmio<u32>,
    pub gen1_i2c_scl: BlockMmio<u32>,
    pub gen1_i2c_sda: BlockMmio<u32>,
    pub gen2_i2c_scl: BlockMmio<u32>,
    pub gen2_i2c_sda: BlockMmio<u32>,
    pub gen3_i2c_scl: BlockMmio<u32>,
    pub gen3_i2c_sda: BlockMmio<u32>,
    pub cam_i2c_scl: BlockMmio<u32>,
    pub cam_i2c_sda: BlockMmio<u32>,
    pub pwr_i2c_scl: BlockMmio<u32>,
    pub pwr_i2c_sda: BlockMmio<u32>,
    pub uart1_tx: BlockMmio<u32>,
    pub uart1_rx: BlockMmio<u32>,
    pub uart1_rts: BlockMmio<u32>,
    pub uart1_cts: BlockMmio<u32>,
    pub uart2_tx: BlockMmio<u32>,
    pub uart2_rx: BlockMmio<u32>,
    pub uart2_rts: BlockMmio<u32>,
    pub uart2_cts: BlockMmio<u32>,
    pub uart3_tx: BlockMmio<u32>,
    pub uart3_rx: BlockMmio<u32>,
    pub uart3_rts: BlockMmio<u32>,
    pub uart3_cts: BlockMmio<u32>,
    pub uart4_tx: BlockMmio<u32>,
    pub uart4_rx: BlockMmio<u32>,
    pub uart4_rts: BlockMmio<u32>,
    pub uart4_cts: BlockMmio<u32>,
    pub dap1_fs: BlockMmio<u32>,
    pub dap1_din: BlockMmio<u32>,
    pub dap1_dout: BlockMmio<u32>,
    pub dap1_sclk: BlockMmio<u32>,
    pub dap2_fs: BlockMmio<u32>,
    pub dap2_din: BlockMmio<u32>,
    pub dap2_dout: BlockMmio<u32>,
    pub dap2_sclk: BlockMmio<u32>,
    pub dap4_fs: BlockMmio<u32>,
    pub dap4_din: BlockMmio<u32>,
    pub dap4_dout: BlockMmio<u32>,
    pub dap4_sclk: BlockMmio<u32>,
    pub cam1_mclk: BlockMmio<u32>,
    pub cam2_mclk: BlockMmio<u32>,
    pub jtag_rtck: BlockMmio<u32>,
    pub clk_32k_in: BlockMmio<u32>,
    pub clk_32k_out: BlockMmio<u32>,
    pub batt_bcl: BlockMmio<u32>,
    pub clk_req: BlockMmio<u32>,
    pub cpu_pwr_req: BlockMmio<u32>,
    pub pwr_int_n: BlockMmio<u32>,
    pub shutdown: BlockMmio<u32>,
    pub core_pwr_req: BlockMmio<u32>,
    pub aud_mclk: BlockMmio<u32>,
    pub dvfs_pwm: BlockMmio<u32>,
    pub dvfs_clk: BlockMmio<u32>,
    pub gpio_x1_aud: BlockMmio<u32>,
    pub gpio_x3_aud: BlockMmio<u32>,
    pub pcc7: BlockMmio<u32>,
    pub hdmi_cec: BlockMmio<u32>,
    pub hdmi_int_dp_hpd: BlockMmio<u32>,
    pub spdif_out: BlockMmio<u32>,
    pub spdif_in: BlockMmio<u32>,
    pub usb_vbus_en0: BlockMmio<u32>,
    pub usb_vbus_en1: BlockMmio<u32>,
    pub dp_hpd0: BlockMmio<u32>,
    pub wifi_en: BlockMmio<u32>,
    pub wifi_rst: BlockMmio<u32>,
    pub wifi_wake_ap: BlockMmio<u32>,
    pub ap_wake_bt: BlockMmio<u32>,
    pub bt_rst: BlockMmio<u32>,
    pub bt_wake_ap: BlockMmio<u32>,
    pub ap_wake_nfc: BlockMmio<u32>,
    pub nfc_en: BlockMmio<u32>,
    pub nfc_int: BlockMmio<u32>,
    pub gps_en: BlockMmio<u32>,
    pub gps_rst: BlockMmio<u32>,
    pub cam_rst: BlockMmio<u32>,
    pub cam_af_en: BlockMmio<u32>,
    pub cam_flash_en: BlockMmio<u32>,
    pub cam1_pwdn: BlockMmio<u32>,
    pub cam2_pwdn: BlockMmio<u32>,
    pub cam1_strobe: BlockMmio<u32>,
    pub lcd_te: BlockMmio<u32>,
    pub lcd_bl_pwm: BlockMmio<u32>,
    pub lcd_bl_en: BlockMmio<u32>,
    pub lcd_rst: BlockMmio<u32>,
    pub lcd_gpio1: BlockMmio<u32>,
    pub lcd_gpio2: BlockMmio<u32>,
    pub ap_ready: BlockMmio<u32>,
    pub touch_rst: BlockMmio<u32>,
    pub touch_clk: BlockMmio<u32>,
    pub modem_wake_ap: BlockMmio<u32>,
    pub touch_int: BlockMmio<u32>,
    pub motion_int: BlockMmio<u32>,
    pub als_prox_int: BlockMmio<u32>,
    pub temp_alert: BlockMmio<u32>,
    pub button_power_on: BlockMmio<u32>,
    pub button_vol_up: BlockMmio<u32>,
    pub button_vol_down: BlockMmio<u32>,
    pub button_slide_sw: BlockMmio<u32>,
    pub button_home: BlockMmio<u32>,
    pub pa6: BlockMmio<u32>,
    pub pe6: BlockMmio<u32>,
    pub pe7: BlockMmio<u32>,
    pub ph6: BlockMmio<u32>,
    pub pk0: BlockMmio<u32>,
    pub pk1: BlockMmio<u32>,
    pub pk2: BlockMmio<u32>,
    pub pk3: BlockMmio<u32>,
    pub pk4: BlockMmio<u32>,
    pub pk5: BlockMmio<u32>,
    pub pk6: BlockMmio<u32>,
    pub pk7: BlockMmio<u32>,
    pub pl0: BlockMmio<u32>,
    pub pl1: BlockMmio<u32>,
    pub pz0: BlockMmio<u32>,
    pub pz1: BlockMmio<u32>,
    pub pz2: BlockMmio<u32>,
    pub pz3: BlockMmio<u32>,
    pub pz4: BlockMmio<u32>,
    pub pz5: BlockMmio<u32>,
}

impl VolatileStorage for Pinmux {
    unsafe fn make_ptr() -> *const Self {
        PINMUX_BASE as *const _
    }
}

impl Pinmux {
    /// Configures an UART device.
    pub fn configure_uart(&self, uart: &Uart) {
        match uart {
            &Uart::A => {
                self.uart1_tx.write(0);
                self.uart1_rx.write(INPUT | PULL_UP);
                self.uart1_rts.write(0);
                self.uart1_cts.write(INPUT | PULL_DOWN);
            },
            &Uart::B => {
                self.uart2_tx.write(0);
                self.uart2_rx.write(INPUT | PULL_UP);
                self.uart2_rts.write(0);
                self.uart2_cts.write(INPUT | PULL_DOWN);
            },
            &Uart::C => {
                self.uart3_tx.write(0);
                self.uart3_rx.write(INPUT | PULL_UP);
                self.uart3_rts.write(0);
                self.uart3_cts.write(INPUT | PULL_DOWN);
            },
            &Uart::D => {
                self.uart4_tx.write(0);
                self.uart4_rx.write(INPUT | PULL_UP);
                self.uart4_rts.write(0);
                self.uart4_cts.write(INPUT | PULL_DOWN);
            },
            &Uart::E => {
                // Unused on the Switch.
                // TODO(Vale): Nonetheless, figure this out.
            },
            _ => {},
        }
    }

    /// Configures an I²C device.
    pub fn configure_i2c(&self, device: &I2c) {
        match device {
            &I2c::C1 => {
                self.gen1_i2c_scl.write(INPUT);
                self.gen1_i2c_sda.write(INPUT);
            },
            &I2c::C2 => {
                self.gen2_i2c_scl.write(INPUT);
                self.gen2_i2c_sda.write(INPUT);
            },
            &I2c::C3 => {
                self.gen3_i2c_scl.write(INPUT);
                self.gen3_i2c_sda.write(INPUT);
            },
            &I2c::C4 => {
                self.cam_i2c_scl.write(INPUT);
                self.cam_i2c_sda.write(INPUT);
            },
            &I2c::C5 => {
                self.pwr_i2c_scl.write(INPUT);
                self.pwr_i2c_sda.write(INPUT);
            },
            &I2c::C6 => {
                // Unused on the Switch.
                // TODO(Vale): Nonetheless, figure this out.
            },
            _ => {},
        }
    }
}
