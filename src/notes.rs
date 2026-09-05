use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::PathBuf;
use std::process::Command;
use clap::ArgMatches;
use regex::Regex;

use crate::error::ZkError;
use crate::utils;
use crate::config::{ZkConfig,ZkCmd};

#[derive(PartialEq,Debug)]
pub enum ZkNumber {
    Num(Vec::<usize>),
    Alpha(String),
    Invalid
}

#[derive(Debug)]
pub struct ZkCard {
    path: PathBuf,
    number: ZkNumber,
    header: String,
    kind: ZkType,
    references: Vec::<ZkRef>,
}

#[derive(Debug)]
pub struct ZkType {
    filetype: FileType
}

#[derive(Debug)]
pub enum FileType {
    Markdown,
}

#[derive(Debug)]
pub enum ZkPath {
    Path(PathBuf),
    Invalid
}

#[derive(Debug)]
pub struct ZkRef {
    path: ZkPath,
    label: String,
}

impl ZkCard {
    pub fn from(path: PathBuf) -> Self {
        if let Ok(true) = fs::exists(&path) {
            let header = match get_first_header(&path) {
                Ok(s) => s,
                Err(_) => String::from("no valid header found")
            };
            if let Some(filename) = path.file_stem() {
                let f = match filename.to_str() {
                    Some(s) => s,
                    None => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Invalid,
                            header: header,
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    }
                };
                let name = String::from(f);
                match parse_number(&f) {
                    Ok(v) => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Num(v),
                            header: header,
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    },
                    Err(_) => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Alpha(name),
                            header: header,
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    }
                }
            } else {
                ZkCard{
                    path: path,
                    number: ZkNumber::Invalid,
                    header: header,
                    kind: ZkType { filetype: FileType::Markdown },
                    references: Vec::<ZkRef>::new(),
                }
            }
        } else {
            if let Some(filename) = path.file_stem() {
                let f = match filename.to_str() {
                    Some(s) => s,
                    None => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Invalid,
                            header: String::from(""),
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    }
                };
                let name = String::from(f);
                match parse_number(&f) {
                    Ok(v) => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Num(v),
                            header: String::from(""),
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    },
                    Err(_) => {
                        return ZkCard{
                            path: path,
                            number: ZkNumber::Alpha(name),
                            header: String::from(""),
                            kind: ZkType { filetype: FileType::Markdown },
                            references: Vec::<ZkRef>::new(),
                        };
                    }
                }
            } else {
                ZkCard{
                    path: path,
                    number: ZkNumber::Invalid,
                    header: String::from(""),
                    kind: ZkType { filetype: FileType::Markdown },
                    references: Vec::<ZkRef>::new(),
                }
            }
        }
    }
    pub fn get_number_length(&self) -> usize {
        match self.number {
            ZkNumber::Num(ref v) => 2*v.len() - 1,
            ZkNumber::Alpha(ref s) => s.len(),
            ZkNumber::Invalid => 0
        }
    }
    pub fn format_number(&self, space: usize) -> String {
        let mut s = String::new();
        match self.number {
            ZkNumber::Num(ref v) => {
                for n in v {
                    s.push_str(n.to_string().as_str());
                    s.push_str(".");
                }
                s.pop();
                while s.len() < space {
                    s.push(' ');
                }
                s
            },
            ZkNumber::Alpha(ref s1) => {
                s.push_str(&s1);
                while s.len() < space {
                    s.push(' ');
                }
                s
            },
            ZkNumber::Invalid => s
        }
    }
}

impl std::fmt::Display for ZkCard {
    fn fmt(&self, f: &mut std::fmt::Formatter ) -> Result<(), std::fmt::Error> {
        let blue = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())).bold();
        let red = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into())).bold();
        match self.number {
            ZkNumber::Num(ref v) => write!(f, "{blue}{:?}{blue:#}\t{}", v, self.header),
            ZkNumber::Alpha(ref s) => write!(f, "{blue}{:?}{blue:#}\t{}", s, self.header),
            ZkNumber::Invalid => write!(f, "{red}{}{red:#}", self.path.display())
        }
    }
}

