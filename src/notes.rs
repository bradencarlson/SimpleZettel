use std::fs;
use std::fmt;
use std::process::Command;
use clap::ArgMatches;

#[derive(Debug)]
pub enum NoteError {
    NoteExists,
    NoName,
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoteError::NoteExists => write!(f, "note already exists"),
            NoteError::NoName => write!(f, "no name was provided"),
        }
    }
}

impl std::error::Error for NoteError {}

pub fn add_note(matches: &ArgMatches) -> Result<(), NoteError> {
    if let Some(name) = matches.get_one::<String>("name") {
        Ok(())
    } else {
        Err(NoteError::NoName)
    }

}
