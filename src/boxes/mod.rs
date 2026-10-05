use std::fs;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::process::Command;
use std::path::PathBuf;
use regex::Regex;
use crate::config::{ZkPrefixes,ZkShowCommands,ZkCmd};

use crate::utils;
use crate::error::ZkError;
use crate::error;
use crate::vcs;

pub mod notes;
pub mod ft;

use notes::{ZkCard,ZkLine,ZkMatch};

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
    pub fn get_cards(&self, p: Option<&Regex>) -> Result<Vec::<ZkCard>, ZkError> {
        let pat = match p {
            Some(pattern) => pattern, 
            None => &Regex::new("").unwrap()
        };
        let hidden = Regex::new(r"^\.").unwrap();
        let mut files = Vec::<ZkCard>::new();
        if let Ok(iter) = fs::read_dir(&self.path) {
            for entry in iter {
                let e = match entry {
                    Ok(e) => {e},
                    Err(_) => {continue;}
                };
                let path = e.path();
                let filename = match path.file_stem() {
                    Some(f) => {
                        match f.to_str() {
                            Some(s) => s,
                            None => {continue;}
                        }
                    },
                    None => {continue;}
                };
                if hidden.is_match(&filename) {
                    continue;
                }
                if pat.is_match(&filename) {
                    files.push(ZkCard::from(path));
                }
            }
        }
        files.sort();
        Ok(files)
    }

    pub fn list_cards(&self, p: Option<&Regex>, prefix: Option<&ZkPrefixes>, style: Option<anstyle::Style>) -> Result<(), ZkError> {
        let cards = self.get_cards(p)?;
        let max = match cards.iter()
            .map(|c| c.get_number_length())
            .max() {
                Some(m) => m+4,
                None => 20
        };
        for card in cards.iter() {
            card.pretty_print(max, prefix, style);
        }
        Ok(())
    }

    pub fn add_card(&self, name: &String, editor: &ZkCmd) -> Result<(), ZkError> {
        let mut file = self.path.clone();
        file.push(name);
        file.set_extension("md");
        match fs::exists(&file) {
            Ok(true) => {
                return Err(ZkError::NoteExists);
            },
            _ => {}
        };
        match File::create(&file) {
            Ok(_) => {
                edit_file(&file, editor)?;
                vcs::add(&self.path)?;
                vcs::commit(&self.path)?;
                return Ok(());
            },
            Err(_) => {
                return Err(ZkError::NoteCreate);
            }
        }
    }

    pub fn edit_card(&self, name: &String, editor: &ZkCmd) -> Result<(), ZkError> {
        let mut path = self.path.clone();
        path.push(name);
        path.set_extension("md");
        if self.card_exists(name)? {
            edit_file(&path, editor)?;
            vcs::add(&self.path)?;
            vcs::commit(&self.path)?;
            Ok(())
        } else {
            Err(ZkError::NoteNotExists)
        }
    }

    pub fn rm_card(&self, name: &String) -> Result<(), ZkError> {
        let card = self.find_card(name)?;
        match fs::remove_file(&card.path) {
            Ok(()) => {
                println!("succesfully removed note");
                Ok(())
            },
            Err(_e) => {
                Err(ZkError::Other(String::from("could not remove note")))
            }
        }
    }

    pub fn show_card(&self, name: &String, show_cmds: &ZkShowCommands) -> Result<(), ZkError> {
        let card = self.find_card(name)?;
        ft::show_note(&card, show_cmds)?;
        Ok(())
    }
    
    pub fn import_file(&self, path: &PathBuf) -> Result<(), ZkError> {
        match fs::exists(&path) {
            Ok(true) => {},
            Ok(false) => {
                return Err(ZkError::Other(String::from("import path does not exist")));
            },
            Err(_) => {
                return Err(ZkError::Access(path.clone()));
            }
        };
        if let Some(fname) = path.file_name() {
            let mut new_path = self.path.clone();
            new_path.push(fname);
            match fs::copy(path, new_path) {
                Ok(_) => {
                    return Ok(());
                },
                Err(_) => {
                    return Err(ZkError::ImportFail);
                }
            }
        } else {
            return Err(ZkError::Other(String::from("unable to get filename of import path")));
        }
    }

    pub fn move_card(&self, old: &String, new: &String) -> Result<(), ZkError> {
        let card = self.find_card(old)?;
        let mut new_path = self.path.clone();
        new_path.push(new);
        if let Some(ext) = new_path.extension() {
            if *ext != OsString::from(card.filetype.get_ext()) {
                new_path.add_extension(card.filetype.get_ext());
            }
        } else {
            new_path.add_extension(card.filetype.get_ext());
        }
        if let Ok(true) = fs::exists(&new_path) {
                return Err(ZkError::NoteExists);
        }
        match fs::rename(card.path, new_path) {
            Ok(_) => {
                Ok(())
            },
            Err(_) => {
                Err(ZkError::Move)
            }
        }
    }

    pub fn search_cards(&self, needle: &Regex) -> Result<Vec::<ZkMatch>, ZkError> {
        let mut mat = Vec::<ZkMatch>::new();
        let cards = self.get_cards(None)?;
        for card in cards.iter() {
            if let Some(matches) = ZkBox::search(&card, needle) {
                let l = &matches.len();
                if *l > 0  {
                    let match_location = ZkMatch { 
                        card: card.clone(),
                        matches: matches
                    };
                    mat.push(match_location);
                }
            }
        }
        Ok(mat)
    }

    fn search(card: &ZkCard, needle: &Regex) -> Option<Vec::<ZkLine>> {
        let f = match File::open(&card.path) {
            Ok(file) => file,
            Err(_) => return None
        };
        let mut matches = Vec::<ZkLine>::new();
        let reader = BufReader::new(f);
        let mut lines = reader.lines();
        let mut line_no = 0;
        while let Some(Ok(line)) = lines.next() {
            line_no += 1;
            if needle.is_match(line.as_str()) {
                matches.push(ZkLine {
                    lineno: line_no,
                    content: line
                });
            }
        }
        Some(matches)
    }

    fn card_exists(&self, name: &String) -> Result<bool, ZkError> {
        let mut path = self.path.clone();
        path.push(name);
        path.set_extension("md");
        match fs::exists(&path) {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(_e) => Err(ZkError::NoteRead(path))
        }
    }

    fn find_card(&self, name: &String) -> Result<ZkCard, ZkError> {
        let mut path = self.path.clone();
        path.push(name);
        let iter = match fs::read_dir(&self.path) {
            Ok(it) => it, 
            Err(_e) => {
                return Err(ZkError::Access(self.path.clone()));
            }
        };
        for entry in iter {
            let entry = match entry {
                Ok(e) => e, 
                Err(_) => {
                    break;
                }
            };
            let p = entry.path();
            if let Some(fstem) = p.file_name() {
                if let Some(fname) = fstem.to_str() {
                    if fname.starts_with(name) {
                        return Ok(ZkCard::from(p));
                    }
                }
            }
        }
        Err(ZkError::NoteNotExists)
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
        let _ = write!(f, "{:?}", self);
        Ok(())
    }
}

