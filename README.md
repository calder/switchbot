# 🤖 SwitchBot

[![Documentation](https://docs.rs/switchbot-api2/badge.svg)](https://docs.rs/switchbot-api2) [![Latest Version](https://img.shields.io/crates/v/switchbot-api2.svg)](https://crates.io/crates/switchbot-api2)

Rust client for controlling [SwitchBot](https://www.switch-bot.com/) devices.

## 🛠️ Basic Usage

Set `SWITCHBOT_TOKEN` to your API token, then:
```rust
let switchbot = switchbot_api2::SwitchBot::new_from_env()?;
for d in switchbot.get_devices().await? {
    println!("{}: {:?}", d.name, switchbot.get_status(&d).await?);
}
```

See [examples](examples) and [docs](https://docs.rs/switchbot-api2) for more.

## 📟 [Supported Devices](src/devices)

| Device | 🌡️ | 💦 | 🔋 |
|-|-|-|-|
| [Hub 2](https://www.switch-bot.com/pages/switchbot-hub-2) | ✅ | ✅ | - |
| [Indoor/Outdoor Thermo-Hygrometer](https://www.switch-bot.com/products/switchbot-indoor-outdoor-thermo-hygrometer) | ✅ | ✅ | ✅ |

## 📖 [Supported APIs](src/api)

| API | 🌡️ | 💦 | 🔋 |
|-|-|-|-|
| [Temperature](src/api/temperature.rs) | ✅ | - | - |
| [Humidity](src/api/humidity.rs) | - | ✅ | - |
| [Battery](src/api/battery.rs) | - | - | ✅ |
| [Climate](src/api/climate.rs) | ✅ | ✅ | - |

## ❤️ Contributing

Pull requests for new device types or anything else are welcome!
