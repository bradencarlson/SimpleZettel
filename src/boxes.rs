use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::cmp::Ordering;
use clap::ArgMatches;
use std::error::Error;

use crate::utils;
use crate::error::ZkError;

struct ZkBox {
    name: String
}

impl ZkBox {
    pub fn get_path(&self) -> Option<PathBuf> {
        if let Ok(mut path) = utils::get_zk_dir() {
            if self.name.len() > 0 {
                path.push(&self.name);
                return Some(path);
            } else {
                return None;
            }
        }

        None
    }

}

pub fn handle_subcommand(matches: &ArgMatches) -> Result<(), ZkError> {
    match matches.subcommand() {
        Some(("ls", _ssub_m)) => {
            list_boxes()?;
        },
        Some(("add", ssub_m)) => {
            create_box(ssub_m)?;
        },
        Some(("rm", ssub_m)) => {
            remove_tracking(ssub_m)?;
        },
        Some(("use", ssub_m)) => {
            use_box(ssub_m)?;
        },
        _ => {
            println!("No subcommand found.");
        }
    }
    Ok(())
}


fn create_box(matches: &ArgMatches) -> Result<(), ZkError> {

    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(name)?;

        match fs::exists(&path) {
            Ok(true) => Err(ZkError::BoxExists),
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => {
                        git_init(&path)?;
                        track(&path)?;
                        create_index(&path)?;
                        println!("Created box successfully");
                        Ok(())
                    },
                    Err(_) => Err(ZkError::BoxCreateFail)
                }
            },
            Err(_) => return Err(ZkError::Access(path))
        }
    } else {
        Err(ZkError::NoName)
    }
}

fn list_boxes() -> Result<(), ZkError> {
    let zk_dir = utils::get_zk_dir()?;
    match fs::read_dir(&zk_dir) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(e) => {
                        let path = e.path();
                        if path.is_dir() {
                            println!("{}", path.display());
                        }
                    },
                    Err(_) => {
                        return Err(ZkError::Other(String::from("error")));
                    }
                }
            }
            Ok(())
        },
        Err(_) => {
            Err(ZkError::Other(String::from("error")))
        }
    }
        
}

fn use_box(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(&name)?;
        match fs::exists(&path) {
            Ok(true) => {
                let mut current = utils::get_zk_dir()?;
                current.push(".current");
                match fs::write(current, name) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ZkError::Current)
                }
            },
            _ => {
                Err(ZkError::InvalidBox)
            }
        }
    } else {
        Err(ZkError::NoName)
    }
}

fn git_init(path: &PathBuf) -> Result<(), ZkError> {
    utils::verify_path(path)?;
    match Command::new("git")
        .arg("init")
        .arg(path)
        .output() {
            Ok(_) => Ok(()),
            Err(_) => Err(ZkError::GitInit)
    }
}

fn track(path: &PathBuf) -> Result<(), ZkError> {
    match utils::verify_path(path) {
        Ok(_) => {
            let mut track_file = PathBuf::from(path);
            track_file.push(".track");
            match fs::File::create(track_file) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZkError::TrackFail)
            }
        },
        Err(e) => Err(e.into())
    }
}

fn remove_tracking(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(name)?;
        remove_track_file(&path);
        return Ok(());
    }
    Err(ZkError::NoName)
        
}

fn remove_track_file(path: &PathBuf) {
    match is_tracked(path) {
        Ok(true) => {
            let mut track_file = PathBuf::from(path);
            track_file.push(".track");
            fs::remove_file(track_file);
        },
        _ => {}
    };
}

fn is_tracked(path: &PathBuf) -> Result<bool, ZkError> {
    utils::verify_path(path)?;
    let mut track_file = PathBuf::from(path);
    track_file.push(".track");
    match fs::exists(&track_file) {
        Ok(e) => Ok(e),
        Err(_) => Err(ZkError::Access(track_file))
    } 
}

fn create_index(path: &PathBuf) -> Result<(), ZkError> {
    match utils::verify_path(path) {
        Ok(_) => {
            let mut index_file = PathBuf::from(path);
            index_file.push(".index");
            match fs::File::create(index_file) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZkError::TrackFail)
            }
        },
        Err(e) => Err(e.into())
    }
}