pub fn get_current_box(base_dir: &PathBuf) -> Result<ZkBox, ZkError> {
    let mut current = base_dir.clone();
    current.push(".current");
    match fs::read_to_string(&current) {
        Ok(c) => {
            let mut box_path = base_dir.clone();
            box_path.push(c.trim());
            let mut current_box = ZkBox::from(box_path);
            current_box.current = true;
            Ok(current_box)
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
                    track(base_dir, name)?;
                    println!("Created box successfully");
                    Ok(())
                },
                Err(_) => Err(ZkError::BoxCreateFail)
            }
        },
        Err(_) => return Err(ZkError::Access(path))
    }
}

pub fn list_boxes(base_dir: &PathBuf, all: bool, color: &u8) -> Result<(), ZkError> {
    let style = anstyle::Style::new().fg_color(Some(anstyle::Ansi256Color::from(*color).into())).bold();
    let bxs = get_boxes(base_dir)?;
    for b in bxs.iter() {
        let prefix = match b.is_current() {
            true => "->",
            false => "  ",
        };
        if b.is_tracked() || all {
            b.pretty_print(&prefix, Some(style));
        }
    }
    Ok(())
}

pub fn use_box(base_dir: &PathBuf, name: &String, color: &u8) -> Result<(), ZkError> {
    let mut box_path = base_dir.clone();
    box_path.push(name);
    let b = ZkBox::from(box_path);
    if !b.is_tracked() {
        return Err(ZkError::BoxNotTracked);
    };
    match fs::exists(&b.get_path()) {
        Ok(true) => {
            let mut current = base_dir.clone();
            current.push(".current");
            match fs::write(current, name) {
                Ok(_) => {},
                Err(_e) => {
                    return Err(ZkError::Current);
                }
            };
            list_boxes(base_dir, false, color)?;
            Ok(())
        },
        _ => {
            Err(ZkError::BoxInvalid)
        }
    }
}


