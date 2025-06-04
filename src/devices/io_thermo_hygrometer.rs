//! Indoor/outdoor thermo-hygrometer.
//!
//! https://www.switch-bot.com/products/switchbot-indoor-outdoor-thermo-hygrometer

use anyhow::Result;
use serde::Deserialize;

use crate::SwitchBot;

/// Indoor/outdoor thermo-hygrometer status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IOThermoHygrometerStatus {
    /// Temperature (C).
    pub temperature: f64,

    /// Humidity (%).
    pub humidity: f64,

    /// Battery level (%).
    pub battery: f64,
}

impl SwitchBot {
    /// Get indoor/outdoor thermo-hygrometer status.
    pub async fn get_io_thermo_hygrometer_status(
        &self,
        id: &str,
    ) -> Result<IOThermoHygrometerStatus> {
        self.get(&format!("v1.1/devices/{}/status", id)).await
    }
}
