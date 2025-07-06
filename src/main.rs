use std::io::Error;

use fbin::*;

fn main() -> Result<(), Error> {

    let mut buf = [0u8; 2];
    
    FBin::open("test", FMode(OpenOrCreate, ReadWrite))?
        .seek(0)?
        .write(&[255, 37, b'h'])?
        .seek(1)?
        .read(&mut buf)?
        .drop()?;
    
    println!("{}", buf[1]);
    
    Ok(())
}