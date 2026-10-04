use std::path::PathBuf;

use crate::error::ZkError;
mod git;

pub fn init(path: &PathBuf) -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::init(path)?;
    Ok(())
}

pub fn add(base_dir: &PathBuf) -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::add(base_dir)?;
    Ok(())
}

pub fn commit(base_dir: &PathBuf) -> Result<(), ZkError> {
    #[cfg(feature = "git")]
    git::commit(base_dir)?;
    Ok(())
}
