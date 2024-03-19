use std::{fs::File, io::Write};

use dialoguer::{theme::ColorfulTheme, Input, Password};
use rand::{thread_rng, Rng};

use crate::{
    config::{PASSWORD_LENGTH, SYMBOLS_TO_USE_IN_PASSWORDS},
    entry::Entry,
};

pub fn write_entries_in_file(file_path: &str, entries: &Vec<Entry>) {
    // This overwrites the content of the file with the current entries.

    let mut file = File::create(file_path).unwrap();
    let mut content = String::new();
    for entry in entries {
        content.push_str(&format!(
            "{},{},{}\n",
            entry.application, entry.username, entry.password
        ));
    }
    let _ = file.write(content.as_bytes());
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
