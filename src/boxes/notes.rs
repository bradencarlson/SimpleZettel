use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{BufRead,BufReader};
use std::path::PathBuf;
use std::process::Command;
use regex::Regex;

use crate::utils;
use crate::error::ZkError;
use crate::config::{ZkCmd,ZkPrefixes};

use crate::boxes::ft::{self,FileType};

#[derive(PartialEq,Debug,Clone)]
pub enum ZkNumber {
    Num(Vec::<usize>),
    Alpha(String),
    Invalid
}

#[derive(Debug,Clone)]
pub struct ZkMatch {
    pub card: ZkCard,
    pub matches: Vec::<ZkLine>
}

#[derive(Debug,PartialEq,Clone)]
pub struct ZkLine {
    pub lineno: usize,
    pub content: String
}

#[derive(Debug,Clone)]
pub struct ZkCard {
    path: PathBuf,
    number: ZkNumber,
    header: String,
    filetype: FileType,
    references: Vec::<ZkRef>,
}

#[derive(Debug,Clone)]
pub enum ZkPath {
    Path(PathBuf),
    Invalid
}

#[derive(Debug,Clone)]
pub struct ZkRef {
    path: ZkPath,
    label: String,
}

impl std::fmt::Display for ZkMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let _ = write!(f, "{}\n", self.card);
        for line in &self.matches {
            let _ = write!(f, "{}: {}\n", line.lineno, line.content);
        }
        Ok(())
    }
}

impl From<PathBuf> for ZkCard {
    fn from(path: PathBuf) -> Self {
        let number = ZkCard::get_number(&path);
        let header = match ZkCard::get_first_header(&path) {
            Ok(s) => s,
            Err(_e) => String::from("No valid header")
        };
        let filetype = match ft::get_filetype(&path) {
            Ok(f) => f,
            Err(_e) => FileType::Markdown
        };
        ZkCard {
            path: path,
            number: number,
            header: header,
            filetype: filetype,
            references: Vec::<ZkRef>::new()
        }
    }
}

impl ZkCard {
    pub fn get_number_length(&self) -> usize {
        match self.number {
            ZkNumber::Num(ref v) => 2*v.len() - 1,
            ZkNumber::Alpha(ref s) => s.len(),
            ZkNumber::Invalid => 0
        }
    }
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_filetype(&self) -> &FileType {
        &self.filetype
    }
    pub fn get_header(&self) -> &String {
        &self.header
    }
    pub fn pretty_print(&self, space: usize, prefix: Option<&ZkPrefixes>, style: Option<anstyle::Style>) {
        let pre = ft::get_prefix(&self.filetype, prefix);
        match style {
            Some(sty) => {
                println!("{}{sty}{}{sty:#}{}", pre, self.format_number(space), self.header);
            },
            None => {
                println!("{}{}{}", pre, self.format_number(space), self.header);
            }
        };
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
    pub fn edit(&self, editor: &ZkCmd) -> Result<(), ZkError> {
        let mut hashes = utils::HashPair::new();
        hashes.push_path(&self.path)?;
        let cmd = match editor {
            ZkCmd::Cmd(edit_cmd) => edit_cmd,
            _ => "vim"
        };
        match Command::new(cmd)
            .arg(&self.path)
            .status() {
                Ok(status) => {
                    if status.success() {
                        hashes.push_path(&self.path)?;
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

    pub fn get_number(path: &PathBuf) -> ZkNumber {
        if let Some(f) = path.file_stem() {
            if let Some(fname) = f.to_str() {
                if let Ok(v) = ZkCard::parse_number(fname) {
                    return ZkNumber::Num(v);
                } else {
                    return ZkNumber::Alpha(fname.to_string());
                }
            } else {
                return ZkNumber::Invalid;
            }
        } else {
            return ZkNumber::Invalid;
        }
    }

    pub fn get_first_header(path: &PathBuf) -> Result<String, ZkError> {
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

    pub fn get_content(&self) -> String {
        if let Ok(s) = fs::read_to_string(&self.path) {
            s
        } else {
            String::new()
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
}

impl std::fmt::Display for ZkCard {
    fn fmt(&self, f: &mut std::fmt::Formatter ) -> Result<(), std::fmt::Error> {
        let blue = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())).bold();
        let red = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into())).bold();
        match self.number {
            ZkNumber::Num(ref v) => write!(f, "{blue}{}{blue:#} {}", self.format_number(0), self.header),
            ZkNumber::Alpha(ref s) => write!(f, "{blue}{}{blue:#} {}", s, self.header),
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

/*fn get_references(path: &PathBuf) -> Result<Vec::<ZkRef>, ZkError> {
    let mut refs = Vec::<ZkRef>::new();
    if let Ok(f) = File::open(path) {
        let reader = BufReader::new(f);
        let mut iter = reader.lines();
        let reference = Regex::new(r"\[(?<linkname>[^\]]+)\]\((?<link>[^\)]+)\)").unwrap();
        while let Some(line_result) = iter.next() {
            if let Ok(line) = line_result {
                if reference.is_match(&line) {
                    let mut matches = reference.captures_iter(&line);
                    while let Some(f_name) = matches.next() {
                        /*let p: ZkPath = match utils::note_path_from_name(&f_name["link"], None, None) {
                            Ok(pth) => ZkPath::Path(pth),
                            Err(_) => ZkPath::Invalid
                        };
                        let zkp = ZkRef {
                            path: p,
                            label: String::from(&f_name["linkname"]),
                        };
                        refs.push(zkp);*/
                    }
                }
            }
        }
    } else {
        return Err(ZkError::Access(path.to_path_buf()))
    }
    Ok(refs)
}*/

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

#[test]
fn card_properties() {
    let card: ZkCard = PathBuf::from("tests/files/1.md").into();
    assert_eq!( card.get_path(), &PathBuf::from("tests/files/1.md"));
    assert_eq!( card.get_filetype(), &FileType::Markdown);
    assert_eq!( card.get_number_length(), 1);

    let header = card.get_header();
    let expected_header = String::from("File 1");
    assert_eq!( header, &expected_header);

}
