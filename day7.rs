use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const RANKS: [i64; 7] = [11111, 1112, 122, 113, 23, 14, 5];

fn get_parsed_input() -> Vec<(String, i64)> {
    let mut args = std::env::args();

    args.next();
    let file_name = args.next().unwrap();

    let mut res = vec![];
    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines().into_iter() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(" ");

        res.push((
            parts.next().unwrap().to_string(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        ));
    }
    res
}

fn main() {
    let daya = get_parsed_input();

    let mut input = map_by_kind(daya);

    for v in input.values_mut() {
        sort_by_position(v);
    }

    let mut res = 0;
    let mut index = 0;

    for r in RANKS {
        let mut def = vec![];
        let cards = input.get_mut(&r).unwrap_or(&mut def);
        for (_cards, bid) in cards.into_iter() {
            index = index + 1;
            res += *bid * index;
            // println!("{index} {cards} {bid}");
        }
    }
    println!("Part1: {res}");
}

fn map_by_kind(hands: Vec<(String, i64)>) -> HashMap<i64, Vec<(String, i64)>> {
    let mut res: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

    for (cards, bid) in hands.iter() {
        let mut map: HashMap<char, i64> = HashMap::new();
        for h in cards.chars() {
            map.entry(h).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut d = map.values().collect::<Vec<&i64>>();
        d.sort();

        let mut v = 0;
        for d in d {
            v = v * 10 + d
        }

        res.entry(v)
            .and_modify(|x| x.push((cards.to_string(), *bid)))
            .or_insert(vec![(cards.to_string(), *bid)]);
    }
    res
}

fn sort_by_position(hands: &mut Vec<(String, i64)>) {
    let sort = |(card, _): &(String, _)| {
        let mut d: [u64; 5] = [0; 5];
        for (i, c) in card.chars().enumerate() {
            d[i] = 13 - CARDS.iter().position(|x| *x == c).unwrap() as u64;
        }

        let mut v = 0;
        for d in d {
            if d / 10 < 1 {
                v = v * 100 + d
            } else {
                v = v * 100 + d
            }
        }

        v
    };

    hands.sort_by_key(sort);
}
