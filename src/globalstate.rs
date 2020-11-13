use std::fs::File;
use std::io::prelude::*;

use log::*;
use serenity;
use std;
use std::time::Duration;

use super::events;

use lazy_static::lazy_static;

use log_panics;
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

use syslog::Facility;

// pub fn get_token_from_file() -> String {
//     let mut file = File::open(".token").unwrap();
//     let mut token = String::new();
//     file.read_to_string(&mut token)
//         .expect("Token could not be read");

//     token
// }

lazy_static! {
    static ref TOKEN: String = {
        let mut file = File::open(".token").unwrap();
        let mut token = String::new();
        file.read_to_string(&mut token)
            .expect("Token could not be read");

        token
    };
}

pub fn get_token() -> String {
    TOKEN.to_string()
}

pub fn make_http() -> std::sync::Arc<serenity::http::client::Http> {
    // let TOKEN = get_token();

    let mut client =
        Client::new(&TOKEN.as_str(), events::HandlerEmpty).expect("Error creating client");

    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")));

    if let Err(why) = client.start() {
        error!("Error: {:?}", why);
    };

    client.cache_and_http.http.clone()
}
