use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::error::ZkError;
use crate::notes::ZkCard;
use crate::config::ZkConfig;

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
}

pub fn show_note(card: &ZkCard, config: &ZkConfig) -> Result<(), ZkError> {
    Ok(())
}
