use std::fs::{OpenOptions, File};
use std::io::{Error, Seek, SeekFrom, Write};


pub struct FBin {
    handle: File,
    offset: u64,

}

impl FBin {

    pub fn open(path: &str) -> Self {
        let handle = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        Self { handle, offset: 0 }
    }

    pub fn seek(&mut self, to: u64) -> Result<(), Error> {

        match self.handle.seek(SeekFrom::Start(to)) {
            Ok(pos) => {
                self.offset = pos;
                Ok(())
            },
            Err(e) => Err(e),
        }

    }

    pub fn write(&mut self, bytes: &[u8]) {
        let _ = self.handle.write_all(bytes);
        self.offset += bytes.len() as u64;
    }

}