impl std::cmp::PartialOrd for ZkCard {
    fn partial_cmp(&self, other: &ZkCard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for ZkCard {
    fn eq(&self, other: &ZkCard) -> bool {
        self.path == other.path &&
            self.number == other.number
    }
}

impl std::cmp::Eq for ZkCard {}

impl std::cmp::Ord for ZkCard {
    fn cmp(&self, other: &ZkCard) -> Ordering {
        match self.number {
            ZkNumber::Num(ref v) => {
                match other.number {
                    ZkNumber::Num(ref v_rhs) => {
                        return v.cmp(&v_rhs);
                    },
                    ZkNumber::Alpha(_) => {
                        return Ordering::Less;
                    },
                    ZkNumber::Invalid => {
                        return Ordering::Less;
                    }
                };
            },
            ZkNumber::Alpha(ref s) => {
                match other.number {
                    ZkNumber::Num(_) => {
                        return Ordering::Greater;
                    },
                    ZkNumber::Alpha(ref s_rhs) => {
                        return s.cmp(&s_rhs);
                    },
                    ZkNumber::Invalid => {
                        return Ordering::Less;
                    }
                };
            },
            ZkNumber::Invalid => {
                match other.number {
                    ZkNumber::Num(_) => {
                        return Ordering::Greater;
                    },
                    ZkNumber::Alpha(_) => {
                        return Ordering::Greater;
                    },
                    ZkNumber::Invalid => {
                        return Ordering::Equal;
                    }
                };
            }

        };
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
        parse_number(&num)?;
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

pub fn show_note(matches: &ArgMatches, config: &ZkConfig) -> Result<(), ZkError> {
    if let Some(name) = matches.get_one::<String>("name") {
        let note = utils::note_path_from_name(name)?;
        match fs::exists(&note) {
            Ok(true) => {},
            Ok(false) => {
                return Err(ZkError::NoteNotExists);
            }
            _ => {}
        };
        match config.commands.show {
            ZkCmd::cmd(ref cmd) => {
                Command::new(cmd)
                    .arg(&note)
                    .status();
            },
            ZkCmd::Invalid => {
                if let Ok(content) = fs::read_to_string(&note) {
                    print!("{}", content);
                    return Ok(());
                } else {
                    return Err(ZkError::NoteRead(note));
                }
            }
        };
        Ok(())
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

pub fn list_notes(m: Option<&ArgMatches>) -> Result<(), ZkError> {
    match m {
        Some(matches) => {
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
        },
        None => {
            let r = Regex::new("").unwrap();
            list_files(&r)?;
            Ok(())
        }
    }
}

fn list_files(pat: &Regex) -> Result<(), ZkError> {
    let current = utils::get_current_box()?;
    let hidden = Regex::new(r"^\.").unwrap();
    let mut files = Vec::<ZkCard>::new();
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
                files.push(ZkCard::from(path));
            }
        }
    }
    files.sort();

    let max = match files.iter()
        .map(|c| c.get_number_length())
        .max() {
            Some(m) => m+4,
            None => 20
    };
    let blue = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())).bold();
    for file in files.iter() {
        println!("{blue}{}{blue:#}{}", file.format_number(max), file.header);
    }
    Ok(())
}


fn edit_file(path: &PathBuf) -> Result<(), ZkError> {
    utils::verify_note_path(path)?;
    let mut hashes = utils::HashPair::new();
    hashes.push_path(&path)?;
    match Command::new("vim")
        .arg(&path)
        .status() {
            Ok(status) => {
                if status.success() {
                    hashes.push_path(&path)?;
                    if hashes.equal() {
                        return Ok(())
                    }
                    git_add()?;
                    git_commit()?;
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
        let reader = BufReader::new(f);
        let header = Regex::new("^[[:space:]]*#[[:space:]]*(?<label>.*)").unwrap();
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

fn get_references(path: &PathBuf) -> Result<Vec::<ZkRef>, ZkError> {
    let mut refs = Vec::<ZkRef>::new();
    // TODO: Perhaps this method should just assume that someone else has checked this?
    //utils::verify_note_path(&path)?;
    if let Ok(f) = File::open(path) {
        let reader = BufReader::new(f);
        let mut iter = reader.lines();
        let reference = Regex::new(r"\[(?<linkname>[^\]]+)\]\((?<link>[^\)]+)\)").unwrap();
        while let Some(line_result) = iter.next() {
            if let Ok(line) = line_result {
                if reference.is_match(&line) {
                    let mut matches = reference.captures_iter(&line);
                    while let Some(f_name) = matches.next() {
                        let p: ZkPath = match utils::note_path_from_name(&f_name["link"]) {
                            Ok(pth) => ZkPath::Path(pth),
                            Err(_) => ZkPath::Invalid
                        };
                        let zkp = ZkRef {
                            path: p,
                            label: String::from(&f_name["linkname"]),
                        };
                        refs.push(zkp);
                    }
                }
            }
        }
    } else {
        return Err(ZkError::Access(path.to_path_buf()))
    }
    Ok(refs)
}

fn git_add() -> Result<(), ZkError> {
    let current = utils::get_current_box()?;
    match Command::new("git")
        .current_dir(&current)
        .arg("add")
        .arg(".")
        .status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(ZkError::GitAdd)
                }
            },
            Err(e) => {
                Err(ZkError::Other(e.to_string()))
            }
    }

}

fn git_commit() -> Result<bool, ZkError> {
    let resp = utils::prompt_user("Would you like to commit changes to git? [Y/n]")?;
    let Y = String::from("Y");
    let y = String::from("y");
    if resp != Y && resp != y {
        return Ok(false);
    }
    let current = utils::get_current_box()?;
    match Command::new("git")
        .current_dir(&current)
        .arg("commit")
        .status() {
            Ok(status) => {
                if status.success() {
                    Ok(true)
                } else {
                    Err(ZkError::GitCommit)
                }
            },
            Err(e) => {
                Err(ZkError::Other(e.to_string()))
            }
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
