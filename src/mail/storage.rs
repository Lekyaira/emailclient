use std::path::PathBuf;

use sha1::{Digest, Sha1};

use crate::config;

/// Store an email message body to disk and return the file path.
pub fn store_message(
    account: &config::EmailAccount,
    folder: &str,
    uid: u32,
    body: &[u8],
) -> anyhow::Result<PathBuf> {
    let mut hasher = Sha1::new();
    hasher.update(folder.as_bytes());
    hasher.update(uid.to_string().as_bytes());
    let id = hex::encode(hasher.finalize());

    let mut dir = config::account_data_dir(&account.email)?;
    dir.push(folder);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.eml", id));
    std::fs::write(&path, body)?;
    log::info!("Saved message {} to {:?}", uid, path);
    Ok(path)
}
