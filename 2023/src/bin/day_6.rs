use std::fs;

fn part_1() {
    let data = fs::read_to_string("res/day_6.txt").unwrap();
    let mut lines = data.split('\n');
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();

    let accumulator: usize = times
        .into_iter()
        .enumerate()
        .map(|(i, time)| {
            let distance = distances[i];

            let a = -1 as f64;
            let b = time as f64;
            let c = -(distance as f64);

            let x1 = (-b + (b.powf(2.0) - 4.0 * a * c).sqrt()) / 2.0 * a;
            let x2 = (-b - (b.powf(2.0) - 4.0 * a * c).sqrt()) / 2.0 * a;

            let x1 = x1.ceil() as usize;
            let x2 = x2.ceil() as usize;

            x2 - x1
        })
        .product();

    println!("Part 1: {}", accumulator);
}

fn part_2() {
    let data = fs::read_to_string("res/day_6.txt").unwrap();
    let mut lines = data.split('\n');
    let time: usize = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: usize = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(' ', "")
        .parse()
        .unwrap();

    let mut wins = 0;

    let a = -1 as f64;
    let b = time as f64;
    let c = -(distance as f64 + 1.0);

    let x1 = (-b + (b.powf(2.0) - 4.0 * a * c).sqrt()) / 2.0 * a;
    let x2 = (-b - (b.powf(2.0) - 4.0 * a * c).sqrt()) / 2.0 * a;

    let x1 = x1.floor() as usize;
    let x2 = x2.ceil() as usize;

    wins += x2 - x1;

    println!("Part 2: {}", wins);
}

fn main() {
    part_1();
    part_2();
}
