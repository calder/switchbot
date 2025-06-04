//! # 🤖 Switchbot
//!
//! Rust client for controlling [SwitchBot](https://www.switch-bot.com/) devices.
//!
//! ## Basic Usage
//!
//! ```rust
//! let switchbot = switchbot_api2::SwitchBot::new_from_env()?;
//! for d in switchbot.get_devices().await? {
//!     println!("{}: {:?}", d.name, switchbot.get_status(&d).await?);
//! }
//! ```

mod api;
mod device;
mod devices;
mod switchbot;

pub use api::*;
pub use device::*;
pub use devices::*;
pub use switchbot::*;

pub use ::anyhow::{Error, Result};
