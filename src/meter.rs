use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::SwitchBot;

/// Meter (combined temperature and humidity sensor) status.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MeterStatus {
    /// Temperature (C).
    pub temperature: f64,

    /// Humidity (%).
    pub humidity: f64,

    /// Battery level (%).
    pub battery: Option<f64>,
}

impl SwitchBot {
    pub async fn get_meter_status(&self, id: &str) -> Result<MeterStatus> {
        self.get(&format!("v1.1/devices/{}/status", id)).await
    }
}
