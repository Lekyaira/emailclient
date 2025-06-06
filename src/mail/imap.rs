use imap::{ClientBuilder, TlsKind};

use crate::config;
use super::auth::get_password;
use super::storage::store_message;

/// Connect to the IMAP server and check for new/unread mail.
/// Any new messages are stored locally and the count is printed to stdout.
pub fn check_mail(folder: Option<String>) -> anyhow::Result<()> {
    let cfg = config::load_config()?;
    let account = cfg.email_account;
    let folder = folder
        .or_else(|| account.default_folder.clone())
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

    println!("{}", uids.len());

    session.logout()?;
    Ok(())
}
