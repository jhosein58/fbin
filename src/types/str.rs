use crate::BinaryType;


pub struct StrLe (pub String);

impl BinaryType for StrLe {

    type Bytes = Vec<u8>;
    type Value = String;

    fn to_bytes(&self) -> Self::Bytes {
        self.0.as_bytes().to_owned()
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(String::from_utf8(bytes).unwrap_or(String::new()))
    }
    fn get(&self) -> Self::Value {
        self.0.clone()
    }
}