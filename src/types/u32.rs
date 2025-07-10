use crate::BinaryType;


pub struct U32Le (pub u32);

impl BinaryType for U32Le {

    type Bytes = Vec<u8>;
    type Value = u32;

    fn to_bytes(&mut self) -> Self::Bytes {
        self.0.to_le_bytes().to_vec()
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
    fn get(&self) -> Self::Value {
        self.0
    }
    fn get_buf(ln: usize) -> Self::Bytes {
        vec![0u8; ln]
    }
    fn len(_: &mut crate::FBin) -> usize {
        4
    }
}