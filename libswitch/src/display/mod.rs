//! Tegra210 Display Controller driver.
//!
//! # Description
//!
//! The Tegra X1 architecture has two entirely independent display controllers.
//! They can support two independent display devices, typically a local display
//! panel and an external HDMI TV or DP monitor. Other configurations are possible
//! such as two local panels. Each display controller can run at a different clock
//! rate and drive a different resolution panel.

pub use display::*;
pub use writer::{Writer, WRITER};
pub use display_config::FRAMEBUFFER_ADDRESS;

mod display;
mod display_config;
mod writer;
