use std::fs::{OpenOptions, File, remove_file as rm_file};
use std::io::{Error, Seek, SeekFrom, Write};


pub struct FBin {
    path: String,
    handle: File,
    offset: u64,
}

impl FBin {

    fn get_handle(path: &str, read: bool, write: bool, create: bool, new: bool) -> Result<File, Error>{
        OpenOptions::new().read(read).write(write).create(create).create_new(new).open(path)
    }

    fn get_instance(path: &str, read: bool, write: bool, create: bool, new: bool) -> Result<Self, Error> {
        Self::get_handle(path,  read, write, create, new)
        .map(|file| Self {path: path.to_string(), handle: file, offset: 0 })
    }

    pub fn open(path: &str) -> Result<Self, Error> { 
        Self::get_instance(path, true, true, false, false)
    }

    pub fn open_or_create(path: &str) -> Result<Self, Error> { 
        Self::get_instance(path, true, true, true, false)
    }

    pub fn create(path: &str) -> Result<Self, Error> { 
        Self::get_instance(path, true, true, false, true)
    }

    pub fn file_exists(path: &str) -> bool {
        Self::open(path).is_ok()
    }

    pub fn remove(self) -> Result<(), Error>{
        rm_file(self.path)
    }
    
    pub fn remove_file(path: &str) -> Result<(), Error>{
        rm_file(path)
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