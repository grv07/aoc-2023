use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Range;

fn main() {
    let lines = get_file_lines();
    parse_file(lines);
}

fn get_file_lines() -> Lines<BufReader<File>> {
    let mut args = std::env::args();
    args.next();

    let fln = args.next().unwrap();

    BufReader::new(File::open(fln).unwrap()).lines()
}

fn parse_file(mut lines: Lines<BufReader<File>>) {
    let seeds = if let Some(Ok(line)) = lines.next() {
        line.replace("seeds: ", "")
            .trim()
            .split(' ')
            .map(|v| {
                let _ = v.trim();
                v.parse::<usize>().unwrap()
            })
            .collect::<Vec<usize>>()
    } else {
        vec![]
    };
    // println!("{seeds:?}");

    let mut maps = vec![];
    let mut map = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if map.is_empty() {
                continue;
            }
            maps.push(map);
            map = HashMap::new();
            continue;
        } else {
            let d = line
                .split(' ')
                .into_iter()
                .map(|v| {
                    let _ = v.trim();
                    v.parse::<usize>().unwrap()
                })
                .collect::<Vec<usize>>();

            let src = d[1]..d[1] + d[2];
            let dest = d[0]..d[0] + d[2];

            map.insert(src, dest);
        }
    }
    maps.push(map);

    part1(seeds.clone(), maps.clone());
    part2(seeds, maps);
}

fn part1(seeds: Vec<usize>, maps: Vec<HashMap<Range<usize>, Range<usize>>>) {
    let mut res = usize::MAX;

    for seed in seeds {
        // println!("START: looking for seed {seed}");
        let mut q = seed;
        for map in &maps {
            for (k, v) in map.iter() {
                if k.contains(&q) {
                    let nq = v.start + (q - k.start);
                    q = nq;
                    break;
                }
            }
        }

        // println!("LAST: location is {q}");
        // println!(" ");

        if res > q {
            res = q;
        }
    }
    println!("Part1: {res}");
}

fn part2(seeds: Vec<usize>, maps: Vec<HashMap<Range<usize>, Range<usize>>>) {
    let mut res = usize::MAX;

    let mut seeds = seeds.iter();
    while let Some(f) = seeds.next() {
        let f = f.clone();
        let s = f + seeds.next().unwrap().clone();

        println!("{f:?} {s:?}");

        for seed in f..s {
            println!("START: looking for seed {seed}");
            let mut q = seed;
            for map in &maps {
                for (k, v) in map.iter() {
                    if k.contains(&q) {
                        let nq = v.start + (q - k.start);
                        q = nq;
                        break;
                    }
                }
            }

            // println!("LAST: location is {q}");
            // println!(" ");

            if res > q {
                res = q;
            }
        }
    }
    println!("Part2: {res}");
}
