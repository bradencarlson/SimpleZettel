use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::ArgMatches;

use crate::utils;
use crate::error::ZkError;
use crate::error;
use crate::vcs;

#[derive(Debug,Default)]
pub struct ZkBox {
    path: PathBuf,
    tracked: bool,
}

impl ZkBox {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_current(&self) -> bool {
        false
    }

    pub fn is_tracked(&self) -> &bool {
        &self.tracked
    }

    pub fn name(&self) -> String {
        match self.path.file_name() {
            Some(os) => {
                let name = match os.to_str() {
                    Some(s) => s,
                    None => ""
                };
                String::from(name)
            },
            None => {
                String::new()
            }
        }
    }

    pub fn pretty_print(&self,prefix: &str, style: Option<anstyle::Style>) {
        match style {
            Some(s) => {
                println!("{s}{}{s:#}{}", prefix, self.name());
            },
            None => {
                println!("{}{}", prefix, self.name());
            }
        };
    }
}

impl From<PathBuf> for ZkBox {
    fn from(path: PathBuf) -> Self {
        match utils::verify_path(&path) {
            Ok(_) => {},
            Err(e) => {
                return ZkBox::new();
            }
        };
        let mut tracked = path.clone();
        tracked.push(".track");
        let track = match fs::exists(tracked) {
            Ok(true) => true, 
            _ => false
        };
        ZkBox {
            path: path, 
            tracked: track,
        }
    }
}

impl std::fmt::Display for ZkBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self);
        Ok(())
    }
}

pub fn get_boxes() -> Vec::<ZkBox> {
    let home = match utils::get_szettel_dir() {
        Ok(p) => p,
        Err(_) => {
            return Vec::<ZkBox>::new();
        }
    };
    let mut vec = Vec::<ZkBox>::new();
    match fs::read_dir(&home) {
        Ok(iter) => {
            for item in iter {
                match item {
                    Ok(entry) => {
                        if !entry.path().is_dir() {
                            continue;
                        }
                        vec.push(entry.path().into());
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        },
        Err(_) => {
            return Vec::<ZkBox>::new();
        }
    }
    vec

}

pub fn add_box(name: &String) -> Result<(), ZkError> {
    let path = utils::path_from_name(name)?;

    match fs::exists(&path) {
        Ok(true) => Err(ZkError::BoxExists),
        Ok(false) => {
            match fs::create_dir(&path) {
                Ok(_) => {
                    vcs::init(&path)?;
                    track(&path)?;
                    println!("Created box successfully");
                    Ok(())
                },
                Err(_) => Err(ZkError::BoxCreateFail)
            }
        },
        Err(_) => return Err(ZkError::Access(path))
    }
}

pub fn list_boxes(bxs: Vec::<ZkBox>, all: bool, color: &u8) -> Result<(), ZkError> {
    let style = anstyle::Style::new().fg_color(Some(anstyle::Ansi256Color::from(*color).into())).bold();
    let current = utils::get_current_box()?;
    for b in bxs.iter() {
        let prefix = match *b.get_path() == current {
            true => "->",
            false => "  ",
        };
        b.pretty_print(&prefix, Some(style));
    }
    Ok(())
}

pub fn use_box(matches: &ArgMatches, color: &u8) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(&name)?;
        if is_tracked(&path)? == false {
            return Err(ZkError::BoxNotTracked);
        };
        match fs::exists(&path) {
            Ok(true) => {
                let mut current = utils::get_szettel_dir()?;
                current.push(".current");
                match fs::write(current, name) {
                    Ok(_) => {
                        let b = get_boxes();
                        list_boxes(b, false, color)?;
                        Ok(())
                    },
                    Err(_e) => Err(ZkError::Current)
                }
            },
            _ => {
                Err(ZkError::BoxInvalid)
            }
        }
    } else {
        Err(ZkError::NoName)
    }
}

fn track_box(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(&name)?;
        track(&path)?;
        Ok(())
    } else {
        Err(ZkError::NoName)
    }
}


fn track(path: &PathBuf) -> Result<(), ZkError> {
    utils::verify_path(path)?;
    let mut track_file = PathBuf::from(path);
    track_file.push(".track");
    match fs::exists(&track_file) {
        Ok(true) => {
            println!("box is already tracked");
            return Ok(());
        },
        _ => {}
    };
    match fs::File::create(track_file) {
        Ok(_) => {
            println!("box successfully tracked");
            Ok(())
        },
        Err(_) => Err(ZkError::TrackFail)
    }
}

fn remove_tracking(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let path = utils::path_from_name(name)?;
        remove_track_file(&path)?;
        Ok(())
    } else {
        Err(ZkError::NoName)
    }
}

fn remove_track_file(path: &PathBuf) -> Result<(), ZkError> {
    match is_tracked(path) {
        Ok(true) => {
            let mut track_file = PathBuf::from(path);
            track_file.push(".track");
            match fs::remove_file(track_file) {
                Ok(_) => {
                    println!("Successfully removed box");
                    Ok(())
                },
                Err(_) => {
                    Err(ZkError::BoxRemove)
                }
            }
        },
        _ => {
            Err(ZkError::BoxNotTracked)
        }
    }
}

fn is_tracked(path: &PathBuf) -> Result<bool, ZkError> {
    utils::verify_path(path)?;
    let mut track_file = PathBuf::from(path);
    match fs::exists(&track_file) {
        Ok(true) => {},
        Ok(false) => {
            return Err(ZkError::BoxInvalid)
        },
        Err(_) => {
            return Err(ZkError::BoxCheck)
        }
    };
    track_file.push(".track");
    match fs::exists(&track_file) {
        Ok(e) => Ok(e),
        Err(_) => Err(ZkError::Access(track_file))
    }
}
