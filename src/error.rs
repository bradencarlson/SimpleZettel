use std::fmt;
use std::path::PathBuf;

#[derive(Debug,PartialEq)]
pub enum ZkError {
    HomeCreate,
    NoHome,
    InvalidPath,
    InvalidNotePath(PathBuf),
    BoxCreateFail,
    NoteCreate,
    BoxExists,
    NoName,
    Access(PathBuf),
    TrackFail,
    IndexFail,
    GitInit,
    Other(String),
}

impl fmt::Display for ZkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZkError::HomeCreate => write!(f, "failed to create ~/.zk"),
            ZkError::NoHome =>  write!(f, "failed to find users home directory"),
            ZkError::InvalidPath => write!(f, "Something weird has happened; I tried to access a directory outside of ~/.zk"),
            ZkError::InvalidNotePath(path) => write!(f, "invalid note path: {}", path.display()),
            ZkError::BoxCreateFail => write!(f, "failed to create box"),
            ZkError::NoteCreate => write!(f, "failed to create note"),
            ZkError::BoxExists => write!(f, "box already exists"),
            ZkError::NoName => write!(f, "no name provided"),
            ZkError::Access(path) => write!(f, "unable to access {}", path.display()),
            ZkError::TrackFail => write!(f, "failed to track box"),
            ZkError::IndexFail => write!(f, "failed to create index file"),
            ZkError::GitInit => write!(f, "failed to innitialize git repo for box"),
            ZkError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ZkError {}
