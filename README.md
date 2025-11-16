# LanJet — Fast LAN File Sharing in One Command

![Rust Version](https://img.shields.io/badge/rust-1.90+-orange)
![Build Status](https://img.shields.io/github/actions/workflow/status/HuRuilizhen/lanjet/rust.yml?branch=release)
![Crates.io](https://img.shields.io/crates/v/lanjet)

A blazing-fast, zero-config LAN file sharing tool written in Rust. Start a lightweight HTTP file server with a clean Web UI and QR code access — all in one command.

## Features

### Core

- Recursive directory scanning
- Predictable and safe path handling
- Automatic LAN IP detection
- MIME type detection
- Graceful shutdown (`Ctrl+C`)
- Structured access logs (`tracing` + `tower-http`)

### CLI & UX

- Beautiful startup banner
- Optional QR code for mobile access
- Auto-open browser on startup
- Configurable host binding (`--local-only`)
- `.lanjetignore` support (similar to `.gitignore`)

### Web UI

- Clean HTML interface (no JS, no external assets)
- Click to download files
- File icons based on MIME type
- File metadata (size, type, modified time)

## Installation

### Install via Cargo

```bash
cargo install lanjet
```

### Install via cargo-binstall

```bash
cargo binstall lanjet
```

## Quick Start

Share the current directory:

```bash
lanjet
```

Share a specific path:

```bash
lanjet --path /path/to/files
```

Start on a different port:

```bash
lanjet --port 8080
```

Local-only mode:

```bash
lanjet --local-only
```

Show QR code in banner:

```bash
lanjet --show-qrcode
```

## CLI Options

| Flag            | Description                | Default         |
| --------------- | -------------------------- | --------------- |
| `--path`        | File or directory to share | `.`             |
| `--port`        | Port to bind               | `80`            |
| `--ignore`      | Ignore rule file           | `.lanjetignore` |
| `--local-only`  | Bind only to localhost     | `false`         |
| `--show-qrcode` | Display QR code in banner  | `false`         |
| `--no-browser`  | Disable auto-open browser  | `false`         |
| `--no-banner`   | Disable banner             | `false`         |

## Ignore Rules

LanJet supports ignore files similar to `.gitignore`.

Example `.lanjetignore`:

```
*.log
*.tmp
.DS_Store
node_modules/
target/
```

## Web Interface

After starting the service, open:

```
http://<LAN-IP>:<PORT>
```

You will see a clean interface listing all files.

## License

This project is licensed under the MIT License.

---

> **LanJet — Making LAN file sharing fast, simple, and beautiful.**
