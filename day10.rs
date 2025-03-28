use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> Vec<Vec<char>> {
    let mut args = std::env::args();
    let _ = args.next();

    let mut input = vec![];
    if let Some(file_name) = args.next() {
        // println!("{file_name}");
        let br = BufReader::new(File::open(file_name).expect("File not found"));
        let mut lines = br.lines();
        while let Some(Ok(line)) = lines.next() {
            let mut inner = vec![];
            for c in line.chars() {
                inner.push(c);
            }
            input.push(inner);
        }
    }
    input
}

fn get_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

type Point<T> = (T, T);

fn avail_paths(c: char) -> [(i32, i32); 2] {
    match c {
        '|' => [(-1, 0), (1, 0)],
        '-' => [(0, -1), (0, 1)],
        'L' => [(-1, 0), (0, 1)],
        'J' => [(-1, 0), (0, -1)],
        '7' => [(0, -1), (1, 0)],
        'F' => [(0, 1), (1, 0)],
        'S' => [(2, 2), (2, 2)],
        _ => [(0, 0), (0, 0)],
    }
}

fn can_connect(input: &Vec<Vec<char>>, a: Point<usize>, b: Point<usize>) -> bool {
    let n = input.len() as i32;
    for i in &avail_paths(input[a.0][a.1]) {
        let temp: Point<i32> = (a.0 as i32 + i.0 as i32, a.1 as i32 + i.1 as i32);

        if temp.1 >= n || temp.0 < 0 {
            continue;
        }

        let temp: Point<usize> = (temp.0 as usize, temp.1 as usize);
        if temp == b {
            return true;
        }
    }
    false
}

fn dfs(
    input: &Vec<Vec<char>>,
    mut s: Point<usize>,
    mut prev: Point<usize>,
    mut res: usize,
) -> usize {
    let n = input.len() as i32;

    loop {
        if input[s.0][s.1] == 'S' {
            return res;
        }

        for i in &avail_paths(input[s.0][s.1]) {
            let nc = (i.0 + s.0 as i32, i.1 + s.1 as i32);

            if nc.1 >= n || nc.0 < 0 {
                continue;
            }
            let nc = (nc.0 as usize, nc.1 as usize);

            if nc == prev || !can_connect(input, s, nc) {
                continue;
            }

            prev = s;
            s = nc;
            res += 1;
        }
    }
}

fn main() {
    let input = get_input();
    let n = input.len() as i32;
    let s = get_start(&input);

    let mut res = 0;
    for (x, y) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nc = (s.0 as i32 + x, s.1 as i32 + y);
        if nc.1 >= n || nc.1 < 0 || nc.0 >= n || nc.0 < 0 {
            continue;
        }

        let ns = (nc.0 as usize, nc.1 as usize);
        // println!("{}", input[nc.0 as usize][nc.1 as usize]);

        res = res.max(dfs(&input, ns, s, 1));
    }

    println!("{res}");
}
