
use std::fs::{File, remove_file};
use std::io::{Error, Seek, SeekFrom, Write as w_l, Read as r_l};

use crate::mode::*;
use crate::types::*;

pub struct FBin {
    path: String,
    handle: File,
}

impl FBin {


    pub fn open(path: &str, mode: FMode) -> Result<Self, Error> {
        mode.get_handle(path)
        .map(|file| Self {path: path.to_string(), handle: file })
    }

    pub fn file_exists(path: &str) -> bool {
        Self::open(path, FMode(FStrategy::Open, FAccess::Read)).is_ok()
    }

    pub fn drop(&self) -> Result<(), Error>{
        remove_file(self.path.clone())
    }

    pub fn remove_file(path: &str) -> Result<(), Error>{
        remove_file(path)
    }

    pub fn seek(&mut self, to: u64) -> Result<&mut Self, Error>{
        self.handle.seek(SeekFrom::Start(to))?;
        Ok(self)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Error> {
        self.handle.write_all(bytes)?;
        Ok(self)
    }
    
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<&mut Self, Error>{
        self.handle.read_exact(buf)?;
        Ok(self)
    }

    pub fn write<T>(&mut self, data: T) -> Result<&mut Self, Error>
    where T: BinaryType
    {
        Ok(self.write_bytes(&data.to_bytes().as_mut())?)
    }
    pub fn read<T>(&mut self) -> Result<T, Error>
    where T: BinaryType
    {
        let mut buf = T::get_buf();
        self.read_bytes(buf.as_mut())?;
        Ok(T::from_bytes(buf))
    }

}