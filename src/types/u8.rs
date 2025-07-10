use crate::BinaryType;


pub struct U8Le (pub u8);

impl BinaryType for U8Le {

    type Bytes = [u8; 1];
    type Value = u8;

    fn to_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(u8::from_le_bytes(bytes))
    }
    fn get(&self) -> Self::Value {
        self.0
    }
}