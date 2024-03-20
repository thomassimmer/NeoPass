use neopass::custom_select::{Select, SelectOutput};
use neopass::entry::{add_a_new_entry, modify_entry};
use neopass::theme::custom_colorful_theme::ColorfulTheme;
use neopass::utils::{
    add_first_entry, build_rows, clear_screen, display_end_of_table, display_instructions,
    display_password_copied, get_user_password, set_password_in_clipboard, write_entries_in_file,
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
        display_instructions();

        if entries.is_empty() {
            add_first_entry(&mut entries, &mut password)?;
            continue;
        }

        if copied_item.is_some() {
            display_password_copied();
        }

        let rows = build_rows(&entries);
        let theme = ColorfulTheme {
            last_line: rows[rows.len() - 1].to_string(),
            ..Default::default()
        };

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
                    set_password_in_clipboard(&entries, index, &mut copied_item)?;
                }

                // User wants to add a new item.
                SelectOutput::Add => {
                    display_end_of_table(rows);
                    add_a_new_entry(&mut entries);
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
                    modify_entry(&mut entries, index);
                    write_entries_in_file(&entries, &password)?;
                }
            }
        } else {
            display_end_of_table(rows);
            return Ok(());
        }
    }
}
