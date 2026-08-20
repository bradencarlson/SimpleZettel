use std::fs::DirBuilder;
use clap::ArgMatches;
use std::io;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum BoxError {
    CreateFail,
    BoxExists,
    NoName,
    CheckFail,
} 

impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxError::CreateFail => write!(f, "failed to create box"),
            BoxError::BoxExists => write!(f, "box already exists"),
            BoxError::NoName => write!(f, "you must provide a name for the box"),
            BoxError::CheckFail => write!(f, "failed to check box existence"),
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
    let mut path = String::from("~/.zk/");

    if let Ok(false) = fs::exists(path.as_str()) {

    }
    if let Some(name) = matches.get_one::<String>("name") {
        path.push_str(name.as_str());

        if let Ok(exist) = fs::exists(path.as_str()) {
            if exist {
                return Err(BoxError::BoxExists);
            }
        } else {
            return Err(BoxError::CheckFail);
        }

        match DirBuilder::new().create(path) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(BoxError::CreateFail)
        }
    } else {
        Err(BoxError::NoName)
    }

}
