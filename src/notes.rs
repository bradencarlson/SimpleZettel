use std::fs;
use std::fmt;
use std::path::PathBuf;
use std::fs::File;
use std::process::Command;
use clap::ArgMatches;

use crate::error::ZkError;
use crate::utils;

pub fn add_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let mut file = utils::get_current_box()?;
        file.push(name);
        match File::create(&file) {
            Ok(F) => {
                Ok(())
            },
            Err(_) => Err(ZkError::NoteCreate)
        }
    } else {
        Err(ZkError::NoName)
    }

}

fn edit_note(path: &PathBuf) -> Result<(), ZkError> {
    Ok(())

}
