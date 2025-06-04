use anyhow::Result;
use serde::Deserialize;

use crate::{Dev, Device, SwitchBot};

/// Climate status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ClimateStatus {
    /// Temperature (C).
    pub temperature: f64,

    /// Humidity (%).
    pub humidity: f64,
}

impl SwitchBot {
    /// Get climate status.
    pub async fn get_climate(&self, dev: &Device) -> Result<Option<ClimateStatus>> {
        match dev.typ {
            Dev::Hub2 | Dev::IOThermoHygrometer => Ok(Some(
                self.get(&format!("v1.1/devices/{}/status", dev.id)).await?,
            )),
            Dev::Other(_) => Ok(None),
        }
    }
}
