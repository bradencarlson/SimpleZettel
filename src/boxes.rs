use std::fs::DirBuilder;
use clap::ArgMatches;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum BoxError {
    CreateFail,
} 

impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoxError::CreateFail => write!(f, "failed to create box"),
            _ => write!(f, "unknown error")

        }
    }
}

impl std::error::Error for BoxError {}


pub fn handle_subcommand(matches: &ArgMatches) -> Result<(), BoxError> {
    match matches.subcommand() {
        Some(("ls", ssub_m)) => {
            println!("ls command found");
        },
        Some(("add", ssub_m)) => {
            create_box(ssub_m)?;
        },
        Some(("rm", ssub_m)) => {
            println!("rm subcommand found");
        },
        _ => {
            println!("No subcommand found.");
        }
    }
    Ok(())
}

fn create_box(matches: &ArgMatches) -> Result<(), BoxError> {
    Ok(())
}
