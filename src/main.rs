mod events;
mod parse_time;

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

    let (reply_msg, time_to_wait_in_seconds, used_args) =
        parse_time::parse_for_wait_time(0, args.raw().collect::<Vec<&str>>());

    for _ in 0..used_args {
        // Consume the arguments that were processed above
        args.advance();
    }

    if time_to_wait_in_seconds > 0 {
        msg.channel_id.say(
            &ctx.http,
            format!(
                "Reminder will be DMed in {}. React with 👀 to also be reminded.",
                &reply_msg
            ),
        )?;
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
        //// Alternative way to mention player instead of `msg.reply`
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
                println!("Err sending DM: {:?}", why);

                // let _ = msg.reply(&ctx, "There was an error DMing you help.");
            }
        };
    } else {
        msg.channel_id.say(&ctx.http, format!("{}", &reply_msg))?;
    }

    Ok(())
}
