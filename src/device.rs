use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::SwitchBot;

/// A single device.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// Unique device identifier.
    #[serde(rename = "deviceId")]
    pub id: String,

    /// Human readable device name.
    #[serde(rename = "deviceName")]
    pub name: String,

    /// Device type.
    #[serde(rename = "deviceType")]
    pub typ: String,

    /// ID of the hub the device is connected to.
    #[serde(rename = "hubDeviceId")]
    pub hub: String,
}

/// A list of devices.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Devices {
    /// Devices.
    #[serde(rename = "deviceList")]
    pub devices: Vec<Device>,
}

impl SwitchBot {
    /// Get all devices.
    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        Ok(self.get::<Devices>("v1.1/devices").await?.devices)
    }
}
