use std::{
    error::Error,
    path::{Path, PathBuf}, fs, io
};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Days, Local, NaiveDate,
};

use ansi_term::Color::Blue;

mod args;
use args::{Args, Command};

const ENTRY_FORMAT: &str = "%m-%d-%Y.md";

pub fn get_entry_filename(date: &NaiveDate) -> DelayedFormat<StrftimeItems<'_>> {
    date.format(ENTRY_FORMAT)
}

pub fn entry_from_filename(filename: &Path) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(filename.to_str()?, ENTRY_FORMAT).ok()
}

pub fn initialize_diary(diary_path: impl AsRef<Path>) -> io::Result<()> {
    println!("{} new diary ({})", Blue.bold().paint("Initializing"), diary_path.as_ref().to_str().unwrap());
    fs::create_dir_all(diary_path)?;

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = argh::from_env::<Args>();
    let diary_path = [
        home::home_dir().expect("find home directory").as_path(),
        Path::new("diary/entries"),
    ]
    .iter()
    .collect::<PathBuf>();

    if !diary_path.try_exists()? {
        initialize_diary(&diary_path)?;
    }

    let today = Local::now().date_naive();
    match args.command {
        Command::Yesterday(_) => {
            let yesterday = today - Days::new(1);
            println!("{}", get_entry_filename(&yesterday));
        }

        Command::Today(_) => {}

        Command::Random(_) => {}
    }

    Ok(())
}
