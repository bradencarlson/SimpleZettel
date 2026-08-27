use std::cmp::{Ordering,Eq};
use std::fs;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::PathBuf;
use std::process::Command;
use clap::ArgMatches;
use regex::Regex;

use crate::error::ZkError;
use crate::utils;

#[derive(PartialEq)]
pub enum ZkNumber {
    Num(Vec::<usize>),
    Alpha(String),
    Invalid
}

pub struct ZkCard {
    path: PathBuf,
    number: ZkNumber
}

impl ZkCard {
    pub fn from(path: PathBuf) -> Self {
        if let Some(filename) = path.file_stem() {
            let f = match filename.to_str() {
                Some(s) => s,
                None => {
                    return ZkCard{
                        path: path,
                        number: ZkNumber::Invalid
                    };
                }
            };
            let name = String::from(f);
            match parse_number(&f) {
                Ok(v) => {
                    return ZkCard{
                        path: path,
                        number: ZkNumber::Num(v)
                    };
                },
                Err(_) => {
                    return ZkCard{
                        path: path,
                        number: ZkNumber::Alpha(name)
                    };
                }
            }
        } else {
            ZkCard{
                path: path,
                number: ZkNumber::Invalid
            }
        }
    }
}




impl std::cmp::PartialOrd for ZkCard {
    fn partial_cmp(&self, other: &ZkCard) -> Option<Ordering> {
        match self.number {
            ZkNumber::Num(ref v) => {
                match other.number {
                    ZkNumber::Num(ref v_rhs) => {
                        return Some(v.cmp(&v_rhs));
                    },
                    ZkNumber::Alpha(ref s) => {
                        return Some(Ordering::Less);
                    },
                    ZkNumber::Invalid => {
                        return Some(Ordering::Less);
                    }
                };
            },
            ZkNumber::Alpha(ref s) => {
                match other.number {
                    ZkNumber::Num(ref v_rhs) => {
                        return Some(Ordering::Greater);
                    },
                    ZkNumber::Alpha(ref s_rhs) => {
                        return Some(s.cmp(&s_rhs));
                    },
                    ZkNumber::Invalid => {
                        return Some(Ordering::Less);
                    }
                };
            },
            ZkNumber::Invalid => {
                match other.number {
                    ZkNumber::Num(ref v_rhs) => {
                        return Some(Ordering::Greater);
                    },
                    ZkNumber::Alpha(ref s_rhs) => {
                        return Some(Ordering::Greater);
                    },
                    ZkNumber::Invalid => {
                        return Some(Ordering::Equal);
                    }
                };
            }

        };
    }
}

impl std::cmp::PartialEq for ZkCard {
    fn eq(&self, other: &ZkCard) -> bool {
        self.path == other.path &&
            self.number == other.number
    }
}


pub fn add_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let file = utils::note_path_from_name(name)?;
        match fs::exists(&file) {
            Ok(true) => {
                return Err(ZkError::NoteExists);
            },
            _ => {}
        };
        match File::create(&file) {
            Ok(_) => {
                edit_file(&file)?;
                return Ok(());
            },
            Err(_) => {
                return Err(ZkError::NoteCreate);
            }
        }
    }
    if let Some(num) = matches.get_one::<String>("number") {
        let v = parse_number(&num)?;
        let file = utils::note_path_from_name(num)?;
        match fs::exists(&file) {
            Ok(true) => {
                return Err(ZkError::NoteExists);
            },
            _ => {}
        };
        match File::create(&file) {
            Ok(_) => {
                edit_file(&file)?;
                return Ok(());
            },
            Err(_) => {
                return Err(ZkError::NoteCreate);
            }
        }
    }
    Err(ZkError::NoteAddArgs)
}

