use rand::{thread_rng, Rng};
use tabled::Tabled;

use dialoguer::{theme::ColorfulTheme, Input, Password};

use crate::config::{PASSWORD_LENGTH, SYMBOLS_TO_USE_IN_PASSWORDS};

#[derive(Debug, Tabled)]
pub struct Entry {
    #[tabled(rename = "Application / Website")]
    pub application: String,
    #[tabled(rename = "Username / Email")]
    pub username: String,
    #[tabled(rename = "Password")]
    pub password: String,
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

pub fn add_a_new_entry(entries: &mut Vec<Entry>) {
    println!();

    let application: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Application / Website:")
        .interact_text()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Username / Email:")
        .interact_text()
        .unwrap();

    let mut password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("  Password (leave empty for random):")
        .allow_empty_password(true)
        .interact()
        .unwrap();

    if password.is_empty() {
        password = generate_password(PASSWORD_LENGTH);
    }

    entries.push(Entry {
        application,
        username,
        password,
    });
}

pub fn modify_entry(entries: &mut [Entry], index: usize) {
    println!();

    let entry = &entries[index];

    let application: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Application / Website:")
        .with_initial_text(entry.application.clone())
        .interact_text()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Username / Email:")
        .with_initial_text(entry.username.clone())
        .interact_text()
        .unwrap();

    let mut password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("  Password (leave empty for random):")
        .allow_empty_password(true)
        .interact()
        .unwrap();

    if password.is_empty() {
        password = generate_password(PASSWORD_LENGTH);
    }

    entries[index] = Entry {
        application,
        username,
        password,
    };
}
