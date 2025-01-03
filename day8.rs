use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (String, HashMap<String, (String, String)>) {
    let mut args = std::env::args();
    args.next();

    let file_name = args.next().unwrap();

    let br = BufReader::new(File::open(file_name).unwrap());

    let mut lines = br.lines();
    let steps = lines.next().unwrap().unwrap().to_string();
    // println!("{steps}");

    let mut res = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let (node, path) = line.split_once('=').unwrap();
        let (path1, path2) = path.split_once(',').unwrap();
        let path1 = path1.trim().replace("(", "").to_string();
        let path2 = path2.trim().replace(")", "").to_string();

        res.insert(node.trim().to_string(), (path1, path2));
    }

    (steps, res)
}

fn walk(steps: &str, nodes: &HashMap<String, (String, String)>) {
    let mut start = "AAA";
    let mut count = 0;
    for step in steps.chars().cycle() {
        if let Some(node) = nodes.get(start) {
            count += 1;
            if step == 'L' {
                start = &node.0;
            } else {
                start = &node.1;
            }

            if start == "ZZZ" {
                println!("COUNT: {count}");
                return;
            }
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(a, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn walk2(steps: &str, nodes: &HashMap<String, (String, String)>) {
    let mut starts = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .collect::<Vec<&String>>();

    let mut res = vec![];
    for i in 0..starts.len() {
        let mut start = starts[i];
        let mut count = 0;
        for step in steps.chars().cycle() {
            // println!("{start:?}");

            if let Some(node) = nodes.get(start) {
                count += 1;
                if step == 'L' {
                    start = &node.0;
                } else {
                    start = &node.1;
                }
            }

            if start.ends_with('Z') {
                println!("COUNT: {start} {count}");
                res.push(count);
                break;
            }
        }
    }

    // let mut a = res.first().unwrap();
    // for b in res {
    //     a = &gcd(*a, b);
    // }

    let res = res.iter().fold(1, |acc, x| lcm(*x, acc));

    println!("{res:?}");
}

fn main() {
    let (input, map) = parse_input();

    walk(&input, &map);
    walk2(&input, &map);
}
