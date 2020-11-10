mod announce;
#[allow(unused_parens)]

extern crate log;
use log::*;
mod cmd_remindme;
mod events;
mod parse_time;
mod storage;
use events::Handler;
use serenity::{
    client::Client,
    framework::standard::Args,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::Context,
};
use std::fs::File;
use std::io::prelude::*;
use syslog::{Facility};
use log_panics;
#[group]
#[commands(help, ping, remindme)]
struct General;

fn main() {
    let init_logger = syslog::init(Facility::LOG_USER, log::LevelFilter::Info, None);

    match init_logger {
        Ok(_what) => {}
        Err(err) => error!("Error initializing logger. {:?}", err),
    }
    
    log_panics::init();

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
        error!("Error: {:?}", msg);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    match msg.reply(ctx, "Pong") {
        _ => {}
    };

    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    match msg
        .channel_id
        .say(&ctx.http, "Available commands: \n * remindme ")
    {
        _ => {}
    };
    Ok(())
}

#[command]
fn remindme(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    cmd_remindme::remindme(ctx, msg, args)
}
