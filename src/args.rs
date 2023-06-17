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
    Yesterday(YesterdayCommand),
    Today(TodayCommand),
    Random(RandomCommand),
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
