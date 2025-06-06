use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

/// Top level configuration structure for RustMail.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Email account configuration.
    pub email_account: EmailAccount,
}

/// Configuration for a single email account.
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailAccount {
    /// Email address used for this account.
    pub email: String,
    /// IMAP server address.
    pub imap_server: String,
    /// IMAP server port.
    pub imap_port: u16,
    /// SMTP server address.
    pub smtp_server: String,
    /// SMTP server port.
    pub smtp_port: u16,
    /// Username for authentication.
    pub username: String,
    /// Command to retrieve the password or token.
    pub password_cmd: String,
    /// Default folder to fetch mail from.
    pub default_folder: Option<String>,
    /// Whether to use TLS when connecting.
    pub use_tls: Option<bool>,
}

/// Return the default configuration file path depending on the platform.
///
/// Linux & macOS: `~/.config/rustmail/config.toml`
/// Windows: `%APPDATA%\\rustmail\\config.toml`
pub fn default_config_path() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("rustmail");
    dir.push("config.toml");
    dir
}

/// Load configuration from the default path.
pub fn load_config() -> anyhow::Result<Config> {
    let path = default_config_path();
    let data = std::fs::read_to_string(path)?;
    let cfg = toml::from_str(&data)?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_path() {
        let path = default_config_path();
        let s = path.to_string_lossy();
        assert!(s.ends_with("rustmail/config.toml") || s.ends_with("rustmail\\config.toml"));
    }
}
