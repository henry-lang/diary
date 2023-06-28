use std::path::{Path, PathBuf};

use argh::FromArgs;

fn default_diary_path() -> PathBuf {
    [
        &home::home_dir().expect("get home directory - use --diary-path to supply your own"),
        Path::new("diary"),
    ]
    .iter()
    .collect::<PathBuf>()
}

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
    Stats(StatsCommand)
}

#[derive(FromArgs)]
#[argh(subcommand, name = "init", description = "initialize the diary")]
pub struct InitCommand {
    #[argh(
        option,
        default = "default_diary_path()",
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

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "stats",
    description = "show stats about the diary"
)]
pub struct StatsCommand {}
