use std::{
    collections::VecDeque,
    io::{BufReader, Read},
};

pub fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let mut stack = Vec::with_capacity(buf.len());
    let mut balanced = false;

    for c in buf.iter() {
        match c {
            40 => stack.push(40),
            91 => stack.push(91),
            41 => {
                match stack.pop() {
                    Some(num) => {
                        if num != 40 {
                            break;
                        }
                    }
                    None => break,
                };
            }
            93 => {
                match stack.pop() {
                    Some(num) => {
                        if num != 91 {
                            break;
                        }
                    }
                    None => break,
                };
            }
            _ => {
                if stack.len() == 0 {
                    balanced = true;
                }
            }
        }
    }

    if balanced {
        println!("1");
    } else {
        println!("0");
    }
}
