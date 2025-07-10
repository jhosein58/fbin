mod mode;
mod core;
mod types;


use mode::*;
pub use mode::FMode;
pub use FStrategy::*;
pub use FAccess::*;

pub use core::FBin;

pub use types::*;

#[cfg(test)]
mod tests;