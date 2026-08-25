use std::fmt;
use std::path::PathBuf;

use anstream::println;

#[derive(Debug,PartialEq)]
pub enum ZkError {
    HomeCreate,
    NoHome,
    InvalidPath,
    InvalidNotePath(PathBuf),
    BoxCreateFail,
    BoxNotTracked,
    BoxRemove,
    InvalidBox,
    BoxCheck,
    NoteCreate,
    NoteExists,
    NoteNotExists,
    NoteRead(PathBuf),
    BoxExists,
    NoName,
    Access(PathBuf),
    TrackFail,
    IndexFail,
    GitInit,
    Current,
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
            ZkError::BoxNotTracked => write!(f, "box is not tracked"),
            ZkError::BoxRemove => write!(f, "failed to remove box"),
            ZkError::InvalidBox => write!(f, "box does not exist"),
            ZkError::BoxCheck => write!(f, "can't tell if box exists or not"),
            ZkError::NoteCreate => write!(f, "failed to create note"),
            ZkError::NoteExists => write!(f, "note already exists"),
            ZkError::NoteNotExists => write!(f, "note does not exist"),
            ZkError::NoteRead(path) => write!(f, "failed to read note: {}", path.display()),
            ZkError::BoxExists => write!(f, "box already exists"),
            ZkError::NoName => write!(f, "no name provided"),
            ZkError::Access(path) => write!(f, "unable to access {}", path.display()),
            ZkError::TrackFail => write!(f, "failed to track box"),
            ZkError::IndexFail => write!(f, "failed to create index file"),
            ZkError::GitInit => write!(f, "failed to innitialize git repo for box"),
            ZkError::Current => write!(f, "failed to update current box"),
            ZkError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ZkError {}

pub fn critical(msg: &str) {
    let red = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into())).bold();
    println!("{red}error:{red:#} {}", msg.trim());
}

pub fn warning(msg: &str) {
    let yellow = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into())).bold();
    println!("{yellow}warning:{yellow:#} {}", msg.trim());
}

pub fn info(msg: &str) {
    let blue = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())).bold();
    println!("{blue}info:{blue:#} {}", msg.trim());
}
