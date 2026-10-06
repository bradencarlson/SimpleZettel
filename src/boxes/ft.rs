use std::path::{Path};
use std::fs::File;
use std::fs;
use std::io::Read;
use std::process::Command;

use crate::error::ZkError;
use crate::boxes::notes::ZkCard;
use crate::config::{ZkCmd,ZkShowCommands,ZkPrefixes};

#[derive(Debug,Clone,PartialEq)]
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
            let _n = f.read(&mut buff)?;
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

pub fn show_note(card: &ZkCard, show_cmds: &ZkShowCommands) -> Result<(), ZkError> {
    if !cfg!(feature = "filetypes") {
        print_note(card)?;
        return Ok(());
    }
    match card.get_filetype() {
        FileType::Markdown => {
            run_cmd(&card, &show_cmds.md)?;
        },
        FileType::PDF => {
            match show_cmds.pdf {
                ZkCmd::Cmd(ref cmd) => {
                    run_cmd(&card, &ZkCmd::Cmd(cmd.to_string()))?;
                    return Ok(());
                },
                ZkCmd::Invalid => {
                    println!("No default command for showing pdf documents.");
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

pub fn get_prefix(typ: &FileType, prefix: Option<&ZkPrefixes>) -> String {
    if cfg!(feature = "filetypes") {
        match prefix {
            Some(p) => {
                match typ {
                    FileType::Markdown => p.md.clone(),
                    FileType::PDF => p.pdf.clone(),
                }
            },
            None => {
                match typ {
                    FileType::Markdown => String::from("   "),
                    FileType::PDF => String::from("(d)"),
                }
            }
        }
    } else {
        String::new()
    }
}

fn run_cmd(card: &ZkCard, cmd: &ZkCmd) -> Result<(), ZkError> {
    match cmd {
        ZkCmd::Cmd(cmd) => {
            match Command::new(cmd)
                .arg(card.get_path())
                .status() {
                    Ok(_) => {
                        return Ok(());
                    },
                    Err(_e) => {
                        return Err(ZkError::Other(String::from("failed to show note")));
                    }
            };
        }, 
        ZkCmd::Invalid => {
            print_note(card)?;
        }
    };
    Ok(())
}

fn print_note(card: &ZkCard) -> Result<(), ZkError> {
    if let Ok(s) = fs::read_to_string(card.get_path()) {
        println!("{}", s);
    } else {
        return Err(ZkError::NoteRead(card.get_path().clone()));
    }
    Ok(())
}
