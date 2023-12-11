use std::fs;

use lazy_regex::regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn part_1() {
    let data = fs::read_to_string("res/day_2.txt").unwrap();
    let lines = data.split('\n');

    let game_number_re = regex!("Game (?<id>\\d+):.*");
    let red_re = regex!("(?<red>\\d+) red");
    let green_re = regex!("(?<green>\\d+) green");
    let blue_re = regex!("(?<blue>\\d+) blue");

    let sum: u32 = lines
        .filter(|line| {
            let red: u32 = red_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("red")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();

            if red > MAX_RED {
                return false;
            }
            let green: u32 = green_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("green")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();

            if green > MAX_GREEN {
                return false;
            }
            let blue: u32 = blue_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("blue")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();

            if blue > MAX_BLUE {
                return false;
            }

            true
        })
        .map(|line| {
            game_number_re
                .captures(line)
                .unwrap()
                .name("id")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("Part 1: {:?}", sum);
}

fn part_2() {
    let data = fs::read_to_string("res/day_2.txt").unwrap();
    let lines = data.split('\n');

    let red_re = regex!("(?<red>\\d+) red");
    let green_re = regex!("(?<green>\\d+) green");
    let blue_re = regex!("(?<blue>\\d+) blue");

    let sum: u32 = lines
        .map(|line| {
            let red: u32 = red_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("red")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();
            let green: u32 = green_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("green")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();
            let blue: u32 = blue_re
                .captures_iter(line)
                .map(|capture| {
                    capture
                        .name("blue")
                        .map_or(0, |c| c.as_str().parse().unwrap())
                })
                .max()
                .unwrap();

            red * green * blue
        })
        .sum();

    println!("Part 1: {:?}", sum);
}

fn main() {
    part_1();
    part_2();
}
