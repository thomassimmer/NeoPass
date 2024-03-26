use clipboard::{ClipboardContext, ClipboardProvider};
use cocoon::Cocoon;
use console::Term;
use std::io::{Error, ErrorKind};
use std::{error::Error as ErrorTrait, fs::File};
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};
use tabled::Table;

use dialoguer::{theme::ColorfulTheme, Password};

use crate::config::{
    CHECKING_PASSWORD, ENTER_NEW_PASSWORD, ENTER_PASSWORD, INSTRUCTIONS, INVALID_PASSWORD,
    NO_PASSWORD, PASSWORD_COPIED,
};
use crate::entry::add_a_new_entry;
use crate::{config::FILE_PATH, entry::Entry};

pub fn decrypt_file(password: &str) -> Result<Vec<Entry>, Box<dyn ErrorTrait>> {
    let mut entries = Vec::new();
    let mut cocoon = Cocoon::new(password.as_bytes());

    // Read the contents of the password file.
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
        // Create password file.
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

    // Write the encrypted contents to the output file.
    let mut file = File::create(FILE_PATH)?;
    cocoon.dump(contents.into_bytes(), &mut file).unwrap();

    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    // Clean and get cursor back on top.
    Term::stderr().clear_last_lines(30)
}

pub fn display_instructions() {
    println!("  {}", INSTRUCTIONS);
}

pub fn display_password_copied() {
    println!("  {}", PASSWORD_COPIED);
}

pub fn get_user_password(
    entries: &mut Vec<Entry>,
    password: &mut String,
) -> Result<(), Box<dyn ErrorTrait>> {
    // Ask the user for a password.
    let mut password_is_correct = false;

    println!();

    while !password_is_correct {
        let msg: String = if File::open(FILE_PATH).is_ok() {
            ENTER_PASSWORD.to_string()
        } else {
            ENTER_NEW_PASSWORD.to_string()
        };

        *password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .interact()
            .unwrap();

        println!("\n  {}", CHECKING_PASSWORD);

        match decrypt_file(password) {
            Ok(found_entries) => {
                Term::stdout().clear_last_lines(6)?;
                password_is_correct = true;
                *entries = found_entries;
            }
            Err(_) => {
                Term::stderr().clear_last_lines(6)?;
                println!("\n  {}\n", INVALID_PASSWORD);
                continue;
            }
        };
    }

    Ok(())
}

pub fn add_first_entry(
    entries: &mut Vec<Entry>,
    password: &mut str,
) -> Result<(), Box<dyn ErrorTrait>> {
    println!("\n  {}", NO_PASSWORD);

    add_a_new_entry(entries);
    write_entries_in_file(entries, password)?;

    Term::stderr().clear_last_lines(NO_PASSWORD.chars().filter(|&x| x == '\n').count() + 6)?;

    Ok(())
}

pub fn build_rows(entries: &[Entry]) -> Vec<String> {
    // Build table.
    let mut table = Table::new(entries.iter().map(|e| Entry {
        application: e.application.clone(),
        username: e.username.clone(),
        password: "********".to_string(),
    }));
    let table = table
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::left());

    let table_as_string = table.to_string();

    // Get table rows so we can make them selectable.
    let rows: Vec<String> = table_as_string
        .split('\n')
        .map(|e| e.into())
        .collect::<Vec<String>>();

    rows
}

pub fn set_password_in_clipboard(
    entries: &[Entry],
    index: usize,
    copied_item: &mut Option<usize>,
) -> Result<(), Box<dyn ErrorTrait>> {
    let entry = &entries[index];

    // Copy password to clipboard.
    let mut cp: ClipboardContext = ClipboardProvider::new()?;
    cp.set_contents(entry.password.clone())?;

    *copied_item = Some(index);

    Ok(())
}
