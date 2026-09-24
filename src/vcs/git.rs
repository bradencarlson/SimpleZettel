use std::process::Command;
use crate::error::ZkError;
use crate::utils;

pub fn add() -> Result<(), ZkError> {
    let current = utils::get_current_box()?;
    match Command::new("git")
        .current_dir(&current)
        .arg("add")
        .arg(".")
        .status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(ZkError::GitAdd)
                }
            },
            Err(e) => {
                Err(ZkError::Other(e.to_string()))
            }
    }

}

#[cfg(feature = "git")]
pub fn commit() -> Result<bool, ZkError> {
    let resp = utils::prompt_user("Would you like to commit changes to git? [Y/n]")?;
    let Y = String::from("Y");
    let y = String::from("y");
    if resp != Y && resp != y {
        return Ok(false);
    }
    let current = utils::get_current_box()?;
    match Command::new("git")
        .current_dir(&current)
        .arg("commit")
        .status() {
            Ok(status) => {
                if status.success() {
                    Ok(true)
                } else {
                    Err(ZkError::GitCommit)
                }
            },
            Err(e) => {
                Err(ZkError::Other(e.to_string()))
            }
    }
}

