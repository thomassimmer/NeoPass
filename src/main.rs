use clipboard::{ClipboardContext, ClipboardProvider};
use neopass::config::{
    CHECKING_PASSWORD, ENTER_NEW_PASSWORD, ENTER_PASSWORD, FILE_PATH, INSTRUCTIONS,
    INVALID_PASSWORD, NO_PASSWORD, PASSWORD_COPIED,
};
use neopass::custom_select::{Select, SelectOutput};
use neopass::theme::custom_colorful_theme::ColorfulTheme;
use neopass::utils::{
    add_a_new_entry, clear_screen, decrypt_file, display_end_of_table, modify_entry,
    write_entries_in_file,
};
use std::error::Error;
use std::fs::File;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};
use tabled::Table;

use neopass::entry::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    let mut password_is_correct = false;
    let mut password = String::new();
    let mut entries = Vec::new();

    clear_screen();

    while !password_is_correct {
        // Ask the user for a password
        if File::open(FILE_PATH).is_ok() {
            println!("{}", ENTER_PASSWORD);
        } else {
            println!("{}", ENTER_NEW_PASSWORD);
        }

        std::io::stdin().read_line(&mut password)?;

        println!("{}", CHECKING_PASSWORD);

        clear_screen();

        entries = match decrypt_file(&password) {
            Ok(entries) => {
                password_is_correct = true;
                entries
            }
            Err(_) => {
                println!("{}", INVALID_PASSWORD);
                vec![]
            }
        };
    }

    let mut copied_item = None;

    loop {
        clear_screen();

        println!("{}", INSTRUCTIONS);

        if entries.is_empty() {
            println!("{}", NO_PASSWORD);

            entries.push(add_a_new_entry());

            write_entries_in_file(&entries, &password)?;
            continue;
        }

        if copied_item.is_some() {
            println!("{}", PASSWORD_COPIED);
        }

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
        let mut rows: Vec<&str> = table_as_string.split('\n').collect::<Vec<&str>>();

        // TODO: Find a way to print this last line.
        let last_line = rows.remove(rows.len() - 1);

        // Display table header.
        println!("  {}", rows.remove(0));
        println!("  {}", rows.remove(0));
        println!("  {}", rows.remove(0));

        // Display entries.
        let mut theme = ColorfulTheme::default();
        theme.last_line = last_line.to_string();

        if let Some(selection) = Select::with_theme(theme)
            .default(copied_item.unwrap_or_default())
            .items(&rows[..rows.len()])
            .interact_opt()?
        {
            match selection {
                // User selected one item.
                SelectOutput::Copy(index) => {
                    let entry = entries.iter().nth(index as usize).ok_or("Invalid index.")?;

                    // Copy password to clipboard
                    let mut cp: ClipboardContext = ClipboardProvider::new()?;
                    cp.set_contents(entry.password.clone())?;

                    copied_item = Some(index);
                }

                // User wants to add a new item.
                SelectOutput::Add => {
                    display_end_of_table(rows);

                    entries.push(add_a_new_entry());

                    write_entries_in_file(&entries, &password)?;

                    copied_item = None;
                }

                // User wants to delete an item.
                SelectOutput::Delete(index) => {
                    let _removed_instance = entries.remove(index);

                    write_entries_in_file(&entries, &password)?;

                    copied_item = None;
                }

                // User wants to modify one item.
                SelectOutput::Modify(index) => {
                    display_end_of_table(rows);

                    let modified_entry = modify_entry(&entries[index]);
                    entries[index] = modified_entry;

                    write_entries_in_file(&entries, &password)?;

                    copied_item = None;
                }
            }
        } else {
            display_end_of_table(rows);
            return Ok(());
        }
    }
}
