use crate::BinaryType;


pub struct U16Le (pub u16);

impl BinaryType for U16Le {

    type Bytes = [u8; 2];
    type Value = u16;

    fn to_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
    fn get(&self) -> Self::Value {
        self.0
    }
}