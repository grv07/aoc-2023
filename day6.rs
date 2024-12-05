// Time: 7 15 30
// Dist: 9 40 200
use std::io::BufRead;

fn parse_file_from_arg() -> Vec<(i64, i64)> {
    let mut args = std::env::args();
    args.next();
    let file_name = args.next();
    let b_reader = std::io::BufReader::new(std::fs::File::open(file_name.unwrap()).unwrap());

    let mut lines = b_reader.lines();

    let data = lines
        .next()
        .unwrap()
        .unwrap()
        .replace("Time:", "")
        .trim()
        .replace("  ", "");
    // dbg!(&data);
    let time: Vec<i64> = data
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let data = lines
        .next()
        .unwrap()
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .replace("  ", "");

    data.split(" ")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .zip(time)
        .collect::<Vec<(i64, i64)>>()
}

fn main() {
    let resp = parse_file_from_arg();
    println!("{resp:?}");
    let mut res = 1;

    for (d, time) in resp {
        res = res * part1(time, d).len();
    }

    println!("{}", res);
}

fn part1(time: i64, d: i64) -> Vec<(i64, i64)> {
    let mut ires = vec![];
    let half = if time % 2 == 0 {
        time / 2
    } else {
        time / 2 + 1
    };

    for (i, j) in (1..(time - half + 1)).rev().enumerate() {
        let upper_limit = half + i as i64;
        if (upper_limit * j) > d {
            // println!(">>> {half} {j}");
            ires.push((upper_limit, j));
            if upper_limit != j {
                ires.push((j, upper_limit));
            }
        }
    }

    ires
}
