#![feature(async_await)]
use futures::{FutureExt, StreamExt, TryFutureExt};

use tg_botapi::methods::ParseMode;
use tg_botapi::types::{Message, UpdateType};
use tg_botapi::Bot;

use std::env;

fn main() {
    let token = env::var("TOKEN")
        .expect("No bot token provided, please set the environment variable TOKEN");

    tokio::run(run_bot(token).boxed().unit_error().compat());
}

async fn run_bot(token: impl Into<String>) {
    let bot = Bot::new(token);

    let mut updates = bot.start_polling();

    while let Some(update) = updates.next().await {
        match update.update_type {
            UpdateType::Message(message) => {
                tokio::spawn(
                    handle_message(bot.clone(), message)
                        .boxed()
                        .unit_error()
                        .compat(),
                );
            }

            _ => {}
        }
    }
}

async fn handle_message(bot: Bot, msg: Message) {
    if let Some(text) = msg.get_text() {
        let mut req = msg.reply(text);
        req.parse_mode = ParseMode::Markdown;

        bot.send(&req).await.unwrap();
    }
}
