use std::io::Error;

use fbin::*;

fn main() -> Result<(), Error> {

    FBin::open("test", FMode(OpenOrCreate, ReadWrite))?.seek(10)?.write(b"Hello World")?;

    Ok(())
}