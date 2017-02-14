use std::io::{self, Read};
use std::str;

fn main() {
    println!("Hello, world!");

    // They say parser is just a state-machine.
    let mut buffer = [0; 1];
    while let Ok(bytes_read) = io::stdin().read(&mut buffer) {
        if bytes_read == 0 { break; }             // EOF
        parse(&buffer[..bytes_read]).unwrap();    // Unwrap to get result of this operation.
    }
}

// Because stdin only read 1 byte at a time.
// So this actually restrict the input can only be ASCII,
// unless we have more sophistic way to read variable length of bytes
// for UTF-8.
fn parse(bs: &[u8]) -> Result<(), io::Error> {
    let str_ch = str::from_utf8(bs).unwrap();
    print!(">> echo: {}", str_ch);
    Ok(())      // TODO
}
