use crate::languages::{Language, Translations};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    sync::Mutex,
};

#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    language_code: &'a str,
}

pub const LOCAL_CONFIG_PATH: &str = "local_config.json";

pub fn read_local_config() -> Result<(), Box<dyn Error>> {
    if let Ok(mut file) = File::open(LOCAL_CONFIG_PATH) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read the file");
        let res: Config = serde_json::from_str(&content).expect("Cannot parse local config file");

        *LANGUAGE.lock().unwrap() = res.language_code.to_string();
    } else {
        let mut file = File::create(LOCAL_CONFIG_PATH)?;
        let config = Config {
            language_code: &LANGUAGE.lock().unwrap(),
        };
        let contents = serde_json::to_string(&config)?;
        file.write(contents.as_bytes())?;
    }
    Ok(())
}

pub fn write_local_config() -> Result<(), Box<dyn Error>> {
    let config = Config {
        language_code: &LANGUAGE.lock().unwrap(),
    };
    let contents = serde_json::to_string(&config)?;
    fs::write(LOCAL_CONFIG_PATH, contents)?;

    Ok(())
}

lazy_static::lazy_static! {
    pub static ref LANGUAGE: Mutex<String> = Mutex::new("en".to_string());
    pub static ref TRANSLATIONS: Mutex<Translations> = Mutex::new(HashMap::new());
}

pub const PASSWORD_LENGTH: usize = 20;
pub const SYMBOLS_TO_USE_IN_PASSWORDS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!*#_-|&@~$";
pub const FILE_PATH: &str = "passwords.txt";
pub const INACTIVITY_DELAY: u64 = 5 * 60;
pub const LANGUAGES_AVAILABLE: [Language; 2] = [
    Language {
        label: "English",
        code: "en",
    },
    Language {
        label: "Français",
        code: "fr",
    },
];
