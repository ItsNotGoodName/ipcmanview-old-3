use dahua_rpc::{modules::configmanager, Client};

pub async fn run(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let general =
        serde_json::to_string_pretty(&configmanager::General::get(client.rpc().await?).await?)?;
    println!("{}", general);

    // println!("{:?}", configmanager::NTP::get(client.rpc().await?).await?);
    // println!(
    //     "{:?}",
    //     configmanager::VideoInMode::get(client.rpc().await?).await?
    // );

    Ok(())
}
