use std::io::{self, BufRead, BufReader, BufWriter};

pub fn game_of_thones() {
    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();

    let _output = BufWriter::new(stdout);
    let input = BufReader::new(stdin);

    let mut lines = input.lines();

    match lines.next().unwrap() {
        Ok(line) => {
            let args: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            (args[0], args[1])
        }
        Err(_) => unreachable!(),
    };
}
