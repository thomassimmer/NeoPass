use std::{fs::File, io::Write};

use dialoguer::{theme::ColorfulTheme, Input, Password};
use rand::{thread_rng, Rng};

use crate::entry::Entry;

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
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!*#_-|&@~$";
    let chars: Vec<char> = (0..length)
        .map(|_| {
            symbols
                .chars()
                .nth(rng.gen_range(0..symbols.len()))
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
        password = generate_password(16);
    }

    Entry {
        application,
        username,
        password,
    }
}
