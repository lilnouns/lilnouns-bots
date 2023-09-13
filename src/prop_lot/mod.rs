use anyhow::Result;

pub use fetcher::fetch_ideas;

mod fetcher;

pub async fn setup() -> Result<()> {
    let ideas = fetch_ideas().await?;
    let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
    println!("All ideas ids({})", ideas_ids.join(","));

    Ok(())
}
