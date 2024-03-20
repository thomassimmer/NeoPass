use clipboard::{ClipboardContext, ClipboardProvider};
use neopass::config::{INSTRUCTIONS, PASSWORD_COPIED};
use neopass::custom_select::{Select, SelectOutput};
use neopass::theme::custom_colorful_theme::ColorfulTheme;
use neopass::utils::{
    add_a_new_entry, add_first_entry, build_rows, clear_screen, display_end_of_table,
    get_user_password, modify_entry, write_entries_in_file,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut password = String::new();
    let mut entries = Vec::new();

    clear_screen();
    get_user_password(&mut entries, &mut password)?;

    let mut copied_item = None;

    loop {
        clear_screen();

        println!("  {}", INSTRUCTIONS);

        if entries.is_empty() {
            add_first_entry(&mut entries, &mut password)?;
            continue;
        }

        if copied_item.is_some() {
            println!("  {}", PASSWORD_COPIED);
        }

        let mut rows = build_rows(&entries);

        // Display table header.
        println!("  {}", rows.remove(0));
        println!("  {}", rows.remove(0));
        println!("  {}", rows.remove(0));

        // Prepare theme for select.
        let mut theme = ColorfulTheme::default();
        theme.last_line = rows[rows.len() - 1].to_string();

        // Display entries.
        if let Some(selection) = Select::with_theme(theme)
            .default(copied_item.unwrap_or_default())
            .items(&rows[..rows.len() - 1])
            .interact_opt()?
        {
            copied_item = None;

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
                }

                // User wants to delete an item.
                SelectOutput::Delete(index) => {
                    let _removed_instance = entries.remove(index);

                    write_entries_in_file(&entries, &password)?;
                }

                // User wants to modify one item.
                SelectOutput::Modify(index) => {
                    display_end_of_table(rows);

                    let modified_entry = modify_entry(&entries[index]);
                    entries[index] = modified_entry;

                    write_entries_in_file(&entries, &password)?;
                }
            }
        } else {
            display_end_of_table(rows);
            return Ok(());
        }
    }
}
