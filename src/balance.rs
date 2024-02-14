use std::io::{BufReader, BufWriter, Read, Write};

pub fn main() {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    let mut input = BufReader::new(stdin);
    let mut output = BufWriter::new(stdout);

    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap();

    let mut stack = Vec::with_capacity(buf.len());
    let mut balanced = false;

    for b in buf.iter() {
        match b {
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
        write!(output, "1\n").ok();
    } else {
        write!(output, "0\n").ok();
    }
}
