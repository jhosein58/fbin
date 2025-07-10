pub mod access;
pub mod strategy;

pub use access::FAccess;
pub use strategy::FStrategy;


use std::fs::{OpenOptions, File};
use std::io::Error;


pub struct FMode(pub FStrategy, pub FAccess);

impl FMode {

    fn combine_flags(strategy: [bool; 4], access: [bool; 4]) -> [bool; 4] {
        [
            strategy[0] || access[0],
            strategy[1] || access[1],
            strategy[2] || access[2],
            strategy[3] || access[3],
        ]
    }

    fn flags(&self) -> (bool, bool, bool, bool) {

        let flags = Self::combine_flags(self.0.flags(), self.1.flags());

        (flags[0], flags[1], flags[2], flags[3])

    }

    pub fn get_handle(&self, path: &str) -> Result<File, Error> {
        let (r, w, c, n) = self.flags();
        OpenOptions::new().read(r).write(w).create(c).create_new(n).open(path)
    }
}