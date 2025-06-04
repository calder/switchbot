use anyhow::Result;
use serde::Deserialize;

use crate::{Dev, Device, SwitchBot};

/// Temperature status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TemperatureStatus {
    /// Temperature (C).
    temperature: f64,
}

impl SwitchBot {
    /// Get temperature (C).
    pub async fn get_temperature(&self, dev: &Device) -> Result<Option<f64>> {
        match dev.typ {
            Dev::Hub2 | Dev::IOThermoHygrometer => Ok(Some(
                self.get::<TemperatureStatus>(&format!("v1.1/devices/{}/status", dev.id))
                    .await?
                    .temperature,
            )),
            Dev::Other(_) => Ok(None),
        }
    }
}
