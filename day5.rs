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

    // part1(seeds.clone(), maps.clone());
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

// fn range_in(a: &Range<isize>, b: &Range<isize>) -> Option<Range<isize>> {
//     let (a_start, a_end) = (a.start, a.end);
//     let (b_start, b_end) = (b.start, b.end);

//     let start = a_start.max(b_start);
//     let end = a_end.min(b_end);

//     if start <= end {
//         Some(start..end)
//     } else {
//         None
//     }
// }

fn dfs(
    mut range: Range<isize>,
    maps: &Vec<HashMap<Range<isize>, Range<isize>>>,
    // res: isize,
    level: usize,
) {
    if level > 6 {
        println!("FINAL range: {range:?}");
        return;
    }

    let map = &maps[level];

    println!("Checks for range {:?} map {:?}", range, map);
    let mut nlevel = level + 1;
    for (k, v) in map {
        let splits = split_range(&range, &k);
        println!("Split {splits:?} {k:?}");
        for split in split_range(&range, &k) {
            println!("Split: {split:?}");
            if k.contains(&split.start) {
                let ds = split.start + v.start - k.start;
                let de = split.end + v.end - k.end;
                println!(">> {ds:?} {de:?}");
                let split = ds..de;
                println!("Next call with range: {split:?}");
                dfs(split, maps, nlevel);
                // range = nq;
            } else {
                // range = split;
                println!("Next call with range: {split:?}");
                dfs(split, maps, nlevel);
            }
        }
    }
    dfs(range.clone(), maps, level + 1);
}

fn sol(seed: Range<isize>, maps: &RMaps) {
    let mut q = seed.clone();
    for map in maps.iter() {
        println!("Map: {map:?}");
        for (k, v) in map {
            for split in split_range(&q, k) {
                println!("split  on {k:?} {split:?}");
                if k.contains(&split.start) {
                    let ds = split.start + v.start - k.start;
                    let de = split.end + v.end - k.end;
                    println!("{ds:?} {de:?}");
                    let nq = ds..de;

                    if nq.start < q.start {
                        q = nq;
                    }
                } else {
                    q = split;
                }
            }
            println!("q= {q:?}");
        }
    }
}

fn part2(seeds: Vec<isize>, maps: RMaps) {
    let mut res = isize::MAX..isize::MAX;

    let mut seeds = seeds.iter();
    while let (Some(f), Some(s)) = (seeds.next(), seeds.next()) {
        let f = f.clone();
        let s = s.clone();
        let mut q = f..(f + s);
        // println!("{q:?}");
        // println!("{maps:?}");
        // sol(q, &maps);
        dfs(
            q, &maps, // isize::MAX,
            0,
        );
    }
    // println!("Part2: {res:?}");
}

#[cfg(test)]
mod test {
    use split_range;

    #[test]
    fn split_range_test() {
        assert_eq!(split_range(10..20, 0..30), vec![10..20]);
        assert_eq!(split_range(0..200, 10..30), vec![0..10, 10..30, 30..200]);
        assert_eq!(split_range(0..200, 0..30), vec![0..0, 0..30, 30..200]);
        assert_eq!(split_range(0..20, 10..30), vec![0..10, 10..20]);
        assert_eq!(split_range(15..40, 10..30), vec![15..30, 30..40]);
        assert_eq!(split_range(10..40, 10..30), vec![10..10, 10..30, 30..40]);
    }
}
