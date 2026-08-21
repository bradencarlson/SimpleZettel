use std::fs;
use std::fmt;
use std::process::Command;
use clap::ArgMatches;

use crate::error::ZkError;

pub fn add_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        Ok(())
    } else {
        Err(ZkError::NoName)
    }

}
