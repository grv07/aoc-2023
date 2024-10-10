use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::str::Chars;

fn get_file_name() -> String {
    let mut args = std::env::args();

    assert!(args.len() > 1, "USAGE: input file required");

    args.next();

    let file_name = args.next().expect("USAGE: input file required");
    file_name
}

fn main() {
    let file_name = get_file_name();
    let mut lines = BufReader::new(File::open(file_name).expect("Unable to find file")).lines();

    let mut curr = lines.next().unwrap().unwrap();
    let mut prev = ".".repeat(curr.len());
    let mut is_done = false;
    let mut sum = 0;

    while !is_done {
        if let Some(Ok(next)) = lines.next() {
            let res = part1(&prev, &curr, &next);
            sum += res;

            prev = curr.to_string();
            curr = next;
        } else {
            is_done = true;
            let next = ".".repeat(curr.len());

            let res = part1(&prev, &curr, &next);
            sum += res;
        }
    }

    println!("Result: {}", sum);
}

fn _dump(m: &Vec<Vec<char>>) {
    for m in m {
        println!("{m:?}");
    }
}

fn part1(prev: &str, curr: &str, next: &str) -> usize {
    let mut sum = 0;

    let mt = vec![
        prev.chars().into_iter().collect::<Vec<char>>(),
        curr.chars().into_iter().collect::<Vec<char>>(),
        next.chars().into_iter().collect::<Vec<char>>(),
    ];

    // dump(&mt);

    // println!("");

    let mut v = HashSet::new();
    for c in 0..mt[1].len() {
        if mt[1][c].is_digit(10) && !v.contains(&(1, c)) {
            let (res, sym) = bfs(&mt, (1, c), &mut v);

            if sym {
                sum += res.parse::<usize>().unwrap();
            }
        }
    }
    sum
}

fn bfs(mt: &Vec<Vec<char>>, s: (usize, usize), v: &mut HashSet<(usize, usize)>) -> (String, bool) {
    let mut res = String::new();
    let mut sym = false;
    let rc = 3;
    let cc = mt[0].len() as i32;

    let mut q = VecDeque::new();
    q.push_back(s);

    while let Some(item) = q.pop_front() {
        v.insert(item);

        let (r, c) = item;
        res.push(mt[r][c]);

        for inr in vec![-1, 0, 1] {
            for inc in vec![-1, 0, 1] {
                let nr = inr + r as i32;
                let nc = inc + c as i32;

                if nr >= 0 && nr < rc && nc >= 0 && nc < cc {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if mt[nr][nc] != '.' && !mt[nr][nc].is_digit(10) {
                        sym = true;
                    }

                    if (inr, inc) == (0, 1) {
                        if mt[nr][nc].is_digit(10) {
                            q.push_back((nr, nc));
                            continue;
                        }
                    }
                }
            }
        }
    }

    (res, sym)
}
