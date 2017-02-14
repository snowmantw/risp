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
//
// The major problem here is Rust provide some nice method to read data as string,
// such as `read_line` or `read_until`. However, since parser is just a state-machine,
// we don't want an extra buffer for just reading things. So we need a "1 char only" reader
// to read "1 char" each time, and then discard it from memory.
//
// Rust has the capability to do that (Read::chars), but it is an unstable implementation.
// So either we need to handle the all cases bytes-to-utf8 will encounter,
// or we stay at ASCII-only situation.
fn parse(bs: &[u8]) -> Result<(), io::Error> {
    let str_ch = str::from_utf8(bs).unwrap();
    print!(">> echo: {}", str_ch);
    Ok(())      // TODO
}
