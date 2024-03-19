use ::rand::{thread_rng, Rng};
use cocoon::Cocoon;
use std::io::{Error, ErrorKind};
use std::{error::Error as ErrorTrait, fs::File};

use dialoguer::{theme::ColorfulTheme, Input, Password};

use crate::config::CLEAR_SCREEN;
use crate::{
    config::{FILE_PATH, PASSWORD_LENGTH, SYMBOLS_TO_USE_IN_PASSWORDS},
    entry::Entry,
};

pub fn decrypt_file(password: &str) -> Result<Vec<Entry>, Box<dyn ErrorTrait>> {
    let mut entries = Vec::new();
    let mut cocoon = Cocoon::new(password.as_bytes());

    // Read the contents of the input file
    if let Ok(mut input_file) = File::open(FILE_PATH) {
        let encoded = match cocoon.parse(&mut input_file) {
            Ok(encoded) => encoded,
            Err(_) => {
                return Err(Box::new(Error::new(ErrorKind::Other, "Invalid password")));
            }
        };

        let lines = std::str::from_utf8(&encoded)?
            .split('\n')
            .collect::<Vec<&str>>();

        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                entries.push(Entry {
                    application: parts[0].trim().to_string(),
                    username: parts[1].trim().to_string(),
                    password: parts[2].trim().to_string(),
                });
            }
        }
    } else {
        let mut file = File::create(FILE_PATH)?;
        let contents = "BEGIN".to_string();
        cocoon.dump(contents.into_bytes(), &mut file).unwrap();
    }

    Ok(entries)
}

pub fn write_entries_in_file(
    entries: &Vec<Entry>,
    password: &str,
) -> Result<(), Box<dyn ErrorTrait>> {
    let mut contents = String::new();
    for entry in entries {
        contents.push_str(&format!(
            "{},{},{}\n",
            entry.application, entry.username, entry.password
        ));
    }

    encrypt_file(contents, password)
}

pub fn encrypt_file(contents: String, password: &str) -> Result<(), Box<dyn ErrorTrait>> {
    let mut cocoon = Cocoon::new(password.as_bytes());

    // Write the encrypted contents to the output file
    let mut file = File::create(FILE_PATH)?;
    cocoon.dump(contents.into_bytes(), &mut file).unwrap();

    Ok(())
}

pub fn generate_password(length: usize) -> String {
    let mut rng = thread_rng();
    let chars: Vec<char> = (0..length)
        .map(|_| {
            SYMBOLS_TO_USE_IN_PASSWORDS
                .chars()
                .nth(rng.gen_range(0..SYMBOLS_TO_USE_IN_PASSWORDS.len()))
                .unwrap()
        })
        .collect();
    chars.into_iter().collect()
}

pub fn add_a_new_entry() -> Entry {
    let application: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Application / Website:")
        .interact_text()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username / Email:")
        .interact_text()
        .unwrap();

    let mut password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password (leave empty for random):")
        .allow_empty_password(true)
        .interact()
        .unwrap();

    if password.is_empty() {
        password = generate_password(PASSWORD_LENGTH);
    }

    Entry {
        application,
        username,
        password,
    }
}

pub fn modify_entry(entry: &Entry) -> Entry {
    let application: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Application / Website:")
        .with_initial_text(entry.application.clone())
        .interact_text()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username / Email:")
        .with_initial_text(entry.username.clone())
        .interact_text()
        .unwrap();

    let mut password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password (leave empty for random):")
        .allow_empty_password(true)
        .interact()
        .unwrap();

    if password.is_empty() {
        password = generate_password(PASSWORD_LENGTH);
    }

    Entry {
        application,
        username,
        password,
    }
}

pub fn display_end_of_table(rows: Vec<&str>) {
    // Print the rest of the table. It's cleaner.
    for row in &rows {
        println!("  {}", row);
    }
    println!();
}

pub fn clear_screen() {
    // Clean and get cursor back on top.
    print!("{}", CLEAR_SCREEN);
}
