use std::io::{self, BufRead, BufWriter, Write};

fn union(leaders: &mut [usize], ranks: &mut [usize], p: usize, q: usize) {
    let p_leader = find_leader(leaders, p);
    let q_leader = find_leader(leaders, q);

    // Set deeper tree as new leader (weighting).
    if ranks[p_leader] < ranks[q_leader] {
        leaders[p_leader] = q_leader;
        ranks[q_leader] += ranks[p_leader];
    } else {
        leaders[q_leader] = p_leader;
        ranks[p_leader] += ranks[q_leader];
    }
}

fn find_leader(leaders: &mut [usize], mut p: usize) -> usize {
    while p != leaders[p] {
        leaders[p] = leaders[leaders[p]]; // Path compression.
        p = leaders[p];
    }
    return p;
}

fn connected(leaders: &mut [usize], p: usize, q: usize) -> bool {
    find_leader(leaders, p) == find_leader(leaders, q)
}

pub fn union_find() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let stdin_handle = stdin.lock();
    let stdout_handle = stdout.lock();

    let mut output = BufWriter::new(stdout_handle);
    let mut lines = stdin_handle.lines();

    let first_line = lines.next().unwrap().unwrap();
    let args: Vec<&str> = first_line.split(' ').collect();
    let n: usize = args[0].parse().unwrap();

    let mut leaders: Vec<usize> = (0..n).collect();
    let mut ranks: Vec<usize> = vec![0; n];

    for line in lines {
        let line = line.unwrap();
        let args: Vec<&str> = line.split(' ').collect();

        let operation: &str;
        let s: usize;
        let t: usize;

        match args[0..3] {
            [op, val1, val2] => {
                operation = op;
                s = val1.parse().unwrap();
                t = val2.parse().unwrap();
            }
            _ => panic!("Invalid input format"),
        }

        match operation {
            "=" => union(&mut leaders, &mut ranks, s, t),
            "?" => {
                if connected(&mut leaders, s, t) {
                    write!(output, "yes\n").ok();
                } else {
                    write!(output, "no\n").ok();
                }
            }
            _ => unreachable!(),
        }

        let _ = output.flush();
    }
}
