use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::process::Command;

use crate::error::ZkError;
use crate::notes::ZkCard;
use crate::config::ZkConfig;
use crate::config::ZkCmd;

#[derive(Debug)]
pub enum FileType {
    Markdown,
    PDF,
}

impl FileType {
    pub fn get_ext(&self) -> String {
        match self {
            FileType::Markdown => String::from("md"),
            FileType::PDF => String::from("pdf"),
        }
    }
}

pub fn get_filetype(path: &Path) -> Result<FileType, ZkError> {
    if cfg!(feature = "filetypes") {
        if let Ok(mut f) = File::open(path) {
            let mut buff = [0u8; 10];
            let n = f.read(&mut buff)?;
            if buff[0..5] == [0x25, 0x50, 0x44, 0x46, 0x2D] {
                return Ok(FileType::PDF);
            }
        } else {
            return Err(ZkError::NoteRead(path.to_path_buf()));
        }
        Ok(FileType::Markdown)
    } else {
        Ok(FileType::Markdown)
    }
}

pub fn show_note(card: &ZkCard, config: &ZkConfig) -> Result<(), ZkError> {
    match card.filetype {
        FileType::Markdown => {
            match config.show.md {
                ZkCmd::cmd(ref cmd) => {
                    Command::new(cmd)
                        .arg(&card.path)
                        .status();
                    return Ok(());
                }, 
                ZkCmd::Invalid => {
                    if let Ok(s) = fs::read_to_string(&card.path) {
                        println!("{}", s);
                        return Ok(());
                    } else {
                        return Err(ZkError::NoteRead(card.path.clone()));
                    }
                }
            };
        },
        FileType::PDF => {
            match config.show.pdf {
                ZkCmd::cmd(ref cmd) => {
                    Command::new(cmd)
                        .arg(&card.path)
                        .status();
                    return Ok(());
                },
                ZkCmd::Invalid => {
                    println!("No default command for showing pdf documents.");
                    return Ok(());
                }
            }
        }
    }
}
