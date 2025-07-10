use crate::BinaryType;


pub struct U32Le (pub u32);

impl BinaryType for U32Le {

    type Bytes = [u8; 4];
    type Value = u32;

    fn to_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(u32::from_le_bytes(bytes))
    }
    fn get(&self) -> Self::Value {
        self.0
    }
}