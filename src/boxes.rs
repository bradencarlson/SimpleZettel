use std::fs::DirBuilder;
use std::env;
use std::io;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use clap::ArgMatches;

#[derive(Debug)]
pub enum BoxError {
    CreateFail,
    BoxExists,
    NoName,
    CheckFail,
    NoHome,
    ZkHomeCreate,

} 

impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxError::CreateFail => write!(f, "failed to create box"),
            BoxError::BoxExists => write!(f, "box already exists"),
            BoxError::NoName => write!(f, "you must provide a name for the box"),
            BoxError::CheckFail => write!(f, "failed to check box existence"),
            BoxError::NoHome => write!(f, "failed to find user's home directory"),
            BoxError::ZkHomeCreate => write!(f, "failed to create ~/.zk directory"),
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

fn get_zk_dir() -> Result<PathBuf, BoxError> {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/");
        match fs::exists(&path) {
            Ok(true) => Ok(path), 
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => Ok(path),
                    Err(_) => Err(BoxError::ZkHomeCreate)
                }
            }, 
            Err(_) => Err(BoxError::ZkHomeCreate)
        }
    } else {
        Err(BoxError::NoHome)
    }

}

fn create_box(matches: &ArgMatches) -> Result<(), BoxError> {
    let mut path = get_zk_dir()?;

    if let Some(name) = matches.get_one::<String>("name") {
        path.push(name.as_str());

        match fs::exists(&path) {
            Ok(true) => Err(BoxError::BoxExists),
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(BoxError::CreateFail)
                }
            },
            Err(_) => return Err(BoxError::CheckFail)
        }

    } else {
        Err(BoxError::NoName)
    }
}
