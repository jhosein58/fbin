use std::io::Error;

use fbin::*;

fn main() -> Result<(), Error> {

    let mut f = FBin::open("test", FMode(OpenOrCreate, ReadWrite))?;

    f.seek(0)?;
    f.write(StrLe("a ".to_string()))?;
    f.write(StrLe("b ".to_string()))?;
    f.write(StrLe("c ".to_string()))?;
    f.write(StrLe("d ".to_string()))?;
    f.seek(0)?;
    let res = f.read::<StrLe>()?.get() + f.read::<StrLe>()?.get().as_str() + f.read::<StrLe>()?.get().as_str();
    f.drop()?;

    println!("{}", res);
    
    Ok(())
}