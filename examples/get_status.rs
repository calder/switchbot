use anyhow::Result;
use switchbot_api2::SwitchBot;

#[tokio::main]
async fn main() -> Result<()> {
    let switchbot = SwitchBot::new_from_env()?;

    for dev in switchbot.get_devices().await? {
        let status = switchbot.get_status(&dev).await?;
        println!("{}: {:?}", dev.name, status);
    }

    Ok(())
}
