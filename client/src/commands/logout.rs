use crate::utils;
use anyhow::Result;

/// Logs the current user out, by deleting local authentication file.
pub fn logout() -> Result<()> {
    utils::delete_keys()?;
    utils::delete_username()?;
    println!("Log out successful");
    Ok(())
}