pub fn track(base_dir: &PathBuf, name: &String) -> Result<(), ZkError> {
    let mut track_file = PathBuf::from(base_dir);
    track_file.push(name);
    match fs::exists(&track_file) {
        Ok(true) => {},
        Ok(false) => {
            return Err(ZkError::BoxInvalid);
        },
        Err(_e) => {
            return Err(ZkError::BoxCheck);
        }
    };

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

pub fn remove_tracking(base_dir: &PathBuf, name: &String) -> Result<(), ZkError> {
    let mut path = base_dir.clone();
    path.push(name);
    match fs::exists(&path) {
        Ok(true) => {},
        Ok(false) => {
            return Err(ZkError::BoxInvalid);
        },
        Err(_e) => {
            return Err(ZkError::BoxCheck);
        }
    };
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

fn edit_file(path: &PathBuf, editor: &ZkCmd) -> Result<(), ZkError> {
    let mut hashes = utils::HashPair::new();
    hashes.push_path(&path)?;
    let cmd = match editor {
        ZkCmd::Cmd(edit_cmd) => edit_cmd,
        _ => "vim"
    };
    match Command::new(cmd)
        .arg(&path)
        .status() {
            Ok(status) => {
                if status.success() {
                    hashes.push_path(&path)?;
                    if hashes.equal() {
                        return Ok(())
                    }
                    Ok(())
                } else {
                    Err(ZkError::Other(String::from("something went wrong while opening vim for the user")))
                }
            },
            Err(_) => Err(ZkError::Other(String::from("something went wrong while opening vim for the user")))
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

#[test]
fn string_search() {
    let test_file = PathBuf::from("tests/files/1.md");
    let card = ZkCard::from(test_file);
    let pat = Regex::new("File").expect("Failed to create regex.");
    let matches = ZkBox::search(&card, &pat).expect("Failed to run search on card.");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches, vec![
        ZkLine {
            lineno: 5,
            content: String::from("# File 1")
        }]
    );
}

#[test]
fn card_names() {
    let name = String::from("1.md");
    let base_dir = PathBuf::from("tests/files");
    let bx = ZkBox::from(base_dir);
    assert!(bx.card_exists(&name).expect("failed to check for card."));
    
    let name = String::from("invalid");
    assert!(!bx.card_exists(&name).expect("failed to check for card."));

    let name = String::from("2");
    assert!(bx.card_exists(&name).expect("failed to check for card."));

    let name = String::from("new-file");
    assert!(bx.card_exists(&name).expect("failed to check for card."));

}

#[test]
fn finding_cards() {
    let base_dir = PathBuf::from("tests/files");
    let bx = ZkBox::from(base_dir);

    let name = String::from("1");
    bx.find_card(&name).expect("failed to find card 1.md");
    let name = String::from("1.m");
    bx.find_card(&name).expect("failed to find card 1.md");
    let name = String::from("1.md");
    bx.find_card(&name).expect("failed to find card 1.md");
    let name = String::from("new-");
    bx.find_card(&name).expect("failed to find card new-file.md");

    let name = String::from("invalid");
    assert_eq!(bx.find_card(&name), Err(ZkError::NoteNotExists));

}

#[test]
fn getting_current_box() {
    let base_dir = PathBuf::from("tests");
    let current = get_current_box(&base_dir).expect("failed to get current box.");
    let expected = PathBuf::from("tests/files");
    let mut expected_box = ZkBox::from(expected);
    expected_box.current = true;

    assert_eq!(current, expected_box);
}

#[test]
fn getting_boxes() {
    let base_dir = PathBuf::from("tests");
    let bxs = get_boxes(&base_dir).expect("failed to get boxes.");
    assert_eq!(bxs.len(), 4);

    assert!(bxs.contains(
            &ZkBox {
                path: PathBuf::from("tests/files"), 
                tracked: true,
                current: true
            })
    );
    assert!(bxs.contains(
            &ZkBox {
                path: PathBuf::from("tests/short"), 
                tracked: true,
                current: false
            })
    );
    assert!(bxs.contains(
            &ZkBox {
                path: PathBuf::from("tests/bib"), 
                tracked: false,
                current: false
            })
    );

}

#[test]
fn adding_boxes() {
    let base_dir = PathBuf::from("tests");
    let name = String::from("files");
    assert_eq!(add_box(&base_dir, &name), Err(ZkError::BoxExists));
    let name = String::from("bib");
    assert_eq!(add_box(&base_dir, &name), Err(ZkError::BoxExists));
}

#[test]
fn using_boxes() {
    let base_dir = PathBuf::from("tests");
    let name = String::from("bib");
    assert_eq!(use_box(&base_dir, &name, &0u8), Err(ZkError::BoxNotTracked));
}
