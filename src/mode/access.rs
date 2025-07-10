#[derive(Copy, Clone)]
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