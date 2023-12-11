use std::fs;

use lazy_regex::regex;

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn parse_game(line: &str) -> Game {
    let game_number_re = regex!("Game (?<id>\\d+):(?<pulls>.*)");
    let red_re = regex!("(?<red>\\d+) red");
    let green_re = regex!("(?<green>\\d+) green");
    let blue_re = regex!("(?<blue>\\d+) blue");

    let capture = game_number_re.captures(line).unwrap();
    let game_number = capture.name("id").unwrap().as_str();
    let pulls = capture.name("pulls").unwrap().as_str();
    let pulls = pulls.split(';').map(|pull| {
        let red: u32 = red_re
            .captures(pull)
            .map_or(0, |r| r.name("red").unwrap().as_str().parse().unwrap());
        let green: u32 = green_re
            .captures(pull)
            .map_or(0, |r| r.name("green").unwrap().as_str().parse().unwrap());
        let blue: u32 = blue_re
            .captures(pull)
            .map_or(0, |r| r.name("blue").unwrap().as_str().parse().unwrap());

        Pull { red, blue, green }
    });

    Game {
        id: str::parse(game_number).unwrap(),
        pulls: pulls.collect(),
    }
}

fn part_1() {
    let data = fs::read_to_string("res/day_2.txt").unwrap();
    let lines = data.split('\n');

    let sum: u32 = lines
        .map(parse_game)
        .filter(|game| {
            !game
                .pulls
                .iter()
                .any(|pull| pull.red > MAX_RED || pull.green > MAX_GREEN || pull.blue > MAX_BLUE)
        })
        .map(|game| game.id)
        .sum();

    println!("Part 1: {:?}", sum);
}

fn part_2() {
    let data = fs::read_to_string("res/day_2.txt").unwrap();
    let lines = data.split('\n');

    let sum: u32 = lines
        .map(parse_game)
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            game.pulls.iter().for_each(|pull| {
                if pull.red > max_red {
                    max_red = pull.red;
                }
                if pull.green > max_green {
                    max_green = pull.green;
                }
                if pull.blue > max_blue {
                    max_blue = pull.blue;
                }
            });

            max_red * max_green * max_blue
        })
        .sum();

    println!("Part 2: {:?}", sum);
}

fn main() {
    part_1();
    part_2();
}
