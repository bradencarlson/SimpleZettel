use std::env;
use std::fs;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug,PartialEq)]
pub enum GenError {
    ZkHomeCreate,
    NoHome,
    InvalidPath,
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenError::NoHome => write!(f, "failed to find user's home directory"),
            GenError::ZkHomeCreate => write!(f, "failed to create ~/.zk directory"),
            GenError::InvalidPath => write!(f, "Something weird has happened; I tried to access a directory outside of ~/.zk"),
        }
    }
}

impl std::error::Error for GenError {}

pub fn path_from_name(name: &str) -> Result<PathBuf, GenError> {
    let mut root = get_zk_dir()?;
    root.push(name);
    match verify_path(&root) {
        Ok(_) => Ok(root),
        Err(e) => Err(e)
    }
}

pub fn get_zk_dir() -> Result<PathBuf, GenError> {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/");
        match fs::exists(&path) {
            Ok(true) => Ok(path),
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => Ok(path),
                    Err(_) => Err(GenError::ZkHomeCreate)
                }
            },
            Err(_) => Err(GenError::ZkHomeCreate)
        }
    } else {
        Err(GenError::NoHome)
    }

}

pub fn verify_path(path: &PathBuf) -> Result<(), GenError> {
    let zk_home = get_zk_dir()?;
    if let Some(parent) = path.parent() {
        if parent == zk_home {
            Ok(())
        } else {
            Err(GenError::InvalidPath)
        }
    } else {
        Err(GenError::InvalidPath)
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
    assert_eq!(verify_path(&invalid_path), Err(GenError::InvalidPath));
}

#[test]
fn deep_path() {
    if let Some(mut invalid_path) = env::home_dir() {
        invalid_path.push(".zk/test/one");
        assert_eq!(verify_path(&invalid_path), Err(GenError::InvalidPath));
    }
}
