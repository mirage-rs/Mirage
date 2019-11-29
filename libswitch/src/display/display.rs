use register::mmio::ReadWrite;

use crate::{
    clock::{Car, CLOCK_BASE},
    gpio::{Gpio, GpioDirection, GpioLevel, GpioMode},
    i2c::*,
    pinmux::{Pinmux, TRISTATE},
    pmc::Pmc,
    timer::{get_microseconds, usleep},
};
use super::display_config::Config;

static mut DISPLAY_VERSION: u32 = 0;

/// Base address for DI registers.
const DI_BASE: u32 = 0x5420_0000;

/// Base address for DSI registers.
const DSI_BASE: u32 = 0x5430_0000;

/// Base address for MIPI CAL registers.
const MIPI_CAL_BASE: u32 = 0x700E_3000;

/// Waits for DSI to be updated.
unsafe fn dsi_wait(timeout: u32, offset: u32, mask: u32) {
    let register = &*((DSI_BASE + offset) as *const ReadWrite<u32>);
    let end = get_microseconds() + timeout;

    while get_microseconds() < end && register.get() & mask != 0 {
        // Wait.
    }
    usleep(5);
}

/// Initializes the display.
pub fn initialize() {
    let car = &Car::new();
    let pinmux = &Pinmux::new();
    let pmc = &Pmc::new();

    // Power on.
    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x23, 0xD0).unwrap();
    I2c::C5.write_byte(MAX77620_PWR_I2C_ADDR, 0x3D, 0x9).unwrap();

    // Enable MIPI CAL, DSI, DISP1, HOST1X, UART_FST_MIPI_CAL, DSIA LP clocks.
    car.rst_dev_h_clr.set(0x1010000);
    car.clk_enb_h_set.set(0x1010000);
    car.rst_dev_l_clr.set(0x1800_0000);
    car.clk_enb_l_set.set(0x1800_0000);
    car.clk_enb_x_set.set(0x20000);
    car.clk_source_uart_fst_mipi_cal.set(0xA);
    car.clk_enb_w_set.set(0x80000);
    car.clk_source_dsia_lp.set(0xA);

    // DPD idle.
    pmc.io_dpd_req.set(0x4000_0000);
    pmc.io_dpd2_req.set(0x4000_0000);

    // Configure pins.
    pinmux.nfc_en.set(pinmux.nfc_en.get() & !TRISTATE);
    pinmux.nfc_int.set(pinmux.nfc_int.get() & !TRISTATE);
    pinmux.lcd_bl_pwm.set(pinmux.lcd_bl_pwm.get() & !TRISTATE);
    pinmux.lcd_bl_en.set(pinmux.lcd_bl_en.get() & !TRISTATE);
    pinmux.lcd_rst.set(pinmux.lcd_rst.get() & !TRISTATE);

    // Configure Backlight +-5V GPIOs.
    Gpio::LCD_BL_P5V.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_N5V.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_P5V.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_N5V.set_direction(GpioDirection::Output);

    // Enable Backlight +5V.
    Gpio::LCD_BL_P5V.write(GpioLevel::High);

    usleep(10_000);

    // Enable Backlight -5V.
    Gpio::LCD_BL_N5V.write(GpioLevel::High);

    usleep(10_000);

    // Configure Backlight PWM, EN and RST GPIOs.
    Gpio::LCD_BL_PWM.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_EN.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_RST.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_PWM.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_EN.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_RST.set_direction(GpioDirection::Output);

    // Enable Backlight EN.
    Gpio::LCD_BL_EN.write(GpioLevel::High);

    unsafe {
        // Configure display interface and display.
        (*((0x700E_3000 + 0x60) as *const ReadWrite<u32>)).set(0);
    }

    Config::CLOCK_1.execute(CLOCK_BASE as *mut u32);
    Config::DISPLAY_A_1.execute(DI_BASE as *mut u32);
    Config::DSI_INIT.execute(DSI_BASE as *mut u32);

    usleep(10_000);

    // Enable Backlight RST.
    Gpio::LCD_BL_RST.write(GpioLevel::High);

    usleep(60_000);

    unsafe {
        (*((DSI_BASE + 0x3F) as *const ReadWrite<u32>)).set(0x50204);
        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x337);
        (*((DSI_BASE + 0x13) as *const ReadWrite<u32>)).set(1 << 1);

        dsi_wait(250_000, 0x13, 0x3);

        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x406);
        (*((DSI_BASE + 0x13) as *const ReadWrite<u32>)).set(1 << 1);
        dsi_wait(250_000, 0x13, 0x3);

        (*((DSI_BASE + 0xF) as *const ReadWrite<u32>)).set(0x200B);
        dsi_wait(150_000, 0xF, 1 << 3);

        usleep(5_000);

        DISPLAY_VERSION = (*((DSI_BASE + 0x9) as *const ReadWrite<u32>)).get();

        if DISPLAY_VERSION == 0x10 {
            Config::DSI_VER_10_2.execute(DSI_BASE as *mut u32);
        }

        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x1105);
        (*((DSI_BASE + 0x13) as *const ReadWrite<u32>)).set(1 << 1);

        usleep(180_000);

        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x2905);
        (*((DSI_BASE + 0x13) as *const ReadWrite<u32>)).set(1 << 1);

        usleep(20_000);

        Config::DSI_1.execute(DSI_BASE as *mut u32);
        Config::CLOCK_2.execute(CLOCK_BASE as *mut u32);

        (*((DI_BASE + 0x42E) as *const ReadWrite<u32>)).set(4);
        Config::DSI_2.execute(DSI_BASE as *mut u32);

        usleep(10_000);

        Config::MIPI_CAL_1.execute(MIPI_CAL_BASE as *mut u32);
        Config::DSI_3.execute(DSI_BASE as *mut u32);
        Config::MIPI_CAL_2.execute(MIPI_CAL_BASE as *mut u32);

        usleep(10_000);

        Config::DISPLAY_A_2.execute(DI_BASE as *mut u32);
    }
}

