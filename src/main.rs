use std::env;

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
}
