use env_logger::Env;

mod cache;
mod prop_house;
mod prop_lot;

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("BOT_LOG_LEVEL", "trace")
        .write_style_or("BOT_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    prop_lot::setup().await;
    prop_house::setup().await;
}
