use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" | "3" => Direction::Up,
            "D" | "1" => Direction::Down,
            "L" | "2" => Direction::Left,
            "R" | "0" => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

fn get_area(instructions: &[Instruction]) -> usize {
    let mut vertices = Vec::with_capacity(instructions.len());
    let mut current_x = 0;
    let mut current_y = 0;
    let mut perimeter = 0;
    for instruction in instructions {
        vertices.push((current_x, current_y));
        match instruction.direction {
            Direction::Up => current_y -= instruction.distance as isize,
            Direction::Down => current_y += instruction.distance as isize,
            Direction::Left => current_x -= instruction.distance as isize,
            Direction::Right => current_x += instruction.distance as isize,
        }
        perimeter += instruction.distance;
    }

    let last_vertex = vertices.len() - 1;
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for i in 0..last_vertex {
        sum_1 += vertices[i].0 * vertices[i + 1].1;
        sum_2 += vertices[i].1 * vertices[i + 1].0;
    }

    sum_1 += vertices[last_vertex].0 * vertices[0].1;
    sum_2 += vertices[0].0 * vertices[last_vertex].1;

    let area = (sum_1 - sum_2).unsigned_abs() / 2;

    area + (perimeter / 2) + 1
}

fn part_1() {
    let data = fs::read_to_string("res/day_18.txt").unwrap();
    let instructions: Vec<Instruction> = data
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let direction: Direction = parts.next().unwrap().into();
            let distance: usize = parts.next().unwrap().parse().unwrap();
            Instruction {
                direction,
                distance,
            }
        })
        .collect();

    let area = get_area(&instructions);

    println!("Part 1: {area}");
}

fn part_2() {
    let data = fs::read_to_string("res/day_18.txt").unwrap();

    let instructions: Vec<Instruction> = data
        .lines()
        .map(|line| {
            let (_, data) = line.split_once('#').unwrap();
            let distance = usize::from_str_radix(&data[0..5], 16).unwrap();
            let direction: Direction = data[5..6].into();
            Instruction {
                direction,
                distance,
            }
        })
        .collect();

    let total_area = get_area(&instructions);

    println!("Part 2: {total_area}");
}

fn main() {
    part_1();
    part_2();
}
