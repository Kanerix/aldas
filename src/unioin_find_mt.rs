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
    let mut stdin = io::stdin().lock();

    let mut first_line = String::new();
    stdin.read_line(&mut first_line).unwrap();

    let args: Vec<&str> = first_line.trim().split(' ').collect();
    let m: usize = args[1].parse().unwrap();
    let n: usize = args[0].parse().unwrap();

    let set = Arc::new(Mutex::new(UnionFind::new(n)));

    let lines_buf: Vec<String> = stdin.lines().map(|line| line.unwrap()).collect();
    let lines = Arc::new(Mutex::new(lines_buf));

    let elements_pet_iter = 30000 as usize;
    let threads = f32::ceil(m as f32 / elements_pet_iter as f32) as usize;
    let mut handles = Vec::with_capacity(m);

    for _ in 0..threads {
        let set = Arc::clone(&set);
        let lines = Arc::clone(&lines);

        handles.push(std::thread::spawn(move || {
            let mut set_guard = set.lock().unwrap();
            let mut lines_guard = lines.lock().unwrap();

            let lines = if lines_guard.len() > elements_pet_iter {
                lines_guard.drain(..elements_pet_iter)
            } else {
                lines_guard.drain(..)
            };

            for line in lines {
                let args: Vec<&str> = line.split(' ').collect();

                let op: &str;
                let s: usize;
                let t: usize;

                match args[0..3] {
                    [op_val, s_val, t_val] => {
                        op = op_val;
                        s = s_val.parse().unwrap();
                        t = t_val.parse().unwrap();
                    }
                    _ => panic!("Invalid input format"),
                }

                match op {
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
