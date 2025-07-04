use std::io::Error;

use fbin::FBin;

fn main() -> Result<(), Error> {

    let f = FBin::open("test");
    f.write(b"hello-----")?.seek(6)?.write(b"fbin ")?.write(b"!")?;

    Ok(())
}