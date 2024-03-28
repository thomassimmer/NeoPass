use console::style;
use dialoguer::theme::ColorfulTheme;
use neopass::config::{read_local_config, INACTIVITY_DELAY};
use neopass::entry::{add_a_new_entry, modify_entry};
use neopass::languages::{read_locales, select_language};
use neopass::select::{Select, SelectOutput};
use neopass::utils::{
    add_first_entry, build_rows, clear_screen, display_instructions, get_user_password,
    set_password_in_clipboard, write_entries_in_file,
};
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    read_local_config()?;
    read_locales();

    let mut password = String::new();
    let mut entries = Vec::new();

    clear_screen()?;
    get_user_password(&mut entries, &mut password)?;

    let mut copied_item = None;
    let mut _last_activity = Instant::now();

    loop {
        if entries.is_empty() {
            add_first_entry(&mut entries, &mut password)?;
            _last_activity = Instant::now();
            clear_screen()?;
            continue;
        }

        display_instructions();

        let (rows, header, footer) = build_rows(&entries, &copied_item);

        let theme = ColorfulTheme {
            header: style(header).for_stderr(),
            footer: style(footer).for_stderr(),
            ..Default::default()
        };

        // Reset the timer on user activity
        _last_activity = Instant::now();

        // Display entries.
        if let Some(selection) = Select::with_theme(&theme)
            .default(copied_item.unwrap_or_default())
            .items(&rows)
            .interact_opt()?
        {
            copied_item = None;

            // Check if INACTIVITY_DELAY seconds have elapsed since the last activity
            if _last_activity.elapsed() >= Duration::from_secs(INACTIVITY_DELAY) {
                clear_screen()?;
                get_user_password(&mut entries, &mut password)?;
                _last_activity = Instant::now();
            }

            match selection {
                // User selected one item.
                SelectOutput::Copy(index) => {
                    set_password_in_clipboard(&entries, index, &mut copied_item)?;
                }

                // User wants to add a new item.
                SelectOutput::Add => {
                    clear_screen()?;
                    add_a_new_entry(&mut entries);
                    write_entries_in_file(&entries, &password)?;
                }

                // User wants to delete an item.
                SelectOutput::Delete(index) => {
                    let _removed_instance = entries.remove(index);
                    write_entries_in_file(&entries, &password)?;
                }

                // User wants to modify one item.
                SelectOutput::Edit(index) => {
                    clear_screen()?;
                    modify_entry(&mut entries, index);
                    write_entries_in_file(&entries, &password)?;
                }

                // Users wants to change the language.
                SelectOutput::ChangeLanguage => {
                    clear_screen()?;
                    select_language()?;
                }
            }
        } else {
            return Ok(());
        }
        clear_screen()?;
    }
}
