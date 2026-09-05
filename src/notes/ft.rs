use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::error::ZkError;

#[derive(Debug)]
pub enum FileType {
    Markdown,
    PDF,
}

pub fn get_filetype(path: &Path) -> Result<FileType, ZkError> {
    if let Ok(mut f) = File::open(path) {
        let mut buff = [0; 10];
        let n = f.read(&mut buff)?;
        if buff[0..4] == [0x25, 0x50, 0x44, 0x46, 0x2D] {
            return Ok(FileType::PDF);
        }
    } else {
        return Err(ZkError::NoteRead(path.to_path_buf()));
    }
    Ok(FileType::Markdown)
}