/// Turns the display off.
pub fn finish() {
    let car = &Car::new();
    let pinmux = &Pinmux::new();

    // Disable backlight.
    set_backlight(false);

    unsafe {
        (*((DSI_BASE + 0x4E) as *const ReadWrite<u32>)).set(1);
        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x2805);

        (*((DI_BASE + 0x40) as *const ReadWrite<u32>)).set(0x5);
        (*((DSI_BASE + 0x4E) as *const ReadWrite<u32>)).set(0);

        Config::DISPLAY_A_3.execute(DI_BASE as *mut u32);
        Config::DSI_4.execute(DSI_BASE as *mut u32);

        usleep(10_000);

        if DISPLAY_VERSION == 0x10 {
            Config::DSI_VER_10_2.execute(DSI_BASE as *mut u32);
        }

        (*((DSI_BASE + 0xA) as *const ReadWrite<u32>)).set(0x1005);
        (*((DSI_BASE + 0x13) as *const ReadWrite<u32>)).set(1 << 1);

        usleep(50_000);
    }

    // Disable Backlight RST.
    Gpio::LCD_BL_RST.write(GpioLevel::Low);

    usleep(10_000);

    // Disable Backlight -5V.
    Gpio::LCD_BL_N5V.write(GpioLevel::Low);

    usleep(10_000);

    // Disable Backlight +5V.
    Gpio::LCD_BL_P5V.write(GpioLevel::Low);

    usleep(10_000);

    // Disable clocks.
    car.rst_dev_h_set.set(0x1010000);
    car.clk_enb_h_clr.set(0x1010000);
    car.rst_dev_l_set.set(0x1800_0000);
    car.clk_enb_l_clr.set(0x1800_0000);

    unsafe {
        (*((DSI_BASE + 0x4B) as *const ReadWrite<u32>)).set(0x10F010F);
        (*((DSI_BASE + 0xB) as *const ReadWrite<u32>)).set(0);
    }

    // Backlight PWM.
    Gpio::LCD_BL_PWM.set_mode(GpioMode::SFIO);

    pinmux.lcd_bl_pwm.set((pinmux.lcd_bl_pwm.get() & !TRISTATE) | TRISTATE);
    pinmux.lcd_bl_pwm.set(((pinmux.lcd_bl_pwm.get() >> 2) << 2) | 1);
}

/// Shows a single color on the display.
pub fn color_screen(color: u32) {
    Config::ONE_COLOR.execute(DI_BASE as *mut u32);

    // Configure display to show a single color.
    unsafe {
        let cmd_state_control_reg = &*((DI_BASE + 0x41) as *const ReadWrite<u32>);

        (*((DI_BASE + 0xB80) as *const ReadWrite<u32>)).set(0);
        (*((DI_BASE + 0xD80) as *const ReadWrite<u32>)).set(0);
        (*((DI_BASE + 0xF80) as *const ReadWrite<u32>)).set(0);
        (*((DI_BASE + 0x4E4) as *const ReadWrite<u32>)).set(color);
        cmd_state_control_reg.set((cmd_state_control_reg.get() & 0xFFFF_FFFE) | (1 << 0));
    }

    usleep(35_000);

    set_backlight(true);
}

/// Turns the backlight on/off.
pub fn set_backlight(enable: bool) {
    let level = if enable { GpioLevel::High } else { GpioLevel::Low };

   // Enable backlight PWM.
    Gpio::LCD_BL_PWM.write(level);
}

/// Initializes display in full 1280x720 resolution.
/// (B8G8R8A8, line stride 768, framebuffer size = 1280*768*4 bytes).
pub fn initialize_framebuffer(address: u32) -> u32 {
    unimplemented!();
}
