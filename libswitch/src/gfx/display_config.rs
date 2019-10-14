use core::ptr::write_volatile;

use register::mmio::ReadWrite;

/// Wrapper around a display configuration value.
#[derive(Clone, Copy, Debug)]
struct ConfigTable {
    /// The offset to write the value to.
    offset: u32,
    /// The actual configuration value.
    value: u32,
}

#[derive(Clone, Debug)]
pub struct Config<'a> {
    tables: &'a [ConfigTable],
}

impl Config {
    pub const CLOCK_1: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x4E,
                value: 0x40000000,
            },
            ConfigTable {
                offset: 0x34,
                value: 0x4830A001,
            },
            ConfigTable {
                offset: 0x36,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x37,
                value: 0x2D0AAA,
            },
        ],
    };

    pub const DISPLAY_A_1: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x40,
                value: 0,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x100,
            },
            ConfigTable {
                offset: 0x41,
                value: 1,
            },
            ConfigTable {
                offset: 0x043,
                value: 0x54,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x100,
            },
            ConfigTable {
                offset: 0x41,
                value: 1,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x480,
                value: 0,
            },
            ConfigTable {
                offset: 0x403,
                value: 0,
            },
            ConfigTable {
                offset: 0x404,
                value: 0,
            },
            ConfigTable {
                offset: 0x36,
                value: 0x50155,
            },
            ConfigTable {
                offset: 0x01,
                value: 0x100,
            },
            ConfigTable {
                offset: 0x28,
                value: 0x109,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF00,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF,
            },
            ConfigTable {
                offset: 0x40,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x70E,
                value: 0,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x611,
                value: 0xF0,
            },
            ConfigTable {
                offset: 0x612,
                value: 0x12A,
            },
            ConfigTable {
                offset: 0x613,
                value: 0,
            },
            ConfigTable {
                offset: 0x614,
                value: 0x198,
            },
            ConfigTable {
                offset: 0x615,
                value: 0x39B,
            },
            ConfigTable {
                offset: 0x616,
                value: 0x32F,
            },
            ConfigTable {
                offset: 0x617,
                value: 0x204,
            },
            ConfigTable {
                offset: 0x618,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x430,
                value: 0x8,
            },
            ConfigTable {
                offset: 0x42F,
                value: 0,
            },
            ConfigTable {
                offset: 0x307,
                value: 0x1000000,
            },
            ConfigTable {
                offset: 0x309,
                value: 0,
            },
            ConfigTable {
                offset: 0x4E4,
                value: 0,
            },
            ConfigTable {
                offset: 0x300,
                value: 0,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF00,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x716,
                value: 0x10000FF,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x716,
                value: 0x10000FF,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x716,
                value: 0x10000FF,
            },
            ConfigTable {
                offset: 0x031,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0,
            },
            ConfigTable {
                offset: 0x32,
                value: 0,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF00,
            },
            ConfigTable {
                offset: 0x41,
                value: 0xF,
            },
        ],
    };

    pub const DSI_INIT: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0xA,
                value: 0,
            },
            ConfigTable {
                offset: 0xC,
                value: 0,
            },
            ConfigTable {
                offset: 0xD,
                value: 0,
            },
            ConfigTable {
                offset: 0xE,
                value: 0,
            },
            ConfigTable {
                offset: 0x1B,
                value: 0,
            },
            ConfigTable {
                offset: 0x1C,
                value: 0,
            },
            ConfigTable {
                offset: 0x1D,
                value: 0,
            },
            ConfigTable {
                offset: 0x1E,
                value: 0,
            },
            ConfigTable {
                offset: 0x1C5,
                value: 0,
            },
            ConfigTable {
                offset: 0x33,
                value: 0,
            },
            ConfigTable {
                offset: 0x25,
                value: 0,
            },
            ConfigTable {
                offset: 0x27,
                value: 0,
            },
            ConfigTable {
                offset: 0x29,
                value: 0,
            },
            ConfigTable {
                offset: 0x2B,
                value: 0,
            },
            ConfigTable {
                offset: 0x2D,
                value: 0,
            },
            ConfigTable {
                offset: 0x24,
                value: 0,
            },
            ConfigTable {
                offset: 0x26,
                value: 0,
            },
            ConfigTable {
                offset: 0x28,
                value: 0,
            },
            ConfigTable {
                offset: 0x2A,
                value: 0,
            },
            ConfigTable {
                offset: 0x2C,
                value: 0,
            },
            ConfigTable {
                offset: 0x2E,
                value: 0,
            },
            ConfigTable {
                offset: 0x10,
                value: 0,
            },
            ConfigTable {
                offset: 0x4C,
                value: 0,
            },
            ConfigTable {
                offset: 0x11,
                value: 0x18,
            },
            ConfigTable {
                offset: 0x12,
                value: 0x1E0,
            },
            ConfigTable {
                offset: 0x13,
                value: 0,
            },
            ConfigTable {
                offset: 0x1A,
                value: 0,
            },
            ConfigTable {
                offset: 0x34,
                value: 0,
            },
            ConfigTable {
                offset: 0x35,
                value: 0,
            },
            ConfigTable {
                offset: 0x36,
                value: 0,
            },
            ConfigTable {
                offset: 0x37,
                value: 0,
            },
            ConfigTable {
                offset: 0x4F,
                value: 0,
            },
            ConfigTable {
                offset: 0x3C,
                value: 0x6070601,
            },
            ConfigTable {
                offset: 0x3D,
                value: 0x40A0E05,
            },
            ConfigTable {
                offset: 0x3E,
                value: 0x30109,
            },
            ConfigTable {
                offset: 0x3F,
                value: 0x190A14,
            },
            ConfigTable {
                offset: 0x44,
                value: 0x2000ffff,
            },
            ConfigTable {
                offset: 0x45,
                value: 0x7652000,
            },
            ConfigTable {
                offset: 0x46,
                value: 0,
            },
            ConfigTable {
                offset: 0x4B,
                value: 0,
            },
            ConfigTable {
                offset: 0xB,
                value: 1,
            },
            ConfigTable {
                offset: 0xB,
                value: 1,
            },
            ConfigTable {
                offset: 0xB,
                value: 0,
            },
            ConfigTable {
                offset: 0xB,
                value: 0,
            },
            ConfigTable {
                offset: 0x4F,
                value: 0,
            },
            ConfigTable {
                offset: 0x3C,
                value: 0x6070601,
            },
            ConfigTable {
                offset: 0x3D,
                value: 0x40A0E05,
            },
            ConfigTable {
                offset: 0x3E,
                value: 0x30118,
            },
            ConfigTable {
                offset: 0x3F,
                value: 0x190A14,
            },
            ConfigTable {
                offset: 0x44,
                value: 0x2000ffff,
            },
            ConfigTable {
                offset: 0x45,
                value: 0x13432000,
            },
            ConfigTable {
                offset: 0x46,
                value: 0,
            },
            ConfigTable {
                offset: 0xF,
                value: 0x102003,
            },
            ConfigTable {
                offset: 0x10,
                value: 0x31,
            },
            ConfigTable {
                offset: 0xB,
                value: 1,
            },
            ConfigTable {
                offset: 0xB,
                value: 1,
            },
            ConfigTable {
                offset: 0x12,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x13,
                value: 0,
            },
            ConfigTable {
                offset: 0x14,
                value: 0,
            },
            ConfigTable {
                offset: 0x1A,
                value: 0,
            },
        ],
    };

    pub const DSI_CONIG_VER_10: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0xA,
                value: 0x439,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x9483FFB9,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xBD15,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x1939,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAAAAAD8,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAAAAAEB,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAEBAAAA,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAAAAAAA,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAAAAAEB,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAAEBAAAA,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xAA,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x1BD15,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x2739,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFD8,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFF,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x2BD15,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xF39,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFD8,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFFFF,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xFFFFFF,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xBD15,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x6D915,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
            ConfigTable {
                offset: 0xA,
                value: 0x439,
            },
            ConfigTable {
                offset: 0xA,
                value: 0xB9,
            },
            ConfigTable {
                offset: 0x13,
                value: 0x2,
            },
        ],
    };

    pub const DSI_CONIG_1: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x4F,
                value: 0x0,
            },
            ConfigTable {
                offset: 0x3C,
                value: 0x6070601,
            },
            ConfigTable {
                offset: 0x3D,
                value: 0x40A0E05,
            },
            ConfigTable {
                offset: 0x3E,
                value: 0x30172,
            },
            ConfigTable {
                offset: 0x3F,
                value: 0x190A14,
            },
            ConfigTable {
                offset: 0x44,
                value: 0x20000A40,
            },
            ConfigTable {
                offset: 0x45,
                value: 0x5A2F2000,
            },
            ConfigTable {
                offset: 0x46,
                value: 0x0,
            },
            ConfigTable {
                offset: 0x23,
                value: 0x40000208,
            },
            ConfigTable {
                offset: 0x27,
                value: 0x40000308,
            },
            ConfigTable {
                offset: 0x2B,
                value: 0x40000308,
            },
            ConfigTable {
                offset: 0x25,
                value: 0x40000308,
            },
            ConfigTable {
                offset: 0x29,
                value: 0x3F3B2B08,
            },
            ConfigTable {
                offset: 0x2A,
                value: 0x2CC,
            },
            ConfigTable {
                offset: 0x2D,
                value: 0x3F3B2B08,
            },
            ConfigTable {
                offset: 0x2E,
                value: 0x2CC,
            },
            ConfigTable {
                offset: 0x34,
                value: 0xCE0000,
            },
            ConfigTable {
                offset: 0x35,
                value: 0x87001A2,
            },
            ConfigTable {
                offset: 0x36,
                value: 0x190,
            },
            ConfigTable {
                offset: 0x37,
                value: 0x190,
            },
            ConfigTable {
                offset: 0xF,
                value: 0x0,
            },
        ],
    };

    pub const CLOCK_2: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x34,
                value: 0x4810C001,
            },
            ConfigTable {
                ofset: 0x36,
                value: 0x20,
            },
            ConfigTable {
                ofset: 0x37,
                value: 0x2DFC00,
            },
        ],
    };

    pub const ONE_COLOR: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x32,
                value: 0x20,
            },
        ],
    };

    pub const FRAMEBUFFER: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x703,
                value: 0xC,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x704,
                value: 0,
            },
            ConfigTable {
                offset: 0x707,
                value: 0,
            },
            ConfigTable {
                offset: 0x708,
                value: 0,
            },
            ConfigTable {
                offset: 0x706,
                value: 0x5000_B40,
            },
            ConfigTable {
                offset: 0x709,
                value: 0x1000_1000,
            },
            ConfigTable {
                offset: 0x705,
                value: 0x5000_2D0,
            },
            ConfigTable {
                offset: 0x70A,
                value: 0x5A00_B40,
            },
            ConfigTable {
                offset: 0x702,
                value: 0,
            },
            ConfigTable {
                offset: 0x80B,
                value: 0,
            },
            ConfigTable {
                offset: 0x800,
                value: 0xC000_0000,
            },
            ConfigTable {
                offset: 0x806,
                value: 0,
            },
            ConfigTable {
                offset: 0x808,
                value: 0,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0x4000_0000,
            },
            ConfigTable {
                offset: 0x32,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x300,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x3,
            },
        ],
    };
}

impl Config {
    pub fn execute(&self, base: *const u32) {
        for table in self.tables {
            unsafe {
                write_volatile(base.offset(table.offset as isize), table.value);
            }
        }
    }
}
