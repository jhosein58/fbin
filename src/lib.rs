use std::fs::{OpenOptions, File, remove_file};
use std::io::{Error, Seek, SeekFrom, Write};


pub enum FStrategy {
    Open,
    Create,
    OpenOrCreate,
}

impl FStrategy {
    pub fn flags(&self) -> [bool; 4] {
        match self {
            Self::Open => [false; 4],
            Self::Create => [false, false, false, true],
            Self::OpenOrCreate => [false, false, true, false]
        }
    }
}

pub enum FAccess {
    Read,
    Write,
    ReadWrite,
}

impl FAccess {
    pub fn flags(&self) -> [bool; 4] {
        match self {
            Self::Read => [true, false, false, false],
            Self::Write => [false, true, false, false],
            Self::ReadWrite => [true, true, false, false]
        }
    }
}



pub struct FMode(pub FStrategy, pub FAccess);

impl FMode {

    pub fn combine_flags(strategy: [bool; 4], access: [bool; 4]) -> [bool; 4] {
        [
            strategy[0] || access[0],
            strategy[1] || access[1],
            strategy[2] || access[2],
            strategy[3] || access[3],
        ]
    }

    pub fn flags(&self) -> (bool, bool, bool, bool) {

        let flags = Self::combine_flags(self.0.flags(), self.1.flags());

        (flags[0], flags[1], flags[2], flags[3])

    }
}

pub struct FBin {
    path: String,
    handle: File,
    offset: u64,
}

impl FBin {

    fn get_handle(path: &str, read: bool, write: bool, create: bool, new: bool) -> Result<File, Error>{
        OpenOptions::new().read(read).write(write).create(create).create_new(new).open(path)
    }

    pub fn open(path: &str, mode: FMode) -> Result<Self, Error> {

        let (read, write, create, new) = mode.flags();

        Self::get_handle(path,  read, write, create, new)
        .map(|file| Self {path: path.to_string(), handle: file, offset: 0 })
    }

    pub fn file_exists(path: &str) -> bool {
        Self::open(path, FMode(FStrategy::Open, FAccess::Read)).is_ok()
    }

    pub fn remove(self) -> Result<(), Error>{
        remove_file(self.path)
    }

    pub fn remove_file(path: &str) -> Result<(), Error>{
        remove_file(path)
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

pub use FStrategy::*;
pub use FAccess::*;
