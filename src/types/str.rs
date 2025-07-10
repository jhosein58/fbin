use crate::{BinaryType, U32Le};
use crate::core::FBin;

pub struct StrLe (pub String);

impl BinaryType for StrLe {

    type Bytes = Vec<u8>;
    type Value = String;

    fn to_bytes(&mut self) -> Self::Bytes {
       
        let bytes = self.0.as_bytes();
        let len = bytes.len(); 
        let mut res = U32Le(len as u32).to_bytes();
        res.extend_from_slice(bytes);  
        res
    }
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(String::from_utf8(bytes).unwrap_or(String::new()))
    }
    fn get(&self) -> Self::Value {
        self.0.clone()
    }
    fn len(fh: &mut FBin) -> usize {
        fh.read::<U32Le>().unwrap_or(U32Le(0)).get() as usize
    }
    fn get_buf(ln: usize) -> Self::Bytes {
        vec![0u8; ln]
    }
}