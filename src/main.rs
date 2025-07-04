use std::io::Error;

use fbin::FBin;

fn main() -> Result<(), Error> {

    println!("{:?}", FBin::file_exists("test"));
    FBin::open_or_create("test")?.seek(10)?.write(b"Hello World")?;
    println!("{:?}", FBin::file_exists("test"));

    Ok(())

}