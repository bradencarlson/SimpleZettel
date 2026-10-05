use std::path::PathBuf;
use std::process::Command;
use crate::error::ZkError;
use crate::utils;
use crate::boxes;

pub fn add(base_dir: &PathBuf) -> Result<(), ZkError> {
    match Command::new("git")
        .current_dir(base_dir)
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
pub fn commit(base_dir: &PathBuf) -> Result<bool, ZkError> {
    let resp = utils::prompt_user("Would you like to commit changes to git? [Y/n]")?;
    let yes = String::from("Y");
    let y = String::from("y");
    if resp != yes && resp != y {
        return Ok(false);
    }
    match Command::new("git")
        .current_dir(base_dir)
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

pub fn init(path: &PathBuf) -> Result<(), ZkError> {
    match Command::new("git")
        .arg("init")
        .arg(path)
        .output() {
            Ok(_) => {
                println!("successfully initialized git for box");
                Ok(())
            },
            Err(_) => Err(ZkError::GitInit)
    }
}
