use clipboard::{ClipboardContext, ClipboardProvider};
use cocoon::Cocoon;
use console::Term;
use std::io::{Error, ErrorKind};
use std::{error::Error as ErrorTrait, fs::File};
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Disable, Style};
use tabled::Table;

use dialoguer::{theme::ColorfulTheme, Password};

use crate::config::FILE_PATH;
use crate::entry::{add_a_new_entry, Entry};
use crate::languages::get_translation;

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
    let mut table = Table::new([
        ["  ↓     ", &get_translation("down_arrow")],
        ["  ↑     ", &get_translation("up_arrow")],
        ["  a     ", &get_translation("add_entry")],
        ["  d     ", &get_translation("delete_entry")],
        ["  e     ", &get_translation("edit_entry")],
        ["  Space ", &get_translation("copy_password")],
        ["  l     ", &get_translation("change_language")],
    ]);
    table.with(Style::blank()).with(Disable::row(Rows::first()));
    println!("\n  {}\n{}\n", get_translation("commands"), table);
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
            get_translation("enter_password")
        } else {
            get_translation("enter_new_password")
        };

        *password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("  {}", msg))
            .interact()
            .unwrap();

        println!("\n  {}", get_translation("checking_password"));

        match decrypt_file(password) {
            Ok(found_entries) => {
                Term::stdout().clear_last_lines(6)?;
                password_is_correct = true;
                *entries = found_entries;
            }
            Err(_) => {
                Term::stderr().clear_last_lines(6)?;
                println!("\n  {}\n", get_translation("invalid_password"));
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
    println!("\n  {}", get_translation("no_password"));

    add_a_new_entry(entries);
    write_entries_in_file(entries, password)?;

    Ok(())
}

pub fn build_rows(entries: &[Entry], copied_item: &Option<usize>) -> (Vec<String>, String, String) {
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
    let mut rows: Vec<String> = table_as_string
        .split('\n')
        .map(|e| e.into())
        .collect::<Vec<String>>();

    let header = format!(
        "  {}\n  {}\n  {}",
        rows.remove(0),
        rows.remove(0),
        rows.remove(0)
    );

    let footer = format!(
        "  {}\n\n  {}",
        rows.remove(rows.len() - 1),
        if copied_item.is_some() {
            get_translation("password_copied")
        } else {
            "".to_string()
        }
    );

    (rows, header, footer)
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
