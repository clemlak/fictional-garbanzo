use std::env;
use std::fs;
use std::io;

fn main()  -> io::Result<()> {
    let file_content = fs::read_to_string("./foo.huff")?;
    println!("{}", file_content);
    Ok(())
}
