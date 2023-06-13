use dahua_rpc::{modules::config, Client};

pub async fn run(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "DisableEmailLinkage={:#?}",
        config::DisableEmailLinkage::get(client.rpc().await?)
            .await
            .ok()
    );
    println!(
        "DisableLinkageTimeSection={:#?}",
        config::DisableLinkageTimeSection::get(client.rpc().await?)
            .await
            .ok()
    );
    println!(
        "VideoInMode={:#?}",
        config::VideoInMode::get(client.rpc().await?).await.ok()
    );
    println!(
        "NTP={:#?}",
        config::NTP::get(client.rpc().await?).await.ok()
    );
    println!(
        "Email={:#?}",
        config::Email::get(client.rpc().await?).await.ok()
    );
    println!(
        "Locales={:#?}",
        config::Locales::get(client.rpc().await?).await.ok()
    );
    println!(
        "General={:#?}",
        config::General::get(client.rpc().await?).await.ok()
    );

    Ok(())
}
