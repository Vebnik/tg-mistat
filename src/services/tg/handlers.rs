use teloxide::{payloads::SendMessageSetters, prelude::{Bot, Message, ResponseResult}, requests::Requester, utils::command::BotCommands};
use teloxide::types::ParseMode;

use super::types::Command;
use crate::config::Config;
use crate::services::emcd::types::Income;
use crate::services::whatsminer::types::{Client, Summary};

async fn get_workers() -> Vec<(Summary, String)> {
    let cfg = Config::get().await;

    let connection_data: Vec<Vec<&str>> = cfg.asics.iter()
        .map(|el| el.split(":").collect()).collect();

    let mut clients: Vec<Client> = connection_data.iter()
        .map(|data| {
            log::info!("{}:{} - {}", data[0].to_string(), data[1].to_string(), data[2].to_string());
            Client::new(data[0].to_string(), data[1].to_string(), data[2].to_string(), false)
        }).collect();

    clients.iter_mut().map(|client| (client.summary(), client.name.clone())).collect()
}   

async fn help_cmd(bot: Bot, msg: Message) {
    let text = Command::descriptions().to_string();

    match bot.send_message(msg.chat.id, text).await {
        Ok(_) => log::info!("Success cmd -> help"),
        Err(_) => log::error!("Error"),
    }
}

async fn stat_cmd(bot: Bot, msg: Message) {
    let summarys = get_workers().await;
    let mut text = String::new();    

    for (i, worker) in summarys.iter().enumerate() {
            let row = format!(
                "💻 WORKER: `{}` \\({}\\)\n```\nВентилятор (вход): {} rpm\nВентилятор (выход): {} rpm\nТемпература: {} °C\nМощность: {} W\nСкорость: {:.2} TH\n```\n", 
                worker.1,
                i,
                worker.0.fan_in,
                worker.0.fan_out,
                worker.0.temp,
                worker.0.power,
                (worker.0.mhs_av / 1000000.0),
            );
            text.push_str(&row)
    }

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

async fn invoice_cmd(bot:Bot, msg: Message) {
    let incomes = Income::get().await;
        
    let text = format!("💰 Invoice\n```\nВсего: {} BTC\nХешрейт: {} TH\nПоследняя: {}\n```",
        incomes.iter().fold(0.0, |acc, el| acc + el.income),
        (incomes.iter().fold(0, |acc, el| acc + el.total_hashrate) as f64) / (10 as f64).powf(13.0),
        incomes.first().take().unwrap().gmt_time,
    );

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2).await
        .expect("Error in invoice_cmd");
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
        Command::Invoice => invoice_cmd(bot, msg).await,
    };

    Ok(())
}