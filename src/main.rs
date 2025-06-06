use clap::{Parser, Subcommand};
use log::LevelFilter;

use std::path::PathBuf;

use imap::{ClientBuilder, TlsKind};
use sha1::{Digest, Sha1};
use hex;

mod config;

fn get_password(cmd: &str) -> anyhow::Result<String> {
    config::retrieve_password(cmd)
}

fn store_message(account: &config::EmailAccount, folder: &str, uid: u32, body: &[u8]) -> anyhow::Result<PathBuf> {
    let mut hasher = Sha1::new();
    hasher.update(folder.as_bytes());
    hasher.update(uid.to_string().as_bytes());
    let id = hex::encode(hasher.finalize());

    let mut dir = config::account_data_dir(&account.email);
    dir.push(folder);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.eml", id));
    std::fs::write(&path, body)?;
    log::info!("Saved message {} to {:?}", uid, path);
    Ok(path)
}

fn check_mail(folder: Option<String>) -> anyhow::Result<()> {
    let cfg = config::load_config()?;
    let account = cfg.email_account;
    let folder = folder
        .or(account.default_folder.clone())
        .unwrap_or_else(|| "inbox".to_string());

    let password = get_password(&account.password_cmd)?;
    let client = ClientBuilder::new(&account.imap_server, account.imap_port)
        .tls_kind(TlsKind::Native)
        .connect()?;
    let mut session = client.login(&account.username, password).map_err(|e| e.0)?;

    session.select(&folder)?;
    let uids = session.search("UNSEEN")?;
    for uid in uids.iter() {
        let fetches = session.fetch(uid.to_string(), "RFC822")?;
        for fetch in fetches.iter() {
            if let Some(body) = fetch.body() {
                let uid = fetch.uid.ok_or_else(|| anyhow::anyhow!("missing uid"))?;
                store_message(&account, &folder, uid, body)?;
            }
        }
    }

    session.logout()?;
    Ok(())
}
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
            check_mail(folder)?;
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
