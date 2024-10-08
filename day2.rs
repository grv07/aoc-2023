use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Range;
use std::str::FromStr;

struct GameRange {
    red: Range<i32>,
    green: Range<i32>,
    blue: Range<i32>,
}

#[derive(Default, Debug)]
struct Play {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct PlayParseError;

impl FromStr for Play {
    type Err = PlayParseError;

    fn from_str(s: &str) -> Result<Self, PlayParseError> {
        // 1 red, 2 green, 6 blue
        let s = s.trim().split(',');
        let play = s.fold(Play::default(), |mut acc, v| {
            if v.ends_with("red") {
                let v = v.replace("red", "");
                acc.red = v.trim().parse::<i32>().unwrap();
            }
            if v.ends_with("blue") {
                let v = v.replace("blue", "");
                acc.blue = v.trim().parse::<i32>().unwrap();
            }
            if v.ends_with("green") {
                let v = v.replace("green", "");
                acc.green = v.trim().parse::<i32>().unwrap();
            }
            acc
        });

        Ok(play)
    }
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    plays: Vec<Play>,
}

#[derive(Debug)]
struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Game, GameParseError> {
        let mut game = Self::default();
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut split = s.split(':');
        let (left, right) = (split.next().unwrap(), split.next().unwrap());

        if left.starts_with("Game") {
            let id = left.replace("Game ", "");
            // println!("Id: {id}");
            game.id = id.trim().parse().unwrap();
        }

        let plays = right
            .split(";")
            .map(|v| Play::from_str(v).unwrap())
            .collect::<Vec<Play>>();
        game.plays.extend(plays);

        Ok(game)
    }
}

fn get_file_name() -> String {
    let mut args = std::env::args();
    let _bin_name = args.next();

    let file_name = args.next().expect("Usage: Input file name is empty");

    file_name
}

fn main() {
    let file_name = get_file_name();
    println!("Input File Name: {file_name:?}");

    let lines = BufReader::new(File::open(file_name).expect("Error: File not found")).lines();
    get_possible_games(lines);

    let input = "34 green, 69 red";
    let play = Play::from_str(input);
    println!("{play:?}");

    // let game_i = "Game 18: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    // let game = Game::from_str(game_i);
    // println!("{game:?}");
}

fn get_possible_games(lines: Lines<BufReader<File>>) {
    let game_range = GameRange {
        red: Range {
            start: 0,
            end: 12 + 1,
        },
        green: Range {
            start: 0,
            end: 13 + 1,
        },
        blue: Range {
            start: 0,
            end: 14 + 1,
        },
    };

    let mut sum = 0;
    for line in lines {
        // println!("{}", line.unwrap());
        let game = Game::from_str(&line.unwrap()).unwrap();
        let valid_game = game.plays.iter().all(|play| {
            game_range.red.contains(&play.red)
                && game_range.blue.contains(&play.blue)
                && game_range.green.contains(&play.green)
        });

        if valid_game {
            sum += game.id;
        }

        println!("{game:?}");
    }

    println!("{sum}");
}
