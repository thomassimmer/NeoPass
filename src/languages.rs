use std::collections::HashMap;
use std::error::Error;
use std::fs;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::config::{LANGUAGE, LANGUAGES_AVAILABLE, TRANSLATIONS};

type Key = String;
type Locale = String;
type Value = String;
pub type Translations = HashMap<Key, HashMap<Locale, Value>>;

pub struct Language<'a> {
    pub label: &'a str,
    pub code: &'a str,
}

pub fn read_locales() {
    let mut translations = TRANSLATIONS.lock().unwrap();

    let content = fs::read_to_string("locales.json").expect("Failed to read the file");
    let res: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&content).expect("Cannot parse locale file");
    translations.extend(res);
}

pub fn get_translation(key: &str) -> String {
    let lang = LANGUAGE.lock().unwrap();
    let translations = TRANSLATIONS.lock().unwrap();

    match translations.get(key) {
        Some(message_code) => match message_code.get(&*lang) {
            Some(translation) => translation.clone(),
            None => "No translation for this language.".to_string(),
        },
        None => "No message for this code".to_string(),
    }
}

pub fn select_language() -> Result<(), Box<dyn Error>> {
    let theme = ColorfulTheme::default();

    let current_language_index = LANGUAGES_AVAILABLE
        .iter()
        .position(|l| l.code == *LANGUAGE.lock().unwrap());

    if let Some(selection) = Select::with_theme(&theme)
        .default(current_language_index.unwrap_or_default())
        .items(LANGUAGES_AVAILABLE.iter().map(|l| l.label.clone()))
        .interact_opt()?
    {
        *LANGUAGE.lock().unwrap() = LANGUAGES_AVAILABLE[selection].code.to_string();
    } else {
        return Ok(());
    }

    Ok(())
}
