#[allow(unused_parens)]
use super::parse_time;
use super::storage;
use chrono::Utc;
use std::io::{BufRead, BufReader, Error, Read, Write};

use serenity::{
    model::{
        channel::{Message, Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;
pub struct HandlerEmpty;

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

impl EventHandler for HandlerEmpty {}

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
                            let user_id = &reaction.user(&ctx).unwrap().id.0;
                            match storage::save_reminder(
                                reaction_msg.timestamp.timestamp(),
                                time_to_wait_in_seconds,
                                *user_id,
                                remind_msg.to_string(),
                            ) {
                                Ok(_x) => {}
                                Err(why) => {
                                    println!("Error saving reminder {:?}", why);
                                }
                            };
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

        // schedule_worklog(&ctx);
        extern crate job_scheduler;
        use job_scheduler::{Job, JobScheduler};
        use std::time::Duration;
        let mut sched = JobScheduler::new();
        sched.add(Job::new("0 0 9 * * FRI".parse().unwrap(), || {
            println!("check_work_log...");
            match check_work_log(&ctx) {
                Ok(x) => println!("Checked worklog loaded."),
                Err(why) => println!("Error checking worklog {:?}", why),
            };
        }));

        // match storage::load_reminders(ctx) {
        //     Ok(x) => println!("Reminders loaded."),
        //     Err(why) => println!("Error loading reminders. {:?}", why),
        // };

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

fn schedule_worklog(ctx: &Context) {}

fn check_work_log(ctx: &Context) -> Result<(), Error> {
    use serenity::model::gateway::Activity;
    use serenity::model::id::{ChannelId, MessageId};

    let worklog_channel_id = 705067423530745957;
    // let test_channel_id = 770656917415788545;

    let channel_id = ChannelId(worklog_channel_id);

    let all_devs: Vec<u64> = vec![
        492385983833047051,
        503494040436604930,
        447503701733539845,
        305360713893937162,
        669148598193225739,
    ]; // sam 669148598193225739
    let mut did_speak = vec![];
    let mut didnt_speak = vec![];
    let _messages = channel_id.messages(&ctx.http, |retriever| retriever);
    let mut didnt_size: u64 = 0;
    let mut did_size: u64 = 0;
    match _messages {
        Ok(msgs) => {
            for dev in all_devs {
                let mut dev_spoke = false;
                let mut words_count = 0;
                for elem in &msgs {
                    if (
                        (&dev == elem.author.id.as_u64())
                            && ((Utc::now().timestamp() - elem.timestamp.timestamp()) < 432000)
                        // week 604800
                    ) {
                        let mut words = elem.content.split_whitespace();
                        let local_words_count = words.count();
                        if (local_words_count > 2) {
                            words_count += local_words_count;
                        }
                    };
                }
                if (words_count > 5) {
                    dev_spoke = true;
                }
                if dev_spoke {
                    did_speak.push(dev);
                } else {
                    didnt_speak.push(dev);
                }
            }
            didnt_size = didnt_speak.len() as u64;
            did_size = did_speak.len() as u64;
        }
        Err(why) => {
            println!("Failed to get messages. {:?}", why);
        }
    }

    let work_log_channel = ctx.http.get_channel(worklog_channel_id);
    let mut msg_didnt_worklog = " remember to post your weekly progress!".to_string();
    let mut msg_did_worklog = " posted in the past week, congrats!".to_string();
    for elem in didnt_speak {
        let str_mention = format!(" <@{}>", &elem.to_string());
        msg_didnt_worklog = format!("{}{}", &str_mention, &msg_didnt_worklog).to_owned();
    }
    for elem in did_speak {
        let str_mention = format!(" <@{}>", &elem.to_string());
        msg_did_worklog = format!("{}{}", &str_mention, &msg_did_worklog);
    }
    match work_log_channel {
        Ok(x) => {
            match x.guild() {
                Some(guild_lock) => {
                    if (didnt_size > 0) {
                        guild_lock.read().say(&ctx.http, msg_didnt_worklog);
                    }
                    if (did_size > 0) {
                        guild_lock.read().say(&ctx.http, msg_did_worklog);
                    }
                }
                None => {
                    println!("It's not a guild!");
                }
            };
        }
        Err(why) => {
            println!("Error sending message to worklog channel. {:?}", why);
        }
    };

    Ok(())
}
