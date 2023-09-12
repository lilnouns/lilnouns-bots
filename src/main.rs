use anyhow::Result;

mod prop_lot;

#[tokio::main]
async fn main() -> Result<()> {
    let ideas = prop_lot::fetch_ideas().await?;

    let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
    let joined_ids = ideas_ids.join(",");

    println!("All ideas ids({})", joined_ids);

    Ok(())
}
