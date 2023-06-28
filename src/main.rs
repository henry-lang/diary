#![feature(iter_next_chunk)]

use std::{
    error::Error,
    fs::{self, File},
    io::{Write, self},
    process::Command as ProcessCommand,
    path::{Path, PathBuf},
};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Days, Local, NaiveDate, Weekday, Datelike,
};

use serde::{Deserialize, Serialize};
use ansi_term::Color::White;

mod args;
mod frontmatter;
use args::{Args, Command, InitCommand};

const ENTRY_FORMAT: &str = "%m-%d-%Y.md";

pub fn get_entry_filename(date: &NaiveDate) -> DelayedFormat<StrftimeItems<'_>> {
    date.format(ENTRY_FORMAT)
}

fn entry_from_filename(filename: &Path) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(filename.to_str()?, ENTRY_FORMAT).ok()
}

fn edit_entry(config: &Config, date: &NaiveDate) -> io::Result<()> {
    let path = config.diary_path.join(Path::new(&get_entry_filename(date).to_string()));
    if !path.is_file() {
        
    }
    ProcessCommand::new(&config.editor).arg(path).spawn()?.wait()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    diary_path: PathBuf,
    editor: String,
}

impl Config {
    pub fn new(diary_path: PathBuf, editor: String) -> Self {
        Self { diary_path, editor }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = argh::from_env::<Args>();
    let config_path = home::home_dir().expect("find home directory").join(Path::new(".config/diary/config.toml"));

    if let Command::Init(InitCommand { diary_path, editor }) = args.command {
        if config_path.is_file() {
            println!("Config already exists at {}. Delete it and rerun the command to generate a new one.", config_path.to_str().expect("path is valid utf-8"));
            return Ok(());
        }

        let config = Config::new(diary_path, editor);

        fs::create_dir_all(config_path.parent().unwrap())?;
        File::create(&config_path)?.write_all(toml::to_string_pretty(&config)?.as_bytes())?;

        println!(
            "Config created at {}",
            config_path.to_str().expect("path is valid utf-8")
        );
        return Ok(());
    }

    if !config_path.is_file() {
        println!("No config file found. Run {} to generate one.", White.bold().paint("diary init"));
        return Ok(());
    }

    let config =
        toml::from_str::<Config>(&fs::read_to_string(&config_path).expect("read config file"))
            .expect("parse config file");

    fs::create_dir_all(&config.diary_path)?;

    let today = Local::now().date_naive();
    match args.command {
        Command::Init(_) => {
            println!(
                "{:?}",
                config
                    .diary_path
                    .with_file_name(get_entry_filename(&today).to_string())
            );
        }

        Command::Yesterday(_) => edit_entry(&config, &(today - Days::new(1)))?,
        Command::Today(_) => edit_entry(&config, &today)?,
        Command::Random(_) => {
            
        }
        Command::Stats(_) => {
            let mut entry_count = 0;
            let mut word_count = 0;

            let diary_entries = fs::read_dir(&config.diary_path)?;
            for entry in diary_entries {
                if let Ok(entry) = entry {
                    entry_count += 1;
                    let content = fs::read_to_string(entry.path())?;
                    let content = frontmatter::skip(&content).unwrap();
                    word_count += content.split_whitespace().count();
                }
            }


            let total_entries = entry_count;
            let total_words = word_count;
            let avg_words_per_entry = word_count as f64 / entry_count as f64;


            println!("Total Entries: {}", total_entries);
            println!("Total Words: {}", total_words);
            println!("Average Words per Entry: {:.2}", avg_words_per_entry);
        }
    }

    Ok(())
}
