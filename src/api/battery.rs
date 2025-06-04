use anyhow::Result;
use serde::Deserialize;

use crate::{Dev, Device, SwitchBot};

/// Battery status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BatteryStatus {
    /// Battery (%).
    battery: f64,
}

impl SwitchBot {
    /// Get battery (%).
    pub async fn get_battery(&self, dev: &Device) -> Result<Option<f64>> {
        match dev.typ {
            Dev::IOThermoHygrometer => Ok(Some(
                self.get::<BatteryStatus>(&format!("v1.1/devices/{}/status", dev.id))
                    .await?
                    .battery,
            )),
            Dev::Hub2 => Ok(None),
            Dev::Other(_) => Ok(None),
        }
    }
}
