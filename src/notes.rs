use std::fs;
use std::process::Command;
use clap::ArgMatches;

#[derive(Debug)]
pub enum NoteError {
    NoteExists
}

pub fn add_note(matches: &ArgMatches) -> Result<(), NoteError> {
    Ok(())
}
