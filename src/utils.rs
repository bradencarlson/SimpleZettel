use std::env;
use std::fs;
use std::fmt;
use std::path::PathBuf;

use crate::error::ZkError;

pub fn path_from_name(name: &str) -> Result<PathBuf, ZkError> {
    let mut root = get_zk_dir()?;
    root.push(name);
    match verify_path(&root) {
        Ok(_) => Ok(root),
        Err(e) => Err(e)
    }
}

pub fn get_zk_dir() -> Result<PathBuf, ZkError> {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/");
        match fs::exists(&path) {
            Ok(true) => Ok(path),
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => Ok(path),
                    Err(_) => Err(ZkError::HomeCreate)
                }
            },
            Err(_) => Err(ZkError::HomeCreate)
        }
    } else {
        Err(ZkError::NoHome)
    }

}

pub fn verify_path(path: &PathBuf) -> Result<(), ZkError> {
    let zk_home = get_zk_dir()?;
    if let Some(parent) = path.parent() {
        if parent == zk_home {
            Ok(())
        } else {
            Err(ZkError::InvalidPath)
        }
    } else {
        Err(ZkError::InvalidPath)
    }

}

#[test]
fn valid_path() {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/test");
        assert_eq!(verify_path(&path), Ok(()));
    }

}

#[test]
fn invalid_path() {
    let invalid_path = PathBuf::from("~/Documents/dir");
    assert_eq!(verify_path(&invalid_path), Err(ZkError::InvalidPath));
}

#[test]
fn deep_path() {
    if let Some(mut invalid_path) = env::home_dir() {
        invalid_path.push(".zk/test/one");
        assert_eq!(verify_path(&invalid_path), Err(ZkError::InvalidPath));
    }
}
