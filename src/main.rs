mod events;

use events::Handler;
use serenity::framework::standard::Args;

use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use std::fs::File;
use std::io::prelude::*;

#[group]
#[commands(help, ping, remindme)]
struct General;

fn main() {
    let mut file = File::open(".token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token)
        .expect("Token could not be read");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP),
    );
    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong")?;

    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Available commands: \n * remindme ")?;
    Ok(())
}

#[command]
fn remindme(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use std::thread;

    let first_arg = args.single::<String>().unwrap();
    let mut reply_msg: String = String::from("Failed to parse date.");

    let time_to_wait_in_seconds = match first_arg.parse::<i32>() {
        Ok(n) => {
            let second_arg = args.single::<String>().unwrap();
            match second_arg.as_ref() {
                "s" | "second" | "seconds" | "sec" | "secs" => {
                    reply_msg = format!("{} seconds", n);
                    n
                }
                "m" | "minute" | "minutes" | "min" | "mins" => {
                    reply_msg = format!("{} minutes", n);
                    n * 60
                }
                "h" | "hour" | "hours" | "hr" | "hrs" => {
                    reply_msg = format!("{} hours", n);
                    n * 60 * 60
                }
                "d" | "day" | "days" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24
                }
                "w" | "week" | "weeks" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7
                }
                "month" | "months" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7 * 4
                }
                "y" | "year" | "years" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7 * 4 * 12
                }
                _ => {
                    reply_msg = format!("{} minutes", n);
                    n * 60
                }
            }
        }
        Err(e) => 0,
    };

    msg.channel_id.say(
        &ctx.http,
        format!(
            "Reminder will be DMed in {}. React with 👀 to also be reminded.",
            &reply_msg
        ),
    )?;

    if time_to_wait_in_seconds > 0 {
        let _ = msg.react(&ctx, '👀');
        let mut msg_url = String::from("Url not found");
        if msg.is_private() {
            msg_url = format!(
                "http://discordapp.com/channels/@me/{}/{}",
                msg.channel_id, msg.id
            );
        } else {
            msg_url = format!(
                "http://discordapp.com/channels/{}/{}/{}",
                msg.guild_id.unwrap(),
                msg.channel_id,
                msg.id
            );
        }
        let remind_msg = format!("Reminder: \"{}\" \nLink: {}", args.rest(), &msg_url);
        // let remind_msg = format!(
        //     "Reminder <@{}>: {} \nLink: {}",
        //     &msg.author.id,
        //     args.rest(),
        //     &msg_url
        // );

        thread::sleep(std::time::Duration::new(time_to_wait_in_seconds as u64, 0));

        let dm = msg.author.direct_message(&ctx, |m| m.content(remind_msg));

        match dm {
            Ok(_) => {
                let _ = msg.react(&ctx, '✅');
                // let _ = msg.react(&ctx, '👌');
            }
            Err(why) => {
                // println!("Err sending help: {:?}", why);

                // let _ = msg.reply(&ctx, "There was an error DMing you help.");
            }
        };
    }

    Ok(())
}
