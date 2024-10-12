use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};

fn get_lines() -> Lines<BufReader<File>> {
    let mut args = std::env::args();
    assert!(args.len() > 1, "USAGE: Expect input file name");
    args.next();

    let f = args.next().unwrap();

    BufReader::new(File::open(f).expect("Error: Unable to open file")).lines()
}

fn main() {
    let lines = get_lines();
    let sum = part1(lines);
    println!("Part 1: {sum}");
}

fn part1(mut lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let (_card, numbers) = line.split_once(':').unwrap();
        let numbers = numbers.trim_start();

        let (win, yh) = numbers.split_once('|').unwrap();
        let win = win
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let yh = yh
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let winning_sum = |win: Vec<i32>, yh: Vec<i32>| -> i32 {
            let mut ws: i32 = 0;

            for w in win {
                if yh.contains(&w) {
                    ws += 1;
                }
            }

            2_i32.pow(ws as u32) / 2
        };

        sum += winning_sum(win, yh);
    }

    sum
}
