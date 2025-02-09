use std::env;

use log::info;
use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
};

use teloxide::{prelude::*, utils::command::BotCommands};

use crate::agent;
use crate::stats::*;


type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn hello_world(bot: Bot, message: Message) -> HandlerResult {
    if let Some(msg) = message.text() {
        info!("sending message=\"{}\" to agent...", msg);

        let agent_url = env::var("AGENT_URL").expect("AGENT_URL must be set");
        let autonome_user = env::var("AUTONOME_USER").expect("AUTONOME_USER must be set");
        let autonome_password = env::var("AUTONOME_PASSWORD").expect("AUTONOME_PASSWORD must be set");

        let agent= agent::Agent::new(agent_url, autonome_user, autonome_password);

        let res = agent.get_response(&msg).await.unwrap();

        info!("response from agent: {}", res);

        bot.send_message(message.chat.id, res).send().await?;
    }
    Ok(())
}

fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    Update::filter_message().endpoint(hello_world)
}

pub async fn deploy() {
    let bot = Bot::from_env();

    info!("bot connected!");

    // Dispatcher::builder(bot, handler_tree())
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;

    Command::repl(bot, answer_to_commands).await;

}

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide_tests::{MockBot, MockMessageText};

    #[tokio::test]
    async fn test_hello_world() {
        let mock_message = MockMessageText::new().text("Hi!");
        let bot = MockBot::new(mock_message, handler_tree());
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");
        assert_eq!(message.text(), Some("Hello World!"));

        let message_text = responses
            .sent_messages_text
            .last()
            .expect("No sent messages were detected!");
        assert_eq!(message_text.message.text(), Some("Hello World!"));
        assert_eq!(message_text.bot_request.parse_mode, None);
    }
}
