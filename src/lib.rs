use std::fs::{OpenOptions, File};
use std::io::{Error, Seek, SeekFrom, Write};


pub struct FBin {
    handle: File,
    offset: u64,
}

impl FBin {

    fn def_option_generator() -> OpenOptions{
        OpenOptions::new().read(true).write(true).to_owned()
    }

    pub fn open(path: &str) -> Result<Self, Error> { 
        Ok(Self { handle:  Self::def_option_generator().open(path)?, offset: 0 })
    }
    pub fn open_or_create(path: &str) -> Result<Self, Error> { 
        Ok(Self { handle:   Self::def_option_generator().create(true).open(path)?, offset: 0 })
    }
    pub fn create(path: &str) -> Result<Self, Error> { 
        Ok(Self { handle:   Self::def_option_generator().create_new(true).open(path)?, offset: 0 })
    }

    pub fn seek(&mut self, to: u64) -> Result<&mut Self, Error>{
        self.offset = self.handle.seek(SeekFrom::Start(to))?;
        Ok(self)
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<&mut Self, Error> {
        self.handle.write_all(bytes)?;
        self.offset += bytes.len() as u64;
        Ok(self)
    }

}