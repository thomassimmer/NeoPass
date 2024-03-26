pub const PASSWORD_LENGTH: usize = 20;
pub const SYMBOLS_TO_USE_IN_PASSWORDS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!*#_-|&@~$";
pub const FILE_PATH: &str = "passwords.txt";
pub const CLEAR_SCREEN: &str = "\x1b[2J\x1b[1;1H";
pub const ENTER_NEW_PASSWORD: &str = "Enter a master password:";
pub const ENTER_PASSWORD: &str = "Enter your master password:";
pub const PASSWORD_COPIED: &str = "✅ Copied password to clipboard.\n";
pub const INSTRUCTIONS: &str = "
  Use ↑ and ↓ arrows to navigate between entries.\n  \
Press 'a' or 'A' to add an new entry.\n  \
Press 'm' or 'M' to modify an entry.\n  \
Press 'd', 'D' or Del to delete an entry.\n  \
Press Enter or Space to copy the password in your clipboard.\n  \
Use 'q' or Ctrl + C to quit.\n";
pub const INVALID_PASSWORD: &str = "❌ Invalid password.";
pub const CHECKING_PASSWORD: &str = "Checking...";
pub const NO_PASSWORD: &str = "You do not have credentials yet. Add one:";
pub const INACTIVITY_DELAY: u64 = 5 * 60;