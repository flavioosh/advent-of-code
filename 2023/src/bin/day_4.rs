use std::{fs, collections::HashSet};

fn main() {
    let data = fs::read_to_string("res/day_4.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();
    let mut wins = vec![1u32; lines.len()];

    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        let l: Vec<&str> = line.split(':').collect();
        let l: Vec<&str> = l[1].trim().split('|').collect();
        let winning = l[0].trim();
        let numbers = l[1].trim();

        let winning = winning.split_ascii_whitespace();
        let numbers = numbers.split_ascii_whitespace();

        let winning: HashSet<&str> = HashSet::from_iter(winning);
        let numbers: HashSet<&str> = HashSet::from_iter(numbers);

        let result: Vec<&&str> = winning.intersection(&numbers).into_iter().collect();
        let matches = result.len();

        if matches > 0 {
            let result = 2u32.pow(result.len() as u32 - 1);
            sum_1 += result;
        }

        for j in 0..matches {
            wins[i + 1 + j] += wins[i];
        }

        sum_2 += wins[i];
    }

    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}
