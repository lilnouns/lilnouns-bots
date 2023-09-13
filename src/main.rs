use anyhow::Result;

mod prop_house;
mod prop_lot;

#[tokio::main]
async fn main() -> Result<()> {
    let ideas = prop_lot::fetch_ideas().await?;
    let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
    println!("All ideas ids({})", ideas_ids.join(","));

    let auctions = prop_house::fetch_auctions().await?;
    let auction_ids: Vec<String> = auctions.iter().map(|a| a.id.to_string()).collect();
    println!("All auctions ids({})", auction_ids.join(","));

    Ok(())
}
