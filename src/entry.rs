use std::borrow::Cow;

use rand::{thread_rng, Rng};
use tabled::Tabled;

use dialoguer::{theme::ColorfulTheme, Input, Password};

use crate::{
    config::{PASSWORD_LENGTH, SYMBOLS_TO_USE_IN_PASSWORDS},
    languages::get_translation,
};

#[derive(Debug)]
pub struct Entry {
    pub application: String,
    pub username: String,
    pub password: String,
}

impl Tabled for Entry {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::Borrowed(&self.application),
            Cow::Borrowed(&self.username),
            Cow::Borrowed(&self.password),
        ]
    }
    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::Owned(get_translation("application_website")),
            Cow::Owned(get_translation("username_email")),
            Cow::Owned(get_translation("password")),
        ]
    }
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
    println!("\n  {}\n", get_translation("add_a_new_entry"));

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
    println!("\n  {}\n", get_translation("edit_an_entry"));

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
