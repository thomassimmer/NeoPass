# NeoPass

NeoPass is a command-line **password manager** written in Rust. It's designed to be simple, secure, and easy to use.

![NeoPass](/docs/neopass.gif)

## Features

- Store an unlimited number of passwords.
- Securely encrypt and decrypt passwords using a master password.
- Add, delete, and modify entries easily.
- Copy passwords to the clipboard for easy pasting.
- Ask for master password after 5 minutes of inactivity.
- Ability to change the language of the application, with the selected language being saved and utilized upon subsequent launches of the app.
- Ability to change the master password.

## Installation

To use NeoPass, you need to have Rust and Cargo installed on your machine. If you haven't installed them yet, you can do so by following the instructions provided in the official documentation [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Usage

To run NeoPass, navigate to the project directory and execute the following command:

```
cargo run
```
## Installation on Ubuntu

Make sure you have the following installed before running:

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

On the first run, you'll be prompted to choose a master password. This password will be used to encrypt and decrypt your password entries.

## Commands

- **↓**: Use this command to move the selection cursor down by one line in the list of entries.

- **↑**: Use this command to move the selection cursor up by one line in the list of entries.

- **a**: This command allows you to add a new entry to the list. You'll be prompted to provide details such as the application or website name, username or email, and password.

- **d**: Use this command to delete the currently selected entry from the list.

- **e**: This command enables you to edit the details of the currently selected entry, such as the application or website name, username or email, and password.

- **Space**: Pressing the Spacebar will copy the password of the currently selected entry to your clipboard, allowing for easy pasting into other applications.

- **l**: Use this command to change the language settings of the tool.

- **p**: Use this command to change the master password of the tool.

## Support

Please [open an issue](https://github.com/thomassimmer/NeoPass/issues/new/) for
support.

## Contributing

Please contribute using [Github Flow](https://guides.github.com/introduction/flow/). Create a branch, add commits, and [open a pull request](https://github.com/thomassimmer/NeoPass/compare).

## License

NeoPass is released under the MIT License. See the LICENSE file for more details.
