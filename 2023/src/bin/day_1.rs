use std::fs;

fn part_1() {
    let data = fs::read_to_string("res/day_1.txt").unwrap();
    let lines = data.split('\n');
    let sum: u32 = lines
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();

    println!("Part 1: {sum}");
}

fn part_2() {
    let data = fs::read_to_string("res/day_1.txt").unwrap();
    let lines = data.split('\n');
    let sum: u32 = lines
        .map(|line| {
            let mut numbers: Vec<u32> = Vec::new();
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                match c {
                    'o' if line.len() > i + 2 && chars[i + 1..i + 3] == ['n', 'e'] => {
                        numbers.push(1);
                    }
                    't' if line.len() > i + 2 && chars[i + 1..i + 3] == ['w', 'o'] => {
                        numbers.push(2);
                    }
                    't' if line.len() > i + 4 && chars[i + 1..i + 5] == ['h', 'r', 'e', 'e'] => {
                        numbers.push(3);
                    }
                    'f' if line.len() > i + 3 && chars[i + 1..i + 4] == ['o', 'u', 'r'] => {
                        numbers.push(4);
                    }
                    'f' if line.len() > i + 3 && chars[i + 1..i + 4] == ['i', 'v', 'e'] => {
                        numbers.push(5);
                    }
                    's' if line.len() > i + 2 && chars[i + 1..i + 3] == ['i', 'x'] => {
                        numbers.push(6);
                    }
                    's' if line.len() > i + 4 && chars[i + 1..i + 5] == ['e', 'v', 'e', 'n'] => {
                        numbers.push(7);
                    }
                    'e' if line.len() > i + 4 && chars[i + 1..i + 5] == ['i', 'g', 'h', 't'] => {
                        numbers.push(8);
                    }
                    'n' if line.len() > i + 3 && chars[i + 1..i + 4] == ['i', 'n', 'e'] => {
                        numbers.push(9);
                    }
                    c if c.is_ascii_digit() => numbers.push(c.to_digit(10).unwrap()),
                    _ => {}
                }
                i += 1;
            }

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            first * 10 + last
        })
        .sum();

    println!("Part 2: {sum}");
}

fn main() {
    part_1();
    part_2();
}
