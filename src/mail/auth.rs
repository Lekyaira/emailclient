use crate::config;

/// Fetch the account password using the configured command.
pub fn get_password(cmd: &str) -> anyhow::Result<String> {
    config::retrieve_password(cmd)
}
