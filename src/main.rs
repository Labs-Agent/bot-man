use std::env;

use bot_man::tgbot;
use bot_man::agent;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let agent_url = env::var("AGENT_URL").expect("AGENT_URL must be set");
    let autonome_user = env::var("AUTONOME_USER").expect("AUTONOME_USER must be set");
    let autonome_password = env::var("AUTONOME_PASSWORD").expect("AUTONOME_PASSWORD must be set");

    let message = "Hello";

    let response = agent::get_agent_response(
        &agent_url,
        &autonome_user,
        &autonome_password,
        message,
    ).await.expect("Failed to get response from agent");

    println!("{}", response);

    tgbot::deploy().await;
}
