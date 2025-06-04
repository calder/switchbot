use anyhow::Result;
use switchbot::SwitchBot;

#[tokio::main]
async fn main() -> Result<()> {
    let switchbot = SwitchBot::new_from_env()?;

    for dev in switchbot.get_devices().await? {
        let status = switchbot.get_meter_status(&dev.id).await?;
        println!("{}: {:?}", dev.name, status);
    }

    Ok(())
}
