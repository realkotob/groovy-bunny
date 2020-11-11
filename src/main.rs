mod announce;
#[allow(unused_parens)]
extern crate log;
use log::*;
mod cmd_remindme;
mod events;
mod globalstate;
mod parse_time;
mod storage;
use events::Handler;
use futures::join;
use log_panics;

use serenity::{
    async_trait,
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
use syslog::Facility;
use tokio;
#[group]
#[commands(help, ping, remindme)]
struct General;

#[tokio::main]
async fn main() {
    let init_logger = syslog::init(Facility::LOG_USER, log::LevelFilter::Info, None);

    match init_logger {
        Ok(_what) => {}
        Err(err) => error!("Error initializing logger. {:?}", err),
    }

    log_panics::init();

    let TOKEN = globalstate::get_token();

    let new_framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .framework(new_framework)
        .await
        .expect("Error creating client");

    if let Err(msg) = client.start().await {
        error!("Client Error: {:?}", msg);
    }
    
    join!(
        announce::schedule_announcements(),
        storage::load_reminders()
    );
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.reply(&ctx.http, "Pong").await {
        _ => {}
    };

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    match msg
        .channel_id
        .say(&ctx.http, "Available commands: \n * remindme ")
        .await
    {
        _ => {}
    };
    Ok(())
}

#[command]
async fn remindme(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    cmd_remindme::remindme(ctx, msg, args).await
}
