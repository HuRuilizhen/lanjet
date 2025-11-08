# LanJet 🚀

![Rust Version](https://img.shields.io/badge/rust-1.90+-orange)

A blazing-fast LAN file sharing tool built with Rust, making file distribution simple and efficient.

## Features

- Basic HTTP file server
- Simple file listing interface
- One-click file downloads
- Automatic LAN IP detection

## Requirements

- Rust stable toolchain (1.90+)
- Supported OS: Windows, macOS, Linux

## Table of Contents

- [LanJet 🚀](#lanjet-)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Basic Usage](#basic-usage)
  - [License](#license)

## Getting Started

### Installation

```bash
# Install from source
cargo install --git https://github.com/HuRuilizhen/lanjet.git
```

### Basic Usage

```bash
# Share current directory using default port 80
lanjet

# Share specific directory
lanjet --dir /path/to/your/files

# Start with specific port
lanjet --port 8080
```

For more usage information, run `lanjet --help`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

**LanJet** - Making LAN file sharing as fast as a jet! ✈️
