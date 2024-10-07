use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        println!("Error: Expect input file");
    }

    let file = args.nth(1).unwrap();
    // println!("Read Input file {file:?}");

    let file = File::open(file).unwrap();
    let br = BufReader::new(file);
    let mut lines = br.lines().into_iter();

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let cs: Vec<char> = line.chars().into_iter().collect();

        let fd = first_digit(&cs);
        let ld = last_digit(&cs);
        println!("{fd:?}{ld:?}");

        let fcd = first_chars_digit(&line);
        let lcd = last_chars_digit(&line);
        println!("{fcd:?}{lcd:?}");

        let fd = if fcd.0 == -1 {
            fd.1
        } else if fd.0 == -1 {
            fcd.1
        } else if fcd.0 > fd.0 {
            fd.1
        } else {
            fcd.1
        };

        let ld = if lcd.0 < ld.0 { ld.1 } else { lcd.1 };
        let num = format!("{}{}", fd, ld).parse::<i32>().unwrap();

        sum += num;
    }
    println!("{sum:?}");
}

fn first_digit(cs: &Vec<char>) -> (i32, char) {
    for i in 0..cs.len() {
        if cs[i].is_digit(10) {
            return (i as i32, cs[i]);
        }
    }

    (-1, '0')
}

fn first_chars_digit(cs: &str) -> (i32, char) {
    for i in 0..cs.len() {
        if cs[i..].starts_with("one") {
            return (i as i32, '1');
        }
        if cs[i..].starts_with("two") {
            return (i as i32, '2');
        }
        if cs[i..].starts_with("three") {
            return (i as i32, '3');
        }
        if cs[i..].starts_with("four") {
            return (i as i32, '4');
        }
        if cs[i..].starts_with("five") {
            return (i as i32, '5');
        }
        if cs[i..].starts_with("six") {
            return (i as i32, '6');
        }
        if cs[i..].starts_with("seven") {
            return (i as i32, '7');
        }
        if cs[i..].starts_with("eight") {
            return (i as i32, '8');
        }
        if cs[i..].starts_with("nine") {
            return (i as i32, '9');
        }
    }
    (-1, '0')
}

fn last_chars_digit(cs: &str) -> (i32, char) {
    for i in (0..cs.len()).rev() {
        if cs[i..].starts_with("one") {
            return (i as i32, '1');
        }
        if cs[i..].starts_with("two") {
            return (i as i32, '2');
        }
        if cs[i..].starts_with("three") {
            return (i as i32, '3');
        }
        if cs[i..].starts_with("four") {
            return (i as i32, '4');
        }
        if cs[i..].starts_with("five") {
            return (i as i32, '5');
        }
        if cs[i..].starts_with("six") {
            return (i as i32, '6');
        }
        if cs[i..].starts_with("seven") {
            return (i as i32, '7');
        }
        if cs[i..].starts_with("eight") {
            return (i as i32, '8');
        }
        if cs[i..].starts_with("nine") {
            return (i as i32, '9');
        }
    }
    (-1, '0')
}

fn last_digit(cs: &Vec<char>) -> (i32, char) {
    for i in (0..cs.len()).rev() {
        if cs[i].is_digit(10) {
            return (i as i32, cs[i]);
        }
    }
    (-1, '0')
}
