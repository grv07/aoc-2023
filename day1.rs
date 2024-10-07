use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        println!("Error: Expect input file");
    }

    let file = args.nth(1).unwrap();
    println!("Read Input file {file:?}");

    let file = File::open(file).unwrap();
    let br = BufReader::new(file);
    let mut lines = br.lines().into_iter();

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let c: Vec<char> = line.chars().into_iter().collect();

        let h = line.find(|c: char| c.is_digit(10)).unwrap();
        let t = line.rfind(|c: char| c.is_digit(10)).unwrap();

        let num = format!("{}{}", c[h], c[t]).parse::<i32>().unwrap();
        sum += num;
    }
    println!("{sum:?}");
}
