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
