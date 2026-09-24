use std::path::PathBuf;

use crate::error::ZkError;
mod git;

pub fn init(path: &PathBuf) -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::init(path)?;
    Ok(())
}

pub fn add() -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::add()?;
    Ok(())
}

pub fn commit() -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::commit()?;
    Ok(())
}
