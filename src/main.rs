use std::env;

use bot_man::agent;
use bot_man::tgbot;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    env::remove_var("RUST_APP_LOG");

    let agent_url = env::var("AGENT_URL").expect("AGENT_URL must be set");
    let autonome_user = env::var("AUTONOME_USER").expect("AUTONOME_USER must be set");
    let autonome_password = env::var("AUTONOME_PASSWORD").expect("AUTONOME_PASSWORD must be set");

    let message = "Hello";

    let agent = agent::Agent::new(agent_url, autonome_user, autonome_password);

    info!("connecting to agent...");
    let response = agent
        .get_response(message)
        .await
        .expect("Failed to get response from agent");

    info!("response from agent: {}", response);

    info!("starting the bot...");

    tgbot::deploy().await;
}
