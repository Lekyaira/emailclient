use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

use anyhow::Context;

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
    match std::fs::read_to_string(&path) {
        Ok(data) => {
            let cfg = toml::from_str(&data)?;
            Ok(cfg)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            const EXAMPLE: &str = "[email_account]\nemail = \"me@example.com\"\nimap_server = \"imap.example.com\"\nimap_port = 993\nsmtp_server = \"smtp.example.com\"\nsmtp_port = 587\nusername = \"me@example.com\"\npassword_cmd = \"pass rustmail/me@example.com\"\ndefault_folder = \"inbox\"\nuse_tls = true\n";
            std::fs::write(&path, EXAMPLE)?;
            anyhow::bail!(
                "created default config at {}. Please fill in your account details",
                path.display()
            );
        }
        Err(e) => Err(e.into()),
    }
}

/// Run `cmd` using the system shell and return its trimmed stdout as a String.
pub fn retrieve_password(cmd: &str) -> anyhow::Result<String> {
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("running password command: {}", cmd))?;
    if !out.status.success() {
        anyhow::bail!("password command exited with status: {}", out.status);
    }
    let pwd = String::from_utf8(out.stdout)?;
    Ok(pwd.trim().to_string())
}

/// Return the base directory for storing mail for the given email address.
pub fn account_data_dir(email: &str) -> anyhow::Result<PathBuf> {
    let mut dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("rustmail");
    dir.push(email);
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_default_config_path() {
        let path = default_config_path();
        let s = path.to_string_lossy();
        assert!(s.ends_with("rustmail/config.toml") || s.ends_with("rustmail\\config.toml"));
    }

    #[test]
    fn test_account_data_dir_creates_dir() {
        let tmp = TempDir::new().unwrap();
        let prev = env::var_os("XDG_DATA_HOME");
        unsafe {
            env::set_var("XDG_DATA_HOME", tmp.path());
        }
        let path = account_data_dir("me@example.com").unwrap();
        unsafe {
            env::remove_var("XDG_DATA_HOME");
            if let Some(v) = prev { env::set_var("XDG_DATA_HOME", v); }
        }
        assert!(path.exists());
        let s = path.to_string_lossy();
        assert!(s.contains("rustmail"));
        assert!(s.ends_with("me@example.com") || s.ends_with("me@example.com"));
    }

    #[test]
    fn test_load_config_missing_file_creates_dir() {
        let dir = TempDir::new().unwrap();
        let prev = env::var_os("XDG_CONFIG_HOME");
        unsafe {
            env::set_var("XDG_CONFIG_HOME", dir.path());
        }
        let result = load_config();
        unsafe {
            env::remove_var("XDG_CONFIG_HOME");
            if let Some(val) = prev { env::set_var("XDG_CONFIG_HOME", val); }
        }
        assert!(result.is_err());
        let config_dir = dir.path().join("rustmail");
        assert!(config_dir.exists());
        assert!(config_dir.join("config.toml").exists());
    }
}
