use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Range;

type RMaps = Vec<HashMap<Range<isize>, Range<isize>>>;

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
                v.parse::<isize>().unwrap()
            })
            .collect::<Vec<isize>>()
    } else {
        vec![]
    };

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
                    v.parse::<isize>().unwrap()
                })
                .collect::<Vec<isize>>();

            let src = d[1]..d[1] + d[2];
            let dest = d[0]..d[0] + d[2];

            map.insert(src, dest);
        }
    }
    maps.push(map);

    part1(seeds.clone(), maps.clone());
    part2(seeds, maps);
}

fn part1(seeds: Vec<isize>, maps: Vec<HashMap<Range<isize>, Range<isize>>>) {
    let mut res = isize::MAX;

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

fn split_range(this: &Range<isize>, other: &Range<isize>) -> Vec<Range<isize>> {
    // this:  [----------------------------]
    // other:       [---------------]
    if this.start <= other.start && this.end >= other.end {
        return vec![
            this.start..other.start,
            other.start..other.end,
            other.end..this.end,
        ];
    }
    // this:      [-------------------]
    // other: [---------------]
    if this.start >= other.start && this.end >= other.end && this.start < other.end {
        return vec![this.start..other.end, other.end..this.end];
    }

    // this:     [----------]
    // other:  [-----------------]
    if this.start >= other.start && this.end <= other.end {
        return vec![this.start..this.end];
    }

    // this:  [------------]
    // other:     [---------------]
    if this.start <= other.start && this.end <= other.end && this.end > other.start {
        return vec![this.start..other.start, other.start..this.end];
    }

    vec![]
}

fn dfs(
    range: Range<isize>,
    maps: &Vec<HashMap<Range<isize>, Range<isize>>>,
    res: &mut isize,
    level: usize,
) {
    println!("");
    println!("{:?} {}", &range, level + 1);
    if level == 7 {
        if *res >= range.start {
            *res = range.start;
            // println!("FINAL range: {res:?}");
        }

        return;
    }

    let map = &maps[level];

    // println!("");
    if level == 5 {
        // println!("Checks for range {:?} in map level {:?}", map, level);
    }

    let mut splits = vec![];
    for (k, v) in map {
        let s_r = split_range(&range, &k);
        if !s_r.is_empty() {
            println!("{:?} Split: {:?} in k: {:?}", &range, &s_r, &k);
        }
        for split in split_range(&range, &k) {
            if k.contains(&split.start) {
                let d = v.start - k.start;
                let ds = split.start + d;
                let de = split.end + v.end - k.end;
                let split = ds..de;
                splits.push(split);
            } else {
                splits.push(split);
            }
        }
    }

    if splits.is_empty() {
        dfs(range.clone(), maps, res, level + 1);
        return;
    }

    let nlevel = level + 1;
    for split in splits {
        println!("Next call with range: {split:?}");
        dfs(split, maps, res, nlevel.clone());
    }
}

fn part2(seeds: Vec<isize>, maps: RMaps) {
    let mut seeds = seeds.iter();
    let mut locations = vec![];
    while let (Some(f), Some(s)) = (seeds.next(), seeds.next()) {
        let f = f.clone();
        let s = s.clone();
        let q = f..(f + s);
        let mut res = isize::MAX;
        dfs(q, &maps, &mut res, 0);
        locations.push(res);
    }
    println!("Locations are: {locations:?}");
}

#[cfg(test)]
mod test {
    use split_range;

    #[test]
    fn split_range_test() {
        // assert_eq!(split_range(10..20, 0..30), vec![10..20]);
        // assert_eq!(split_range(0..200, 10..30), vec![0..10, 10..30, 30..200]);
        // assert_eq!(split_range(0..200, 0..30), vec![0..0, 0..30, 30..200]);
        // assert_eq!(split_range(0..20, 10..30), vec![0..10, 10..20]);
        // assert_eq!(split_range(15..40, 10..30), vec![15..30, 30..40]);
        // assert_eq!(split_range(10..40, 10..30), vec![10..10, 10..30, 30..40]);
        assert_eq!(
            split_range(&(2533907499..2590772674), &(2552562071..2604610343)),
            vec![10..10, 10..30, 30..40]
        );
        //         2533907499..2590772674 5
        // Split for range: [2533907499..2552562071, 2552562071..2590772674] in k: 2552562071..2604610343
        // Split for range: [2533907499..2552562071, 2552562071..2590772674] in k: 2475393991..2552562071
    }
}
