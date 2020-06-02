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
#[commands(ping, remindme)]
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
fn remindme(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use std::thread;
    thread::sleep(std::time::Duration::new(1, 0));

    let remind_msg = format!(
        "{} wants to be reminded of something. {}",
        &msg.author.name,
        args.rest()
    );
    if let Err(err) = msg.channel_id.say(&ctx.http, remind_msg) {
        println!("Error giving message: {:?}", err);
    }

    Ok(())
}
