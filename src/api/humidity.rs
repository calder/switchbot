use anyhow::Result;
use serde::Deserialize;

use crate::{Dev, Device, SwitchBot};

/// Humidity status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HumidityStatus {
    /// Humidity (%).
    humidity: f64,
}

impl SwitchBot {
    /// Get humidity (%).
    pub async fn get_humidity(&self, dev: &Device) -> Result<Option<f64>> {
        match dev.typ {
            Dev::Hub2 | Dev::IOThermoHygrometer => Ok(Some(
                self.get::<HumidityStatus>(&format!("v1.1/devices/{}/status", dev.id))
                    .await?
                    .humidity,
            )),
            Dev::Other(_) => Ok(None),
        }
    }
}
