use lazy_static::lazy_static;

use std::fs::File;
use std::io::prelude::*;

use serenity;
use std;

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
    let new_h = serenity::http::client::Http::new_with_token(&TOKEN);
    std::sync::Arc::new(new_h)
}
