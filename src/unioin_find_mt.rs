use std::{
    io::{self, BufRead, Write},
    sync::{Arc, Mutex},
};

struct UnionFind {
    leaders: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            leaders: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_leader = self.find_leader(p);
        let q_leader = self.find_leader(q);

        if self.ranks[p_leader] < self.ranks[q_leader] {
            self.leaders[p_leader] = q_leader;
            self.ranks[q_leader] += self.ranks[p_leader];
        } else {
            self.leaders[q_leader] = p_leader;
            self.ranks[p_leader] += self.ranks[q_leader];
        }
    }

    fn find_leader(&mut self, mut p: usize) -> usize {
        while p != self.leaders[p] {
            self.leaders[p] = self.leaders[self.leaders[p]];
            p = self.leaders[p];
        }
        return p;
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find_leader(p) == self.find_leader(q)
    }
}

pub fn union_find() {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();
    let args: Vec<&str> = first_line.trim().split(' ').collect();
    let m: usize = args[1].parse().unwrap();
    let n: usize = args[0].parse().unwrap();

    let set = Arc::new(Mutex::new(UnionFind::new(n)));
    let mut handles = Vec::with_capacity(m);

    for _ in 0..f32::ceil(m as f32 / 1000.) as usize {
        let set = Arc::clone(&set);

        handles.push(std::thread::spawn(move || {
            let handle = io::stdin().lock();
            let lines: Vec<String> = handle
                .lines()
                .take(1000)
                .map(|line| line.unwrap())
                .collect();

            for line in lines {
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

                let mut set_guard = set.lock().unwrap();

                match operation {
                    "=" => set_guard.union(s, t),
                    "?" => {
                        let mut stdout = io::stdout().lock();
                        if set_guard.connected(s, t) {
                            write!(stdout, "yes\n").ok();
                        } else {
                            write!(stdout, "no\n").ok();
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
