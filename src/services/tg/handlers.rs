use teloxide::{payloads::SendMessageSetters, prelude::{Bot, Message, ResponseResult}, requests::Requester, utils::command::BotCommands};
use teloxide::types::ParseMode;

use super::types::Command;
use crate::services::minerstat::types::Worker;
use crate::services::emcd::types::Income;


async fn help_cmd(bot: Bot, msg: Message) {
    let text = String::from("Для получения статистики введи команду - /stat");

    match bot.send_message(msg.chat.id, text).await {
        Ok(_) => log::info!("Success cmd -> help"),
        Err(_) => log::error!("Error"),
    }
}

async fn stat_cmd(bot: Bot, msg: Message) {
    let workers = Worker::get().await;
    let incomes = Income::get().await;

    let mut text = String::new();    
    let invoice_text = format!("💰 Invoice\n```\nВсего: {} BTC\nХешрейт: {} TH\nПоследняя: {}\n```",
        incomes.iter().fold(0.0, |acc, el| acc + el.income),
        incomes.iter().fold(0, |acc, el| acc + el.total_hashrate),
        incomes.first().take().unwrap().gmt_time,
    );

    for worker in workers {
            let row = format!(
                "💻 {} \n```\nВентилятор: {} rpm\nТемпература: {} °C\n```\n", 
                worker.info.name,
                worker.hardware.iter().fold(0, |acc, el| acc + el.fan)/2,
                worker.hardware.iter().fold(0, |acc, el| acc + el.temp)/worker.hardware.len() as i32,
            );
            text.push_str(&row)
    }

    text.push_str(&invoice_text);

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2).await
        .expect("Error in stat_cmd");
}

async fn start_cmd(bot: Bot, msg: Message) {
    let text = Command::descriptions().to_string();

    match bot.send_message(msg.chat.id, text).await {
        Ok(_) => log::info!("Success cmd -> help"),
        Err(_) => log::error!("Error"),
    }
}

pub async fn handlers(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let allowed_chat: [i64; 2] = [978068405, 1509403669];

    if !allowed_chat.contains(&msg.chat.id.0) {
        log::info!("Not allowed chat: {} ({})", msg.chat.id, msg.chat.first_name().unwrap_or("NoName"));
        return Ok(())
    };

    match cmd {
        Command::Help => help_cmd(bot, msg).await,
        Command::Stat => stat_cmd(bot, msg).await,
        Command::Start => start_cmd(bot, msg).await,
    };

    Ok(())
}