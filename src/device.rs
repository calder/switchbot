use anyhow::Result;
use serde::Deserialize;

use crate::{Hub2Status, IOThermoHygrometerStatus, SwitchBot};

/// A single device.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Device {
    /// Unique device identifier.
    #[serde(rename = "deviceId")]
    pub id: String,

    /// Human readable device name.
    #[serde(rename = "deviceName")]
    pub name: String,

    /// Device type.
    #[serde(rename = "deviceType")]
    pub typ: Dev,

    /// ID of the hub the device is connected to.
    #[serde(rename = "hubDeviceId")]
    pub hub: String,
}

/// A list of devices.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Devices {
    /// Devices.
    #[serde(rename = "deviceList")]
    pub devices: Vec<Device>,
}

/// Device type.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(field_identifier)]
pub enum Dev {
    /// Hub 2.
    #[serde(rename = "Hub 2")]
    Hub2,

    /// Thermo-hygrometer.
    #[serde(rename = "WoIOSensor")]
    IOThermoHygrometer,

    /// Other device type.
    Other(String),
}

/// Device status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum DevStatus {
    Hub2(Hub2Status),
    IOThermoHygrometer(IOThermoHygrometerStatus),
    Other,
}

impl SwitchBot {
    /// Get all devices.
    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        Ok(self.get::<Devices>("v1.1/devices").await?.devices)
    }

    /// Get device status.
    pub async fn get_status(&self, dev: &Device) -> Result<DevStatus> {
        match dev.typ {
            Dev::Hub2 => Ok(DevStatus::Hub2(self.get_hub2_status(&dev.id).await?)),
            Dev::IOThermoHygrometer => Ok(DevStatus::IOThermoHygrometer(
                self.get_io_thermo_hygrometer_status(&dev.id).await?,
            )),
            Dev::Other(_) => Ok(DevStatus::Other),
        }
    }
}
