use std::{collections::HashMap, sync::Mutex};

use crate::languages::{Language, Translations};

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
