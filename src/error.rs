use std::fmt;
use std::io;
use std::path::PathBuf;

use anstream::println;

#[derive(Debug,PartialEq)]
pub enum ZkError {
    Access(PathBuf),
    BoxCheck,
    BoxCreateFail,
    BoxNotTracked,
    BoxRemove,
    BoxExists,
    BoxInvalid,
    ConfigError,
    ConfigNotExists,
    ConfigRead,
    Current,
    GitAdd,
    GitCommit,
    GitInit,
    HomeCreate,
    ImportArgs,
    ImportFail,
    InvalidNotePath(PathBuf),
    InvalidPath,
    NoCurrentBox,
    NoHome,
    NoName,
    NoteAddArgs,
    NoteCreate,
    NoteExists,
    NoteHeader,
    NoteNotExists,
    NoteNumber,
    NoteRead(PathBuf),
    NoteShow(PathBuf),
    Other(String),
    TrackFail,
}

impl fmt::Display for ZkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZkError::Access(path) => write!(f, "unable to access {}", path.display()),
            ZkError::BoxCheck => write!(f, "can't tell if box exists or not"),
            ZkError::BoxCreateFail => write!(f, "failed to create box"),
            ZkError::BoxExists => write!(f, "box already exists"),
            ZkError::BoxInvalid => write!(f, "box does not exist"),
            ZkError::BoxNotTracked => write!(f, "box is not tracked"),
            ZkError::BoxRemove => write!(f, "failed to remove box"),
            ZkError::ConfigError => write!(f, "Error in config file"),
            ZkError::ConfigNotExists => write!(f, "no config file found"),
            ZkError::ConfigRead => write!(f, "unable to read config file"),
            ZkError::Current => write!(f, "failed to update current box"),
            ZkError::GitAdd => write!(f, "failed to add files to box's git repo"),
            ZkError::GitCommit => write!(f, "failed to commit files to box's git repo"),
            ZkError::GitInit => write!(f, "failed to innitialize box's git repo"),
            ZkError::HomeCreate => write!(f, "failed to create ~/.szettel"),
            ZkError::ImportArgs => write!(f, "invalid arguments for the import command"),
            ZkError::ImportFail => write!(f, "failed to import file to current box"),
            ZkError::InvalidNotePath(path) => write!(f, "invalid note path: {}", path.display()),
            ZkError::InvalidPath => write!(f, "Something weird has happened; I tried to access a directory outside of ~/.szettel"),
            ZkError::NoCurrentBox => write!(f, "No current box. Run `szettel box` to view boxes, and `szettel use`\nto set the current box."),
            ZkError::NoHome =>  write!(f, "failed to find users home directory"),
            ZkError::NoName => write!(f, "no name provided"),
            ZkError::NoteAddArgs => write!(f, "no name or number found"),
            ZkError::NoteCreate => write!(f, "failed to create note"),
            ZkError::NoteExists => write!(f, "note already exists"),
            ZkError::NoteHeader => write!(f, "could not find header"),
            ZkError::NoteNotExists => write!(f, "note does not exist"),
            ZkError::NoteNumber => write!(f, "invalid note number"),
            ZkError::NoteRead(path) => write!(f, "failed to read note: {}", path.display()),
            ZkError::NoteShow(path) => write!(f, "failed to show note: {}", path.display()),
            ZkError::Other(msg) => write!(f, "{}", msg),
            ZkError::TrackFail => write!(f, "failed to track box"),
        }
    }
}

impl std::error::Error for ZkError {}

impl From<io::Error> for ZkError {
    fn from(value: io::Error) -> Self {
        ZkError::Other(value.to_string())
    }
}

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
