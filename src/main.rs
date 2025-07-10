use std::io::Error;

use fbin::*;

fn main() -> Result<(), Error> {

    let mut f = FBin::open("test", FMode(OpenOrCreate, ReadWrite))?;

    f.seek(0)?.write(StringLe("hello world".to_string()))?.seek(0)?;
    let res: StringLe = f.read()?;
    f.drop()?;

    println!("{}", res.get());
    
    Ok(())
}