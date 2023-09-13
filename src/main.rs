use anyhow::Result;

mod cache;
mod prop_house;
mod prop_lot;

#[tokio::main]
async fn main() -> Result<()> {
    prop_lot::setup().await?;
    prop_house::setup().await?;

    Ok(())
}
