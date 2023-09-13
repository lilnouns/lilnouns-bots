use clap::{Parser, Subcommand};
use env_logger::Env;

mod cache;
mod prop_house;
mod prop_lot;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs setups
    Setup {
        /// Forces the setup
        #[arg(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("BOT_LOG_LEVEL", "trace")
        .write_style_or("BOT_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup { force }) => {
            if *force {
                println!("Running setup...");
                // Uncomment these lines to actually run setup
                prop_lot::setup().await;
                prop_house::setup().await;
            } else {
                println!("Setup not forced, not running...");
            }
        }
        None => {}
    }
}
