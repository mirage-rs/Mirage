use libswitch::i2c::*;
use libswitch::clock::CLOCK_BASE;
use libswitch::pmc::PMC_BASE;
use libswitch::pinmux::PINMUX_BASE;
use libswitch::gfx::display_config::*;

/// Initializes the display.
pub fn display_init() {
    let i2c = I2c::C5;
    i2c.write_byte(MAX77620_PWR_I2C_ADDR, 0x23, 0xD0).unwrap();
    i2c.write_byte(MAX77620_PWR_I2C_ADDR, 0x3D, 0x09).unwrap();

    // Clear reset DSI, MIPI_CAL
    (unsafe { &(*((CLOCK_BASE + 0x30C) as *const ReadWrite<u32>)) }).set(0x1010000);
    // Set enable clock DSI, MIPI_CAL
    (unsafe { &(*((CLOCK_BASE + 0x328) as *const ReadWrite<u32>)) }).set(0x1010000);

    // Clear reset DSIP1, HOST1X
    (unsafe { &(*((CLOCK_BASE + 0x304) as *const ReadWrite<u32>)) }).set(0x18000000);
    // Set enable clock DISP1, HOST1X
    (unsafe { &(*((CLOCK_BASE + 0x320) as *const ReadWrite<u32>)) }).set(0x18000000);

    // Set enable clock UART_FST_MIPI_CAL
    (unsafe { &(*((CLOCK_BASE + 0x284) as *const ReadWrite<u32>)) }).set(0x20000);
    // Set PLLP_OUT3 and div 6 (17MHz)
    (unsafe { &(*((CLOCK_BASE + 0x66C) as *const ReadWrite<u32>)) }).set(0xA);

    // Set enable clock DSIA_LP
    (unsafe { &(*((CLOCK_BASE + 0x448) as *const ReadWrite<u32>)) }).set(0x80000);
    // Set PLLP_OUT and div 6 (68MHz)
    (unsafe { &(*((CLOCK_BASE + 0x620) as *const ReadWrite<u32>)) }).set(0xA);

    // Disable deap power down
    (unsafe { &(*((PMC_BASE + 0x1B8) as *const ReadWrite<u32>)) }).set(0x40000000);
    (unsafe { &(*((PMC_BASE + 0x1C0) as *const ReadWrite<u32>)) }).set(0x40000000);

    // Config LCD and Backlight pins
    (unsafe { &(*((PMC_BASE + 0x1C0) as *const ReadWrite<u32>)) }).set(0x40000000);
}

/// Turns the display of
pub fn display_end() {
    unimplemented!();
}

/// Show one single color on the display.
pub fn display_color_screen(color: u32) {
    unimplemented!();
}

/// Turn the backlight on / off
pub fn display_backlight(enable: bool) {
   unimplemented!();
}

/// Init display in full 1280x720 resolution
/// (B8G8R8A8, line stride 768, framebuffer size = 1280*768*4 bytes).
pub fn display_init_framebuffer(address: u32) -> u32 {
   unimplemented!();
}
