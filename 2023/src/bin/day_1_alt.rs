use std::fs;

fn part_1() {
    let data = fs::read_to_string("res/day_1.txt").unwrap();
    let lines = data.split('\n');
    let mut sum: u32 = 0;
    for line in lines {
        let mut first: u32 = 10;
        let mut last: u32 = 0;
        for char in line.chars() {
            if char.is_ascii_digit() {
                last = char.to_digit(10).unwrap();
                if first == 10 {
                    first = last;
                }
            }
        }
        sum += first * 10 + last;
    }

    println!("Part 1: {}", sum);
}

fn part_2() {
    let data = fs::read_to_string("res/day_1.txt").unwrap();
    let lines = data.split('\n');
    let sum: u32 = lines
        .map(|line| {
            let mut first: u32 = 10;
            let mut last: u32 = 0;
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                match c {
                    'o' if line.len() > i + 2 && chars[i + 1..i + 3] == ['n', 'e'] => {
                        last = 1;
                        if first == 10 {
                            first = 1;
                        }
                    }
                    't' if line.len() > i + 2 && chars[i + 1..i + 3] == ['w', 'o'] => {
                        last = 2;
                        if first == 10 {
                            first = 2;
                        }
                    }
                    't' if line.len() > i + 4 && chars[i + 1..i + 5] == ['h', 'r', 'e', 'e'] => {
                        last = 3;
                        if first == 10 {
                            first = 3;
                        }
                    }
                    'f' if line.len() > i + 3 && chars[i + 1..i + 4] == ['o', 'u', 'r'] => {
                        last = 4;
                        if first == 10 {
                            first = 4;
                        }
                    }
                    'f' if line.len() > i + 3 && chars[i + 1..i + 4] == ['i', 'v', 'e'] => {
                        last = 5;
                        if first == 10 {
                            first = 5;
                        }
                    }
                    's' if line.len() > i + 2 && chars[i + 1..i + 3] == ['i', 'x'] => {
                        last = 6;
                        if first == 10 {
                            first = 6;
                        }
                    }
                    's' if line.len() > i + 4 && chars[i + 1..i + 5] == ['e', 'v', 'e', 'n'] => {
                        last = 7;
                        if first == 10 {
                            first = 7;
                        }
                    }
                    'e' if line.len() > i + 4 && chars[i + 1..i + 5] == ['i', 'g', 'h', 't'] => {
                        last = 8;
                        if first == 10 {
                            first = 8;
                        }
                    }
                    'n' if line.len() > i + 3 && chars[i + 1..i + 4] == ['i', 'n', 'e'] => {
                        last = 9;
                        if first == 10 {
                            first = 9;
                        }
                    }
                    c if c.is_ascii_digit() => {
                        last = c.to_digit(10).unwrap();
                        if first == 10 {
                            first = last;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }

            first * 10 + last
        })
        .sum();

    println!("Part 2: {}", sum);
}

fn main() {
    part_1();
    part_2();
}
