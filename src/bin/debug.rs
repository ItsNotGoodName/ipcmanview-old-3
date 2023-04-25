use anyhow::Result;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = Utc::now().to_rfc3339();
    println!("{time}");
    Ok(())
}
