use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands list:")]
pub enum Command {
    #[command(description = "display help.")]
    Help,
    #[command(description = "get asic statistic.")]
    Stat,
    #[command(description = "start dialog.")]
    Start
}