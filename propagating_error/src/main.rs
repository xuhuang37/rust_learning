use std::fs::File;
// use std::fs;
// use std::io;
use std::error::Error;
// use std::io::Read;

// fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    
    // A Shortcut for Propagating Errors: the ? Operator

    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // fs::read_to_string("hello.txt")
// }

fn main()->Result<(), Box<dyn Error>> {
    // read_username_from_file();
    let f = File::open("hello.txt")?;
    Ok(())
}
