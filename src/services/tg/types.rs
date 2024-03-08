use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands list:")]
pub enum Command {
    #[command(description = "Display help.")]
    Help,
    #[command(description = "Get asic statistic.")]
    Stat,
    #[command(description = "Start dialog.")]
    Start,
    #[command(description = "Get invoice info.")]
    Invoice
}