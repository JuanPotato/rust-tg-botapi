extern crate tg_botapi;

use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    let token = &env::var("TOKEN").expect("No bot token provided, please set the environment variable TOKEN");
    let bot = Arc::new(BotApi::new(token));

    let mut update_args = args::GetUpdates::new().timeout(600).offset(0);
            
    'update_loop: loop {
        let updates = bot.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                // let from = message.from.unwrap();

                let message_text = message.text.unwrap_or(String::new());
                let mut split_text = message_text.split_whitespace();

                if let Some(cmd) = split_text.next() {
                    match cmd {
                        "/exit" => {
                            let _ = bot.send_message(&args::SendMessage::new("Goodbye!")
                                            .chat_id(message.chat.id)
                                            .reply_to_message_id(message.message_id));
                            break 'update_loop;
                        }
                        "/start" | "/help" => {
                            let _ = bot.send_message(&args::SendMessage::new("Hi, I'm a bot!")
                                            .chat_id(message.chat.id)
                                            .reply_to_message_id(message.message_id));
                        }
                        "/photo" => {
                            let _ = bot.send_photo(&args::SendPhoto::new()
                                            .chat_id(message.chat.id)
                                            .reply_to_message_id(message.message_id)
                                            .photo("/home/juan/Documents/JuanPotato.png"));
                        }
                        "/edit" => {
                            let sent = bot.send_message(&args::SendMessage::new("Editing")
                                            .chat_id(message.chat.id)
                                            .reply_to_message_id(message.message_id));

                            match sent {
                                Ok(sent_message) => {
                                    let mut edit_args = args::EditMessageText::new("Edited")
                                            .chat_id(message.chat.id)
                                            .message_id(sent_message.message_id)
                                            .parse_mode("Markdown");

                                    if let Some(arg) = split_text.next() {
                                        edit_args.text = &arg;
                                    }

                                    let _ = bot.edit_message_text(&edit_args);
                                }
                                Err(_) => {}
                            }
                        }
                        "/thread" | "/threads" => {
                            let bot1 = bot.clone();
                            let bot2 = bot.clone();
                            let bot3 = bot.clone();

                            let chat_id = message.chat.id;
                            let msg_id = message.message_id;

                            thread::spawn(move || {
                                let _ = bot1.send_message(&args::SendMessage::new("Thread 1")
                                            .chat_id(chat_id)
                                            .reply_to_message_id(msg_id));
                            });
                            thread::spawn(move || {
                                let _ = bot2.send_message(&args::SendMessage::new("Thread 2")
                                            .chat_id(chat_id)
                                            .reply_to_message_id(msg_id));
                            });
                            thread::spawn(move || {
                                let _ = bot3.send_message(&args::SendMessage::new("Thread 3")
                                            .chat_id(chat_id)
                                            .reply_to_message_id(msg_id));
                            });
                        }
                        "/inline" => {
                            let keyboard = [
                                    &[
                                        types::KeyboardButton::new("Yes"),
                                        types::KeyboardButton::new("No"),
                                    ][..],
                                    &[
                                        types::KeyboardButton::new("Eh"),
                                        types::KeyboardButton::new("He"),
                                    ][..]
                                ]; // Find prettier way to do this :\
                              
                            let _ = bot.send_message(&args::SendMessage
                                ::new("Yes or No?")
                                .chat_id(message.chat.id)
                                .reply_markup(&types::ReplyMarkup::new_reply_keyboard(&keyboard[..]))
                            );
                        }
                        "/clear" | "No" => {                              
                            let _ = bot.send_message(&args::SendMessage
                                ::new("Me too")
                                .chat_id(message.chat.id)
                                .reply_markup(&types::ReplyMarkup::new_reply_keyboard_remove(true)),
                            );
                        }
                        _ => {}
                    }
                }
            }

            if let Some(inline_query) = update.inline_query {
                let lenny_txt = format!("{} {}", inline_query.query, "( ͡° ͜ʖ ͡°)");
                let shrug_txt = format!("{} {}", inline_query.query, "¯\\_(ツ)_/¯");
                let lenny = types::InputMessageContent::new_text(&lenny_txt);
                let shrug = types::InputMessageContent::new_text(&shrug_txt);
                let results = &[
                    types::InlineQueryResult::new_article("article", "lenny", &lenny_txt, &lenny),
                    types::InlineQueryResult::new_article("article", "shrug", &shrug_txt, &shrug)
                ];

                bot.answer_inline_query(&args::AnswerInlineQuery::new(&inline_query.id, results));
            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot.get_updates(&update_args);
    // Alright, so if you ever decide to have a function that terminates
    // your bot, make sure you have a check at the beginning of the loop
    // that makes sure you aren't processing old messages. You could also
    // just make a getUpdates at the end of the execution that just uses
    // the latest offset. This prevents you from having to reread any
    // updates that were in the update array you got that had the terminate
    // command. Because telegram only will stop sending you the update
    // after you have used an offset greater than its. So if you never make
    // another getUpdates, you will boot up, and terminate, forever. :)
}
