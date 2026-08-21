use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::cmp::Ordering;
use clap::ArgMatches;

#[derive(Debug,PartialEq)]
pub enum BoxError {
    CreateFail,
    BoxExists,
    NoName,
    Access,
    NoHome,
    ZkHomeCreate,
    InvalidPath,
    TrackFail,
    IndexFail,
    GitInit,
}

struct ZkBox {
    name: String
}

impl ZkBox {
    pub fn get_path(&self) -> Option<PathBuf> {
        if let Ok(mut path) = get_zk_dir() {
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

impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxError::CreateFail => write!(f, "failed to create box"),
            BoxError::BoxExists => write!(f, "box already exists"),
            BoxError::NoName => write!(f, "you must provide a name for the box"),
            BoxError::Access => write!(f, "failed to access ~/.zk"),
            BoxError::NoHome => write!(f, "failed to find user's home directory"),
            BoxError::ZkHomeCreate => write!(f, "failed to create ~/.zk directory"),
            BoxError::InvalidPath => write!(f, "Something weird has happened; I tried to access a directory outside of ~/.zk"),
            BoxError::TrackFail => write!(f, "Failed to track box"),
            BoxError::IndexFail => write!(f, "Failed to create index file"),
            BoxError::GitInit => write!(f, "failed to initialize git repo for box"),
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
            remove_tracking(ssub_m)?;
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
                    Ok(_) => {
                        git_init(&path)?;
                        track(&path)?;
                        create_index(&path)?;
                        Ok(())
                    },
                    Err(_) => Err(BoxError::CreateFail)
                }
            },
            Err(_) => return Err(BoxError::Access)
        }

    } else {
        Err(BoxError::NoName)
    }
}

fn git_init(path: &PathBuf) -> Result<(), BoxError> {
    verify_path(path)?;
    match Command::new("git")
        .arg("init")
        .arg(path)
        .output() {
            Ok(_) => Ok(()),
            Err(_) => Err(BoxError::GitInit)
    }
}

fn verify_path(path: &PathBuf) -> Result<(), BoxError> {
    let zk_home = get_zk_dir()?;
    if let Some(parent) = path.parent() {
        if parent == zk_home {
            Ok(())
        } else {
            Err(BoxError::InvalidPath)
        }
    } else {
        Err(BoxError::InvalidPath)
    }

}

fn track(path: &PathBuf) -> Result<(), BoxError> {
    match verify_path(path) {
        Ok(_) => {
            let mut track_file = PathBuf::from(path);
            track_file.push(".track");
            match fs::File::create(track_file) {
                Ok(_) => Ok(()),
                Err(_) => Err(BoxError::TrackFail)
            }
        },
        Err(e) => Err(e)
    }
}

fn remove_tracking(matches: &ArgMatches) -> Result<(), BoxError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let mut path = get_zk_dir()?;
        path.push(name);
        verify_path(&path)?;
        remove_track_file(&path);
        return Ok(());
    }
    Err(BoxError::NoName)
        
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

fn is_tracked(path: &PathBuf) -> Result<bool, BoxError> {
    verify_path(path)?;
    let mut track_file = PathBuf::from(path);
    track_file.push(".track");
    match fs::exists(track_file) {
        Ok(e) => Ok(e),
        Err(_) => Err(BoxError::Access)
    } 
}

fn create_index(path: &PathBuf) -> Result<(), BoxError> {
    match verify_path(path) {
        Ok(_) => {
            let mut index_file = PathBuf::from(path);
            index_file.push(".index");
            match fs::File::create(index_file) {
                Ok(_) => Ok(()),
                Err(_) => Err(BoxError::TrackFail)
            }
        },
        Err(e) => Err(e)
    }
}

#[test]
fn valid_path() {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/test");
        assert_eq!(verify_path(&path), Ok(()));
    }

}

#[test]
fn invalid_path() {
    let invalid_path = PathBuf::from("~/Documents/dir");
    assert_eq!(verify_path(&invalid_path), Err(BoxError::InvalidPath));
}

#[test]
fn deep_path() {
    if let Some(mut invalid_path) = env::home_dir() {
        invalid_path.push(".zk/test/one");
        assert_eq!(verify_path(&invalid_path), Err(BoxError::InvalidPath));
    }
}
