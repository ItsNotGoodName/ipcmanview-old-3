use dahua_rpc::{modules::configmanager, Client};

pub async fn run(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        serde_json::to_string_pretty(&configmanager::Email::get(client.rpc().await?).await?)?
    );

    // configmanager::Email::get(client.rpc().await?)
    //     .await
    //     .map(|mut r| {
    //         println!("{:?}", r);
    //         r.port = 25;
    //         return r;
    //     })?
    //     .set(client.rpc().await?)
    //     .await?;

    // println!("{:?}", configmanager::NTP::get(client.rpc().await?).await?);
    // println!(
    //     "{:?}",
    //     configmanager::VideoInMode::get(client.rpc().await?).await?
    // );

    Ok(())
}
