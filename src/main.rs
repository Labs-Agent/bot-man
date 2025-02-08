use bot_man::tgbot;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tgbot::deploy().await;
}
