use std::env;
use std::fs;
use std::io;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::assert_matches;
use std::path::{Path, PathBuf};

use crate::error::ZkError;

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

pub fn prompt_user(msg: &str) -> Result<String, ZkError> {
    println!("{}", msg);
    let mut buff = String::new();
    if let Ok(_resp) = io::stdin().read_line(&mut buff) {
        buff.pop();
        Ok(buff)
    } else {
        Err(ZkError::Other(String::from("failed to get response from user")))
    }
}

pub fn get_szettel_dir() -> Result<PathBuf, ZkError> {
    if let Some(mut path) = env::home_dir() {
        path.push(".szettel/");
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
