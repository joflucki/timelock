use crate::utils;
use anyhow::Result;

pub fn logout() -> Result<()> {
    utils::delete_keys()?;
    utils::delete_username()?;
    Ok(())
}
