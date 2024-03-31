#![warn(clippy::all, clippy::pedantic)]

pub mod services;
pub mod config;

use dotenv::dotenv;
use teloxide::{prelude::Bot, repls::CommandReplExt};

use crate::services::tg::types::Command;
use crate::services::tg::handlers::handlers;

#[tokio::main]
async fn main() {
    // expose .env
    dotenv().ok();

    // init logger
    pretty_env_logger::init();

    log::info!("Try to start app");
    let bot = Bot::from_env();

    log::info!("Expose command");
    Command::repl(bot, handlers).await;
}


// #[tokio::main]
// async fn main() -> Result<(), ()> {
//     use std::{io::{Read, Write}, net::TcpStream};
//     use serde_json::json;

//     let url = format!("{}:{}", "192.168.0.109", 4028);
//     let mut stream = TcpStream::connect(&url).expect("Error in crate tcp connect");

//     let payload = json!({"cmd": "status"});
//     let cmd = payload.to_string();
//     let mut buffer = String::new();
    
//     dbg!(&cmd);

//     stream.write(cmd.as_bytes()).unwrap();
//     stream.read_to_string(&mut buffer).unwrap();

//     dbg!(&buffer);

//     Ok(())
// }
