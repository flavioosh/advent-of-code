use std::fs;

fn get_diffs(input: &Vec<isize>) -> Vec<isize> {
    let mut output = Vec::with_capacity(input.len() - 1);
    for i in 0..input.len() - 1 {
        output.push(input[i + 1] - input[i]);
    }
    output
}

fn part_1() {
    let data = fs::read_to_string("res/day_9.txt").unwrap();
    let lines = data.split('\n');

    let result: isize = lines
        .map(|line| {
            let reading = line
                .split_ascii_whitespace()
                .map(|reading| str::parse(reading).unwrap())
                .collect();
            let mut differences: Vec<Vec<isize>> = vec![reading];
            while differences.is_empty() || differences.last().unwrap().iter().any(|d| *d != 0) {
                differences.push(get_diffs(differences.last().unwrap()));
            }
            let next_value: isize = differences.iter().map(|step| step.last().unwrap()).sum();
            next_value
        })
        .sum();

    println!("Part 1: {result}");
}

fn part_2() {
    let data = fs::read_to_string("res/day_9.txt").unwrap();
    let lines = data.split('\n');

    let result: isize = lines
        .map(|line| {
            let mut reading: Vec<isize> = line
                .split_ascii_whitespace()
                .map(|reading| str::parse(reading).unwrap())
                .collect();
            reading.reverse();
            let mut differences: Vec<Vec<isize>> = vec![reading];
            while differences.is_empty() || differences.last().unwrap().iter().any(|d| *d != 0) {
                differences.push(get_diffs(differences.last().unwrap()));
            }
            let next_value: isize = differences.iter().map(|step| step.last().unwrap()).sum();
            next_value
        })
        .sum();

    println!("Part 2: {result}");
}

fn main() {
    part_1();
    part_2();
}
