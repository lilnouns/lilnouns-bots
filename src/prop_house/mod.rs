use anyhow::Result;

pub use fetcher::fetch_auctions;

mod fetcher;

pub async fn setup() -> Result<()> {
    let auctions = fetch_auctions().await?;
    let auction_ids: Vec<String> = auctions.iter().map(|a| a.id.to_string()).collect();
    println!("All auctions ids({})", auction_ids.join(","));

    Ok(())
}
