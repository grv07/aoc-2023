use std::collections::HashMap;
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
    let mut lines = get_lines();

    let mut sum1 = 0;
    let mut rs: HashMap<i32, i32> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }
        sum1 += part1(&line);
        part2(&line, &mut rs);
    }

    println!("Part 1: {sum1}");

    let sum2 = rs.values().sum::<i32>();
    println!("Part 2: {sum2}");
}

fn parse_input(line: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let (card, numbers) = line.split_once(':').unwrap();
    let card = card.replace("Card", "");
    let card = card.trim();
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

    (card.parse::<i32>().unwrap(), win, yh)
}

fn get_win_count(win: Vec<i32>, yh: Vec<i32>) -> i32 {
    let mut count: i32 = 0;

    for w in win {
        if yh.contains(&w) {
            count += 1;
        }
    }
    count
}

fn part2(line: &str, rs: &mut HashMap<i32, i32>) {
    let (card, win, yh) = parse_input(line);
    rs.entry(card).or_insert(1);

    let wc = get_win_count(win, yh);

    let tc = rs.get(&card).unwrap().clone();

    for i in 1..wc + 1 {
        rs.entry(card + i)
            .and_modify(|v| {
                // for j in 0..tc {
                *v += tc;
                // }
            })
            .or_insert(1 + tc);
    }
    // println!("line: {card} {:?}", wc);
}

fn part1(line: &str) -> i32 {
    let mut sum = 0;

    let (_, win, yh) = parse_input(&line);

    let wc = get_win_count(win, yh);
    sum += 2_i32.pow(wc as u32) / 2;

    sum
}