pub fn show_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let note = utils::note_path_from_name(name)?;
        match fs::exists(&note) {
            Ok(true) => {},
            Ok(false) => {
                return Err(ZkError::NoteNotExists);
            }
            _ => {}
        };
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
                    Err(_e) => {
                        Err(ZkError::Other(String::from("could not remove note")))
                    }
                }
            },
            _ => {
                Err(ZkError::NoteNotExists)
            }
        }
    } else {
        Err(ZkError::NoName)
    }
}

pub fn edit_note(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let note = utils::note_path_from_name(name)?;
        match fs::exists(&note) {
            Ok(true) => {
                edit_file(&note)?;
                return Ok(())
            },
            Ok(false) => {
                return Err(ZkError::NoteNotExists)
            },
            Err(_) => {
                return Err(ZkError::Other(String::from("Could not check existence of note")))
            }
        }
    } else {
        Err(ZkError::NoName)
    }
}

pub fn list_notes(matches: &ArgMatches) -> Result<(), ZkError> {
    if let Some(pat) = matches.get_one::<String>("pattern") {
        let r = match Regex::new(pat) {
            Ok(p) => p,
            Err(e) => {
                return Err(ZkError::Other(e.to_string()));
            }
        };
        list_files(&r)?;
        Ok(())
    } else {
        let r = Regex::new("").unwrap();
        list_files(&r)?;
        Ok(())
    }
}

fn list_files(pat: &Regex) -> Result<(), ZkError> {
    let current = utils::get_current_box()?;
    let hidden = Regex::new(r"^\.").unwrap();
    if let Ok(iter) = fs::read_dir(current) {
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
                utils::print_blue(filename);
                match get_first_header(&path) {
                    Ok(header) => {
                        print!("\t{}\n", header);
                    },
                    Err(_) => {
                        print!("header not found\n");
                    }
                };
            }
        }
    }
    Ok(())
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

fn parse_number(num: &str) -> Result<Vec::<usize>, ZkError> {
    let mut v = Vec::<usize>::new();
    for number in num.split('.') {
        match number.parse::<usize>() {
            Ok(n) => {
                v.push(n);
            },
            Err(_) => {
                return Err(ZkError::NoteNumber);
            }
        };
    }
    Ok(v)
}

fn insert_number(path: &PathBuf, num: &Vec::<usize>) -> Result<(), ZkError> {
    Ok(())
}

fn get_first_header(path: &PathBuf) -> Result<String, ZkError> {
    utils::verify_note_path(&path)?;
    if let Ok(f) = File::open(path) {
        let mut reader = BufReader::new(f);
        let header = Regex::new("^[[:space:]]*#[[:space:]]*(?<label>([a-zA-Z]+[ ]?)+)").unwrap();
        let mut iter = reader.lines();
        while let Some(line_result) = iter.next() {
            if let Ok(line) = line_result {
                if header.is_match(&line) {
                    let mut matches = header.captures_iter(&line);
                    if let Some(h) = matches.next() {
                        return Ok(String::from(&h["label"]));
                    }
                }
            }
        }
        return Err(ZkError::NoteHeader);
    } else {
        Err(ZkError::Access(path.to_path_buf()))
    }
}

#[test]
fn ordering() {
    let c0 = ZkCard::from(PathBuf::from("./1.md"));
    let c1 = ZkCard::from(PathBuf::from("./1.1.1.md"));
    let c2 = ZkCard::from(PathBuf::from("./filename.md"));
    let c3 = ZkCard::from(PathBuf::from("./filename_long.md"));
    let c4 = ZkCard::from(PathBuf::from("./1.10.2.1.md"));
    let c5 = ZkCard::from(PathBuf::from("./1.1.1.4.md"));
    let c6 = ZkCard::from(PathBuf::from("./1.2.1.md"));
    assert!(c0 < c1);
    assert!(c1 < c2);
    assert!(c1 < c3);
    assert!(c1 < c4);
    assert!(c1 < c5);
    assert!(c2 < c3);
    assert!(c5 < c4);
    assert!(c5 < c6);
    assert!(c6 < c4);
}
