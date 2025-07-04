use std::io::Error;

use fbin::FBin;

fn main() -> Result<(), Error> {

    FBin::open_or_create("test")?.write(b"hello-----")?.seek(6)?.write(b"fbin ")?.write(b"!")?;

    Ok(())

}