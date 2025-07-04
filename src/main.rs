use std::io::Error;

use fbin::*;

fn main() -> Result<(), Error> {

    println!("{:?}", FBin::file_exists("test"));

    FBin::open("test", FMode(OpenOrCreate, ReadWrite))?.seek(10)?.write(b"Hello World")?;

    println!("{:?}", FBin::file_exists("test"));

    Ok(())

}