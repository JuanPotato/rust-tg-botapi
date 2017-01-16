extern crate tg_botapi;

use tg_botapi::args;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    let token = &env::var("TOKEN").expect("No bot token provided, please set the environment variable TOKEN");
    let bot_arc = Arc::new(BotApi::new(token));

    let mut update_args = args::GetUpdates {
        offset: Some(0),
        limit: None,
        timeout: Some(600),
        allowed_updates: None,
    };
            
    'update_loop: loop {
        let updates = bot_arc.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                let bot = bot_arc.clone();

                let chat_id = message.chat.id;
                let msg_id = message.message_id;

                let message_text = format("\"{}\"\n    - <i>You, CURRENT_YEAR</i>",
                                          message.text.unwrap_or(String::new()));

                thread::spawn(move || {
                    let _ = bot.send_message(&args::SendMessage {
                        chat_id: Some(chat_id),
                        chat_username: None,
                        text: ,
                        parse_mode: Some("HTML"),
                        disable_web_page_preview: None,
                        disable_notification: None,
                        reply_to_message_id: None,
                        reply_markup: None,
                    });
                });
            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot.get_updates(&update_args);
}