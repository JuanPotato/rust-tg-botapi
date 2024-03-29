use std::env;

use futures::StreamExt;

use tg_botapi::api::{InputFile, Message, ParseMode, UpdateType};
use tg_botapi::Bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TOKEN")
        .expect("No bot token provided, please set the environment variable TOKEN");

    let bot = Bot::new(token);

    let mut updates = bot.start_polling();

    while let Some(update) = updates.next().await {
        match update.update_type {
            UpdateType::Message(message) => {
                tokio::spawn(handle_message(bot.clone(), message));
            }

            _ => {}
        }
    }

    Ok(())
}

async fn handle_message(bot: Bot, msg: Message) {
    if let Some(ref img) = msg.photo {
        let smallest_photo = &img.iter().min_by_key(|ps| ps.height).unwrap().file_id;
        let req = msg.reply_photo(InputFile::FileId(smallest_photo.into()));

        bot.send(&req).await.unwrap();
    } else if let Some(text) = msg.get_text() {
        let req = msg.reply(text).with_parse_mode(ParseMode::Markdown);

        bot.send(&req).await.unwrap();
    }
}
