use super::parse_time;

use serenity::{
    model::{
        channel::{Message, Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, mut reaction: Reaction) {
        let reaction_msg = reaction.message(&ctx.http).unwrap();
        match &reaction.emoji {
            ReactionType::Unicode(uni) => match uni.as_ref() {
                "👀" => {
                    use std::thread;

                    let message_content = &reaction_msg.content;
                    let msg_args: Vec<&str> = message_content.split_whitespace().collect();
                    let mut msg_url = String::from("Url not found");
                    println!(
                        "Msg author {} reaction user {}",
                        reaction_msg.author.id, reaction.user_id
                    );

                    if msg_args.len() > 0 && msg_args[0] == "!remindme" {
                        let (_command, date_args) = msg_args.split_at(1);
                        use chrono::Utc;
                        let time_since_message = Utc::now()
                            .signed_duration_since(reaction_msg.timestamp)
                            .num_seconds();
                        let (reply_msg, time_to_wait_in_seconds, used_args) =
                            parse_time::parse_for_wait_time(
                                time_since_message as i32,
                                Vec::from(date_args),
                            );
                        if reaction_msg.author.id == reaction.user_id
                            || reaction.user(&ctx).unwrap().bot
                        {
                            println!("Bots and original user cannot be reminded with reaction.");
                        } else if time_to_wait_in_seconds <= 0 {
                            let dm_confirm =
                                reaction.user(&ctx).unwrap().direct_message(&ctx, |m| {
                                    m.content(format!("Reminder already passed."))
                                });
                        } else {
                            if reaction_msg.is_private() {
                                msg_url = format!(
                                    "http://discordapp.com/channels/@me/{}/{}",
                                    reaction_msg.channel_id, reaction_msg.id
                                );
                            } else {
                                msg_url = format!(
                                    "http://discordapp.com/channels/{}/{}/{}",
                                    reaction_msg.guild_id.unwrap(),
                                    reaction_msg.channel_id,
                                    reaction_msg.id
                                );
                            }
                            // TODO Add rest of the arguments to the message
                            let remind_msg = format!("Reminder for link: {}", &msg_url);
                            println!("Requested reminder through :eyes: emoji.");
                            let dm_confirm = reaction.user(&ctx).unwrap().direct_message(&ctx, |m| {
                                m.content(format!(
                                    "Reminder will be DMed in {} from original message date. Others can react with 👀 to also be reminded.",
                                    &reply_msg
                                ))
                            });
                            thread::sleep(std::time::Duration::new(
                                time_to_wait_in_seconds as u64,
                                0,
                            ));
                            let dm = &reaction
                                .user(&ctx)
                                .unwrap()
                                .direct_message(&ctx.http, |m| m.content(remind_msg));
                            match dm {
                                Ok(_) => {
                                    let _ = reaction_msg.react(&ctx, '✅');
                                }
                                Err(why) => {
                                    println!("Err sending DM: {:?}", why);
                                }
                            };
                        }
                    }
                    ()
                }
                _ => (),
            },
            _ => (),
        };
    }
    fn message(&self, ctx: Context, _new_message: Message) {
        if _new_message.content == "???" {
            use std::thread;
            thread::sleep(std::time::Duration::new(1, 0));

            let remind_msg = format!(
                "<@{}> wants to be reminded of something.",
                &_new_message.author.id
            );
            if let Err(err) = _new_message.channel_id.say(&ctx.http, remind_msg) {
                println!("Error giving message: {:?}", err);
            }
        }
    }
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);

        use serenity::model::gateway::Activity;

        ctx.set_activity(Activity::playing(&String::from(
            "Oh dear! I shall be too late!",
        )));
    }
}
