
use crate::core::FBin;
pub trait  BinaryType {
    
    type Bytes: AsMut<[u8]> + Default;
    type Value;

    fn to_bytes(&mut self) -> Self::Bytes;

    fn from_bytes(bytes: Self::Bytes) -> Self;

    fn get_buf(ln: usize) -> Self::Bytes;

    fn get(&self) -> Self::Value;

    fn len(fh: &mut FBin) -> usize;
}

pub mod u8;
pub use u8::U8Le;

pub mod u16;
pub use u16::U16Le;

pub mod u32;
pub use u32::U32Le;

pub mod str;
pub use str::StrLe;