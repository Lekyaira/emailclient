# RustMail

A minimal command line email client based on the design in `spec/rustmail_design_doc.md`.

This repository currently contains only the basic scaffolding:

- CLI commands implemented with `clap`
- Configuration loading from a platform specific location
- Stubs for future IMAP/SMTP integration
- The `check` command prints the number of new emails to stdout

Run `cargo run --help` for available commands.
