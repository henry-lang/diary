use std::{
    error::Error,
    fs::{self, File, OpenOptions}, io::{self, Write},
    path::{Path, PathBuf},
};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Days, Local, NaiveDate,
};

use serde::{Deserialize, Serialize};

use ansi_term::Color::Blue;

mod args;
use args::{Args, Command, InitCommand};

const ENTRY_FORMAT: &str = "%m-%d-%Y.md";

pub fn get_entry_filename(date: &NaiveDate) -> DelayedFormat<StrftimeItems<'_>> {
    date.format(ENTRY_FORMAT)
}

pub fn entry_from_filename(filename: &Path) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(filename.to_str()?, ENTRY_FORMAT).ok()
}

#[derive(Serialize, Deserialize)]
struct Config {
    diary_path: PathBuf,
    editor: String,
}

impl Config {
    pub fn new(diary_path: PathBuf, editor: String) -> Self {
        Self {diary_path, editor}
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = argh::from_env::<Args>();
    let home_dir = home::home_dir().expect("find home directory");
    let home_dir = home_dir.as_path();

    let config_path = [home_dir, Path::new(".config/diary/config.toml")]
        .iter()
        .collect::<PathBuf>();

    let config_exists = fs::metadata(&config_path).is_ok_and(|m| m.is_file());

    if let Command::Init(InitCommand {diary_path, editor}) = args.command {
        if config_exists {
            println!("Config already exists at {}. Delete it and rerun the command to generate a new one.", config_path.to_str().expect("path is valid utf-8"));
            return Ok(())
        }

        let config = Config::new(diary_path, editor);

        fs::create_dir_all(&config_path.parent().unwrap())?;
        File::create(&config_path)?.write_all(toml::to_string_pretty(&config)?.as_bytes())?;

        println!("Config created at {}", config_path.to_str().expect("path is valid utf-8"));
        return Ok(())
    }
 
    let config = toml::from_str::<Config>(&fs::read_to_string(&config_path).expect("read config file")).expect("parse config file");

    let today = Local::now().date_naive();
    match args.command {
        Command::Init(_) => {
            println!("{:?}", config.diary_path.with_file_name(get_entry_filename(&today).to_string()));
        }

        Command::Yesterday(_) => {
            let yesterday = today - Days::new(1);
            println!("{}", get_entry_filename(&yesterday));
        }
        Command::Today(_) => {

        }
        Command::Random(_) => {}
    }

    Ok(())
}
