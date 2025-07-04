use std::io::Error;

use fbin::FBin;

fn main() -> Result<(), Error> {

    FBin::open_or_create("test")?.seek(10)?.write(b"Hello World")?;

    Ok(())

}