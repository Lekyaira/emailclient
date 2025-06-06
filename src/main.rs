use clap::{Parser, Subcommand};
use log::LevelFilter;

mod config;
mod mail;
/// RustMail command line interface.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Verbose output
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

/// Supported subcommands of RustMail.
#[derive(Subcommand)]
enum Commands {
    /// Check for new or unread mail
    Check { folder: Option<String> },
    /// List recent emails in a folder
    List {
        folder: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        offset: Option<u32>,
    },
    /// Read an email from local storage
    Read { path: String },
    /// Send a plaintext email
    Send {
        #[arg(long)]
        to: String,
        #[arg(long)]
        subject: Option<String>,
        #[arg(long)]
        body: Option<String>,
        #[arg(long, value_name = "FILE")]
        body_file: Option<std::path::PathBuf>,
    },
}

fn init_logger(levels: u8) {
    let level = match levels {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    env_logger::Builder::new()
        .filter_level(level)
        .init();
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_logger(cli.verbose);

    match cli.command {
        Commands::Check { folder } => {
            mail::imap::check_mail(folder)?;
        }
        Commands::List { folder, limit, offset } => {
            println!(
                "Listing emails in {:?} limit {:?} offset {:?}",
                folder.unwrap_or_else(|| "inbox".into()),
                limit,
                offset
            );
        }
        Commands::Read { path } => {
            println!("Reading email {}", path);
        }
        Commands::Send { to, subject, body, body_file } => {
            println!("Sending email to {} with subject {:?}", to, subject);
            if let Some(file) = body_file {
                println!("Using body from file {:?}", file);
            } else if let Some(text) = body {
                println!("Body: {}", text);
            }
        }
    }

    Ok(())
}
