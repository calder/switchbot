//! Hub 2.
//!
//! https://us.switch-bot.com/products/switchbot-hub-2

use anyhow::Result;
use serde::Deserialize;

use crate::SwitchBot;

/// Hub 2 status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Hub2Status {
    /// Temperature (C).
    pub temperature: f64,

    /// Humidity (%).
    pub humidity: f64,
}

impl SwitchBot {
    /// Get Hub 2 status.
    pub async fn get_hub2_status(&self, id: &str) -> Result<Hub2Status> {
        self.get(&format!("v1.1/devices/{}/status", id)).await
    }
}
