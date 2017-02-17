extern crate tg_botapi;

use tg_botapi::args;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    let token = &env::var("TOKEN")
        .expect("No bot token provided, please set the environment variable TOKEN");
    let bot_arc = Arc::new(BotApi::new(token));

    let mut update_args = args::GetUpdates::new().timeout(600).offset(0);

    'update_loop: loop {
        let updates = bot_arc.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                let bot = bot_arc.clone();

                thread::spawn(move || {
                    let chat_id = message.chat.id;
                    let msg_id = message.message_id;

                    let message_text = format!("\"{}\"\n    - <i>You, CURRENT_YEAR</i>",
                                               message.text.unwrap_or(String::new()));

                    let _ = bot.send_message(&args::SendMessage::new(&message_text)
                        .chat_id(chat_id)
                        .parse_mode("HTML"));
                });
            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot_arc.get_updates(&update_args);
}
