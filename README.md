# ProtonDB CLI

ProtonDB CLI is a command-line tool to fetch and display game summaries from the ProtonDB API. It provides information about game ratings, Steam Deck compatibility, and more.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- - [Arch Linux](#arch-linux)
- - [AppImage](#appimage)
- [Compiling from source](#compiling-from-source)
- [Contributing](#contributing)
- [License](#license)

## Features

- Fetch game summaries from ProtonDB.
- Display game ratings and Steam Deck compatibility.
- Optionally display game images using the `viuer` crate.

## Installation

### Arch Linux

`protondb-cli` is available as a package in the [AUR](https://aur.archlinux.org). You can install it with your preferred [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). Example:
```sh
paru -S protondb-cli
```

### AppImage

Download the latest AppImage from the [releases page](https://github.com/hypeedev/protondb-cli/releases/latest).

Make sure the file is executable:
```sh
chmod +x protondb-cli.AppImage
```

Run the AppImage:
```sh
./protondb-cli.AppImage --help
```

You can also move the AppImage to a directory in your `$PATH` to run it from anywhere:
```sh
sudo mv protondb-cli.AppImage /usr/local/bin/protondb-cli
protondb-cli --help
```

## Compiling from source

Ensure you have Rust and Cargo installed on your system. You can install Rust and Cargo using [rustup](https://rustup.rs/).

Clone the repository:
```sh
git clone https://github.com/hypeedev/protondb-cli.git
cd protondb-cli
```

Build the project:
```sh
cargo build --release
```

Run the executable:
```sh
./target/release/protondb-cli --help
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

<br>

**Disclaimer:** This software is an independent project and is not affiliated with ProtonDB in any way.
