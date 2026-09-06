use std::env;
use std::fs;
use std::io;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::assert_matches;
use std::path::{Path, PathBuf};

use crate::error::ZkError;
use crate::notes::ft::FileType;

#[derive(Debug)]
pub struct HashPair {
    one: Option<u64>,
    two: Option<u64>,
}

impl HashPair {
    pub fn new() -> Self {
        HashPair {
            one: None,
            two: None
        }
    }

    pub fn get_first(&self) -> &Option<u64> {
        &self.one
    }
    pub fn get_second(&self) -> &Option<u64> {
        &self.two
    }

    pub fn equal(&self) -> bool {
        match self.one {
            Some(ref o) => {
                match self.two {
                    Some(ref t) => {
                        o == t
                    },
                    None => false
                }
            },
            None => false
        }
    }

    pub fn push<T: Hash>(&mut self, t: &T) {
        let mut s = DefaultHasher::new();
        match self.one {
            Some(h) => {
                self.two = Some(h);
                t.hash(&mut s);
                self.one = Some(s.finish());
            },
            None => {
                t.hash(&mut s);
                self.one = Some(s.finish());
            }
        }
    }

    pub fn push_path(&mut self, path: &Path) -> Result<(), ZkError> {
        match fs::read_to_string(path) {
            Ok(s) => self.push(&s),
            Err(_) => { return Err(ZkError::NoteRead(path.to_path_buf()))}
        };
        Ok(())
    }

    pub fn len(&self) -> usize {
        match self.two {
            Some(_h) => 2,
            None => {
                match self.one {
                    Some(_v) => 1,
                    None => 0
                }
            }
        }
    }
}

pub fn print_blue(msg: &str) {
    let blue = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())).bold();
    print!("{blue}{}{blue:#}", msg);
}

pub fn prompt_user(msg: &str) -> Result<String, ZkError> {
    println!("{}", msg);
    let mut buff = String::new();
    if let Ok(resp) = io::stdin().read_line(&mut buff) {
        buff.pop();
        Ok(buff)
    } else {
        Err(ZkError::Other(String::from("failed to get response from user")))
    }
}

pub fn path_from_name(name: &str) -> Result<PathBuf, ZkError> {
    let mut root = get_zk_dir()?;
    root.push(name);
    match verify_path(&root) {
        Ok(_) => Ok(root),
        Err(e) => Err(e)
    }
}

pub fn note_path_from_name(name: &str, filetype: Option<FileType>) -> Result<PathBuf, ZkError> {
    let extension: String = match filetype {
        Some(ft) => ft.get_ext(),
        None => String::from("md")
    };
    let mut file = get_current_box()?;
    file.push(name);
    if let Some(ext) = file.extension() {
        if *ext == *extension {
            return Ok(file);
        }
    }
    file.add_extension(extension);
    Ok(file)
}

pub fn get_zk_dir() -> Result<PathBuf, ZkError> {
    if let Some(mut path) = env::home_dir() {
        path.push(".zk/");
        match fs::exists(&path) {
            Ok(true) => Ok(path),
            Ok(false) => {
                match fs::create_dir(&path) {
                    Ok(_) => Ok(path),
                    Err(_) => Err(ZkError::HomeCreate)
                }
            },
            Err(_) => Err(ZkError::HomeCreate)
        }
    } else {
        Err(ZkError::NoHome)
    }

}

pub fn verify_path(path: &PathBuf) -> Result<(), ZkError> {
    let zk_home = get_zk_dir()?;
    if let Some(parent) = path.parent() {
        if parent == zk_home {
            Ok(())
        } else {
            Err(ZkError::InvalidPath)
        }
    } else {
        Err(ZkError::InvalidPath)
    }

}

pub fn verify_note_path(path: &PathBuf) -> Result<(), ZkError> {
    let zk_home = get_zk_dir()?;
    match path.starts_with(zk_home) {
        true => Ok(()),
        false => Err(ZkError::InvalidNotePath(path.clone()))
    }
}

pub fn get_current_box() -> Result<PathBuf, ZkError> {
    let mut current = get_zk_dir()?;
    current.push(".current");
    match fs::read_to_string(&current) {
        Ok(content) => {
            let current = path_from_name(&content.trim())?;
            match current.try_exists() {
                Ok(_) => {
                    Ok(current)
                },
                Err(e) => {
                    Err(ZkError::Other(String::from("I couldn't read the ~/.zk/.current file. I might need you to delete it for me.")))
                }
            }
        },
        Err(e) => Err(ZkError::NoCurrentBox)
    }
}

pub fn note_exists(note: &Path) -> Result<bool, ZkError> {
    match fs::exists(&note)? {
        true => {Ok(true)},
        false => {
            return Err(ZkError::NoteNotExists);
        }
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
    assert_eq!(verify_path(&invalid_path), Err(ZkError::InvalidPath));
}

#[test]
fn deep_path() {
    if let Some(mut invalid_path) = env::home_dir() {
        invalid_path.push(".zk/test/one");
        assert_eq!(verify_path(&invalid_path), Err(ZkError::InvalidPath));
    }
}

#[test]
fn hashpair() {
    let mut hp = HashPair::new();
    let x = 506;
    hp.push(&x);
    assert_matches!(hp.get_first(), &Some(_));
    assert_eq!(hp.len(), 1);
    let y = 102;
    hp.push(&y);
    assert_eq!(hp.len(), 2);
    assert_matches!(hp.get_second(), &Some(_));

}
