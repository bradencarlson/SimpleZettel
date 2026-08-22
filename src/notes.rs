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
        match fs::exists(&file) {
            Ok(true) => {
                return Err(ZkError::NoteExists);
            },
            _ => {}
        };
        match File::create(&file) {
            Ok(F) => {
                edit_file(&file)?;
                Ok(())
            },
            Err(_) => Err(ZkError::NoteCreate)
        }
    } else {
        Err(ZkError::NoName)
    }

}

pub fn show_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let note = utils::note_path_from_name(name)?;
        if let Ok(content) = fs::read_to_string(&note) {
            print!("{}", content);
            Ok(())
        } else {
            Err(ZkError::NoteRead(note))
        }
    } else {
        Err(ZkError::NoName)
    }
}

pub fn rm_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let note = utils::note_path_from_name(name)?;
        match fs::exists(&note) {
            Ok(true) => {
                match fs::remove_file(&note) {
                    Ok(()) => {
                        println!("succesfully removed note");
                        Ok(())
                    }, 
                    Err(e) => {
                        Err(ZkError::Other(String::from("could not remove note")))
                    }
                }
            },
            _ => {
                Err(ZkError::Other(String::from("note does not exist")))
            }
        }
    } else {
        Err(ZkError::NoName)
    }
}


fn edit_file(path: &PathBuf) -> Result<(), ZkError> {
    utils::verify_note_path(path)?;
    match Command::new("vim")
        .arg(path)
        .status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(ZkError::Other(String::from("something went wrong while opening vim for the user")))
                }
            }, 
            Err(_) => Err(ZkError::Other(String::from("something went wrong while opening vim for the user")))
    }
}
