use std::fs;
use std::path::PathBuf;

use crate::utils;
use crate::error::ZkError;
use crate::error;
use crate::vcs;

#[derive(Debug,Default,PartialEq)]
pub struct ZkBox {
    path: PathBuf,
    tracked: bool,
    current: bool,
}

impl ZkBox {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_current(&self) -> bool {
        self.current
    }

    pub fn is_tracked(&self) -> bool {
        self.tracked
    }

    fn name(&self) -> String {
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
        /*match utils::verify_path(&path) {
            Ok(_) => {},
            Err(e) => {
                return ZkBox::new();
            }
        };*/
        let mut tracked = path.clone();
        tracked.push(".track");
        let track = match fs::exists(tracked) {
            Ok(true) => true, 
            _ => false
        };
        ZkBox {
            path: path, 
            tracked: track,
            current: false
        }
    }
}

impl std::fmt::Display for ZkBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self);
        Ok(())
    }
}

pub fn get_current_box(base_dir: &PathBuf) -> Result<ZkBox, ZkError> {
    let mut current = base_dir.clone();
    current.push(".current");
    match fs::read_to_string(&current) {
        Ok(c) => {
            let mut box_path = base_dir.clone();
            box_path.push(c);
            Ok(ZkBox::from(box_path))
        }, 
        _ => {
            Err(ZkError::NoCurrentBox)
        }
    }
}

pub fn get_boxes(base_dir: &PathBuf) -> Result<Vec::<ZkBox>, ZkError> {
    let mut vec = Vec::<ZkBox>::new();
    let current_box = match get_current_box(base_dir) {
        Ok(bx) => bx,
        Err(e) => {
            error::warning(&e.to_string());
            ZkBox::new()
        }
    };
    match fs::read_dir(&base_dir) {
        Ok(iter) => {
            for item in iter {
                match item {
                    Ok(entry) => {
                        if !entry.path().is_dir() {
                            continue;
                        }
                        let mut b = ZkBox::from(entry.path());
                        if b.path == current_box.path {
                            b.current = true;
                        }
                        vec.push(b);
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        },
        Err(_) => {
            return Err(ZkError::Access(base_dir.clone()));
        }
    }
    Ok(vec)

}

pub fn add_box(base_dir: &PathBuf, name: &String) -> Result<(), ZkError> {
    let mut path = base_dir.clone();
    path.push(name);
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
        if b.is_tracked() || all {
            b.pretty_print(&prefix, Some(style));
        }
    }
    Ok(())
}

pub fn use_box(name: &String, color: &u8) -> Result<(), ZkError> {
    let b: ZkBox = utils::path_from_name(&name)?.into();
    if !b.is_tracked() {
        return Err(ZkError::BoxNotTracked);
    };
    match fs::exists(&b.get_path()) {
        Ok(true) => {
            let mut current = utils::get_szettel_dir()?;
            current.push(".current");
            match fs::write(current, name) {
                Ok(_) => {
                    /*let b = get_boxes();
                    list_boxes(b, false, color)?;*/
                    Ok(())
                },
                Err(_e) => Err(ZkError::Current)
            }
        },
        _ => {
            Err(ZkError::BoxInvalid)
        }
    }
}

pub fn track_box(name: &String) -> Result<(), ZkError> {
    let path = utils::path_from_name(&name)?;
    track(&path)?;
    Ok(())
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

pub fn remove_tracking(name: &String) -> Result<(), ZkError> {
    let path = utils::path_from_name(name)?;
    remove_track_file(path)?;
    Ok(())
}

fn remove_track_file(path: PathBuf) -> Result<(), ZkError> {
    let b: ZkBox = path.into();
    if b.is_tracked() {
        let mut track_file = PathBuf::from(b.get_path());
        track_file.push(".track");
        match fs::remove_file(track_file) {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err(ZkError::BoxRemove);
            }
        }
    } else {
        return Err(ZkError::BoxNotTracked);
    }

}

#[test]
fn get_list_of_boxes() {
    let base_dir = PathBuf::from("tests");
    let b = get_boxes(&base_dir).expect("Could not get boxes from tests directory");
    assert_eq!(b.len(), 4);
    println!("{:?}", b);
    assert!(b.contains(
            &ZkBox {
                path: PathBuf::from("tests/short"),
                tracked: true,
                current: false,
            })
    );
}

#[test]
fn invalid_box_dir() {
    let base_dir = PathBuf::from("tests/invalid");
    assert_eq!(get_boxes(&base_dir), Err(ZkError::Access(base_dir)));
}
