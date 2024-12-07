use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDSJ: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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
    let data = get_parsed_input();

    let mut input1 = map_by_kind(&data, false);
    let mut input2 = map_by_kind(&data, true);

    for v in input1.values_mut() {
        sort_by_position(v, false);
    }

    for v in input2.values_mut() {
        sort_by_position(v, true);
    }

    let mut res1 = 0;
    let mut res2 = 0;
    let mut index = 0;

    for r in RANKS {
        let mut def = vec![];
        let cards = input1.get_mut(&r).unwrap_or(&mut def);

        for (_cards, bid) in cards.into_iter() {
            index = index + 1;
            res1 += *bid * index;
            // println!("{index} {cards} {bid}");
        }
    }

    index = 0;
    for r in RANKS {
        let mut def = vec![];
        let cards = input2.get_mut(&r).unwrap_or(&mut def);

        for (cards, bid) in cards.into_iter() {
            index = index + 1;
            res2 += *bid * index;
            println!("{index} {cards} {bid}");
        }
    }

    println!("Part1: {res1}");
    println!("Part2: {res2}");
}

fn map_by_kind(hands: &Vec<(String, i64)>, jocker: bool) -> HashMap<i64, Vec<(String, i64)>> {
    let mut res: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

    for (cards, bid) in hands.iter() {
        let mut map: HashMap<char, i64> = HashMap::new();
        for h in cards.chars() {
            map.entry(h).and_modify(|x| *x += 1).or_insert(1);
        }

        let d = if jocker {
            let j_count = map.remove(&'J').unwrap_or_default();
            let mut d = map.values().map(|v| v.to_owned()).collect::<Vec<i64>>();

            d.sort();

            if let Some(last) = d.last_mut() {
                *last += j_count;
            }

            if d.is_empty() {
                d.push(5);
            }

            d
        } else {
            let mut d = map.values().map(|v| v.to_owned()).collect::<Vec<i64>>();
            d.sort();
            d
        };

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

fn sort_by_position(hands: &mut Vec<(String, i64)>, jocker: bool) {
    let sort = |(card, _): &(String, _)| {
        let mut d: [u64; 5] = [0; 5];
        for (i, c) in card.chars().enumerate() {
            let mut cards = if jocker { CARDSJ.iter() } else { CARDS.iter() };
            d[i] = 13 - cards.position(|x| *x == c).unwrap() as u64;
        }

        let mut v = 0;
        for d in d {
            v = v * 100 + d
        }

        v
    };

    hands.sort_by_key(sort);
}
