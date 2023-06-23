use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "commands")]
pub struct Args {
    #[argh(subcommand, description = "the main command to run")]
    pub command: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Init(InitCommand),
    Yesterday(YesterdayCommand),
    Today(TodayCommand),
    Random(RandomCommand),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "init", description = "initialize the diary")]
pub struct InitCommand {
    #[argh(
        option,
        default = "PathBuf::new()",
        description = "the root path of the diary"
    )]
    pub diary_path: PathBuf,

    #[argh(
        option,
        default = "\"vim\".into()",
        description = "the editor used to open entries"
    )]
    pub editor: String,
}

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "yesterday",
    description = "start or open yesterday's entry"
)]
pub struct YesterdayCommand {}

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "today",
    description = "start or open today's entry"
)]
pub struct TodayCommand {}

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "random",
    description = "open a random existing entry"
)]
pub struct RandomCommand {}
