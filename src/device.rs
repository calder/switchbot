use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::SwitchBot;

/// A single device.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// Unique device identifier.
    #[serde(rename = "deviceId")]
    pub id: String,

    #[serde(rename = "deviceType")]
    pub typ: DeviceType,

    #[serde(rename = "deviceName")]
    pub name: String,

    #[serde(rename = "hubDeviceId")]
    pub hub: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Devices {
    #[serde(rename = "deviceList")]
    pub devices: Vec<Device>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DeviceType {
    #[serde(rename = "Hub 2")]
    Hub2,

    #[serde(rename = "WoIOSensor")]
    WoIOSensor,
}

impl SwitchBot {
    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        Ok(self.get::<Devices>("v1.1/devices").await?.devices)
    }
}
