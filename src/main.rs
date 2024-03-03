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
