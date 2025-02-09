use std::env;

use alloy::primitives::address;
use bot_man::stat_handler::UserStatsHandler;
use bot_man::tgbot;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    info!("starting the bot...");

    tgbot::deploy().await;

    let address = address!("0xd7756396414101992541102445cfb46edbbf0ae4");
    let mut stats = UserStatsHandler::new(
        "https://sepolia-rollup.arbitrum.io/rpc".to_string(),
        address,
    );

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Err(e) = stats.update_user_stats().await {
                error!("failed to update stats: {:?}", e);
            }
        }
    });
}
