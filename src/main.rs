extern crate regex;

// Need to make nodes as a crate so main.rs can use it w/o coupling
// it with the nodes module.
//
// And one annoying thing is the 'lib.rs' of 'nodes' crate,
// seems need to be specified in Cargo.toml.
extern crate nodes;

use std::io::{self, Read};
use std::str;
use regex::Regex;
use nodes::node;
use nodes::number;
use nodes::list;

fn main() {
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
// Unfortunately, although Rust has the capability to do that (Read::chars),
// it is still an unstable implementation.
// So either we need to handle the all cases during bytes-to-utf8,
// or we stay at ASCII-only situation.
fn parse(bs: &[u8]) -> Result<(), io::Error> {
    let str_ch = str::from_utf8(bs).unwrap();
    if Regex::new(r"\d{1}").unwrap().is_match(&str_ch) {
        print!(">>> Number");
        let node = number::Number {
            node: node::Node {
                text: str_ch.to_string()
            }
        };
    } else if Regex::new(r"\({1}").unwrap().is_match(&str_ch){
        print!(">>> ListBegin");
        let node = list::ListBegin {
            node: node::Node {
                text: str_ch.to_string()
            }
        };
    } else if Regex::new(r"\){1}").unwrap().is_match(&str_ch){
        print!(">>> ListEnd");
        let node = list::ListEnd {
            node: node::Node {
                text: str_ch.to_string()
            }
        };
    }

    // TODO: to have a state-transferring mechanism here.
    // TODO: and the transferring method need to accept a trait.
    Ok(())      // TODO
}

/*
// The body of the parser.
// 
// It's "kind of" a LR parser because it can begin from very
// basic patterns to construct the whole tree.
//

// Not now. TODO.
struct Lineno {
    line: i32,
    column: i32,
}

struct Node {
    text: String,
    lineno: Lineno,
}

trait Validator {
    fn validate(node: Node) -> bool;
}

// Do something like to store the position of a string fragment,
// then give the next state to receive the next text segment.
//
// Compound state can be generated via the latest transferring:
//
// ListBegin -> AtomSymbol -> AtomSymbol -> ... -> ListEnd -> List
//
// The latest List get created because the chain successfully reach ListEnd. 
//
trait State {
    fn transfer(&self, node: Node) -> State;
}

struct ProgramBegin {
    node: Node,
}

impl State for ProgramBegin {
    fn transfer(&self, node: Node) -> State {
        if Number::validate(node) {
            return Invalid {
                message: "Invalid start for a list as a Number".to_string(),
            }
        } else if ListBegin::validate(node) {
            return ListBegin {
                node: node,
            }
        } else {
            return Invalid {
                message: "Invalid start for a list as a not a ListBegin".to_string(),
            }
        }
    }
}

struct ProgramEnd {
    node: Node,
}

impl State for ProgramEnd {
    // It's end. So any extra input is invalid.
    fn transfer(&self, node: Node) -> State {
        return Invalid {
            message: "Invalid transferring after ProgramEnd".to_string(),
        }
    }
}

struct AtomSymbol {

}

struct AtomPart {

}

struct Number {}

struct SExpression {}

struct List {}

struct ListBegin {}

struct ListEnd {}

// Is a state for any error during parsing.
// How to handle that is the duty of the outside controller.
struct Invalid {
    message: String
}

// Desugar for some quote
//
// The quote expression:
//
//     '(+ 1 2)
//
// is equal to
//
//     (quote 1 2)
//
// So there is no need to add extra burden to parser.
// This de-sugaring things will be handled by pre-processer,
// with a simple RegExp replacement.
//
*/
