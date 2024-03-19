use clipboard::{ClipboardContext, ClipboardProvider};
use neopass::custom_select::{Select, SelectOutput};
use neopass::theme::custom_colorful_theme::ColorfulTheme;
use neopass::utils::{add_a_new_entry, display_end_of_table, modify_entry, write_entries_in_file};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};
use tabled::Table;

use neopass::entry::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    // Load existing passwords from file
    let mut entries = Vec::new();
    let file_path = "passwords.txt";

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        let lines = reader.lines().enumerate();

        for (_index, bufread) in lines {
            let line = bufread.unwrap_or_default();
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                entries.push(Entry {
                    application: parts[0].trim().to_string(),
                    username: parts[1].trim().to_string(),
                    password: parts[2].trim().to_string(),
                });
            }
        }
    }

    let mut copied_item = None;

    loop {
        // Clean and get cursor back on top.
        print!("{}[2J", 27 as char);
        print!("\x1b[1;1H");

        // Instructions.
        println!("Use ↑ and ↓ arrows to navigate between entries.");
        println!("Press 'a' to add an new entry.");
        println!("Press 'd' or Del to delete an entry.");
        println!("Press Enter or Space to copy the password in your clipboard.");
        println!("Use 'q' or Ctrl + C to quit.\n");

        if entries.is_empty() {
            println!("No credentials yet. Add one:\n");

            entries.push(add_a_new_entry());

            write_entries_in_file(file_path, &entries);
            continue;
        }

        if copied_item.is_some() {
            println!("✅ Copied password to clipboard.\n");
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
            .interact_opt()
            .unwrap()
        {
            match selection {
                SelectOutput::Copy(index) => {
                    let entry = entries.iter().nth(index as usize).ok_or("Invalid index.")?;

                    // Copy password to clipboard
                    let mut cp: ClipboardContext = ClipboardProvider::new().unwrap();
                    cp.set_contents(entry.password.clone())?;

                    copied_item = Some(index);
                }

                SelectOutput::Add => {
                    display_end_of_table(rows);

                    entries.push(add_a_new_entry());

                    write_entries_in_file(file_path, &entries);

                    copied_item = None;
                }

                SelectOutput::Delete(index) => {
                    let _removed_instance = entries.remove(index);

                    write_entries_in_file(file_path, &entries);

                    copied_item = None;
                }

                SelectOutput::Modify(index) => {
                    display_end_of_table(rows);

                    let modified_entry = modify_entry(&entries[index]);
                    entries[index] = modified_entry;

                    write_entries_in_file(file_path, &entries);

                    copied_item = None;
                }
            }
        } else {
            display_end_of_table(rows);
            exit(0);
        }
    }
}
