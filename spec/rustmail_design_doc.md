# 📬 RustMail: A Minimal Rust Email Client

## Overview

**RustMail** is a secure, CLI-based email client written in Rust, designed for simplicity, scriptability, and modular extensibility. The MVP focuses on basic IMAP/SMTP support, plaintext emails, flat-file storage, and efficient CLI usage.

---

## 🎯 MVP Goals

- Fetch emails from IMAP servers (e.g., Gmail, Outlook.com)
- Send plaintext emails via SMTP
- Provide simple, script-friendly CLI commands
- Store emails as flat files under `~/.local/share/rustmail`
- Store configuration in TOML format under `~/.config/rustmail`
- Support multiple email addresses

---

## 🧰 CLI Commands

### `check`
Check for new/unread mail in the default or specified folder. The command prints
the number of new messages to standard output.

```bash
rustmail check
rustmail check inbox
# prints the number of new emails (e.g. `3`)
```

### `list [<folder>] [--limit N] [--offset N]`
List recent emails in a folder, defaulting to the inbox. Supports pagination.

```bash
rustmail list
rustmail list inbox --limit 20 --offset 40
```

### `read <folder>/<id>`
Read the full content of a locally stored email by path reference.

```bash
rustmail read inbox/b9f1c6d3c3b8f8c2c442f15f66fd8c7e0f1a2f1b
rustmail read inbox/<subject>
```

### `send`
Send a plaintext email. Three input methods for the message body:

```bash
rustmail send --to alice@example.com --subject "Hi" --body "Hello there!"
rustmail send --to bob@example.com --subject "From File" --body-file ./msg.txt
echo "Body from stdin" | rustmail send --to eve@example.com --subject "Stdin Test"
```

---

## 📦 File and Email Storage

### Path structure:

```
~/.local/share/rustmail/<email-address>/<folder>/<id>.eml
```

### Email ID Format:
- SHA-1 or BLAKE3 hash of: `<folder> + <imap_uid>`
- Stable and unique across sessions
- Enables direct referencing via CLI

### Optional metadata:
Per-folder index file:
```
~/.local/share/rustmail/<email-address>/<folder>/index.json
```
Contains:
- UID
- From, To, Subject
- Timestamp
- Read status

---

## ⚙️ Configuration

Stored as:
```
~/.config/rustmail/config.toml
```

### Example:

```toml
[email_account]
email = "me@example.com"
imap_server = "imap.example.com"
imap_port = 993
smtp_server = "smtp.example.com"
smtp_port = 587
username = "me@example.com"
password_file = "passwords/me@example.com"
default_folder = "inbox"
use_tls = true
```

- password_file: Optional, ask for password if does not exist. File should be secured and contents hashed.
- Multiple profiles can be added via `[email_account.<label>]` for future multi-account support

---

## 🌐 Protocol Support

### IMAP (Receiving)
- Connect via TLS (port 993)
- Fetch folders and message headers
- Fetch full message body by UID

### SMTP (Sending)
- Connect via STARTTLS (port 587)
- Authenticate and send plaintext messages

### Authentication
- MVP: Username + password or app password
- Post-MVP: OAuth2 for Gmail/Outlook.com

---

## 🧪 Logging & Error Handling

- Print clear errors on connection/auth issues
- `--verbose` flag for debugging
- Logging to stderr or optional log file

---

## 📍 Post-MVP Milestones

| Feature | Description |
|--------|-------------|
| OAuth2 Support | Required for modern Gmail/Outlook authentication |
| SQLite Metadata Index | Enable fast local search and filtering |
| CLI GUI / REPL | Interactive command session |
| Graphical UI | Built with `egui` or `eframe` |
| Maildir Support | Interop with other tools (optional) |

---

## 🧱 Technical Stack

| Component | Tool/Crate |
|----------|------------|
| IMAP     | [`imap`](https://crates.io/crates/imap) |
| SMTP     | [`lettre`](https://crates.io/crates/lettre) |
| TLS      | [`native-tls`] or [`rustls-tls`] |
| Config   | [`toml`, `serde`, `dirs`] |
| CLI      | [`clap`] |
| Hashing  | [`sha1`, `blake3`] |
| Logging  | [`log`, `env_logger`] |
| Parsing  | [`mailparse`] |

---

## ✅ Summary

RustMail MVP is a lean, secure, scriptable email client with:
- IMAP/SMTP support
- Configurable CLI
- Flat-file storage per email account
- Secure defaults
- Clear path for future enhancements
