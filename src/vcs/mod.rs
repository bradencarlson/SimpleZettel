
use crate::error::ZkError;
mod git;

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
