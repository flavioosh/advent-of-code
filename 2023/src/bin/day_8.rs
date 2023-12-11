use std::{str, collections::BTreeMap, fs};

#[derive(Debug)]
struct Node {
    name: usize,
    directions: [usize; 2],
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let name = value[0..3].as_bytes();
        let left = value[7..10].as_bytes();
        let right = value[12..15].as_bytes();

        let name = (name[0] as usize) << 16 | (name[1] as usize) << 8 | (name[2] as usize);
        let left = (left[0] as usize) << 16 | (left[1] as usize) << 8 | (left[2] as usize);
        let right = (right[0] as usize) << 16 | (right[1] as usize) << 8 | (right[2] as usize);

        Self {
            name,
            directions: [left, right],
        }
    }
}

fn part_1() {
    let data = fs::read_to_string("res/day_8.txt").unwrap();
    let mut sections = data.split("\n\n");

    let instructions: Vec<u8> = sections
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => 0u8,
            'R' => 1u8,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let nodes = sections
        .next()
        .unwrap()
        .split('\n')
        .map(Node::from)
        .map(|node| (node.name, node.directions));

    let map = BTreeMap::from_iter(nodes);

    let starting_location = ('A' as usize) << 16 | ('A' as usize) << 8 | ('A' as usize);
    let ending_location = ('Z' as usize) << 16 | ('Z' as usize) << 8 | ('Z' as usize);

    let mut instruction_index: usize = 0;
    let mut current_location = starting_location;
    let mut steps: usize = 0;
    while current_location != ending_location {
        current_location = map.get(&current_location).unwrap()[instructions[instruction_index] as usize];
        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }
        steps += 1;
    }

    println!("Part 1: {}", steps);
}

fn part_2() {
    let data = fs::read_to_string("res/day_8.txt").unwrap();
    let mut sections = data.split("\n\n");

    let instructions: Vec<u8> = sections
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => 0u8,
            'R' => 1u8,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let nodes = sections
        .next()
        .unwrap()
        .split('\n')
        .map(Node::from)
        .map(|node| (node.name, node.directions));

    let nodes: Vec<(usize, [usize; 2])> = nodes.collect();

    let mut starting_nodes = Vec::new();
    for (name, _) in nodes.iter() {
        if name & 0xFF == ('A' as usize) {
            starting_nodes.push(*name);
        }
    }

    let map = BTreeMap::from_iter(nodes);

    let mut instruction_index: usize = 0;
    let mut steps: usize = 0;

    let mut current_locations = starting_nodes.clone();
    let mut finished = vec![0; current_locations.len()];

    print!("Start: ");
    for l in current_locations.iter() {
        print!("{}, ", to_chars(*l));
    }
    println!();
    while finished.iter().any(|f| *f == 0) {
        for i in 0..current_locations.len() {
            if finished[i] != 0 {
                continue;
            }
            current_locations[i] = map.get(&current_locations[i]).unwrap()[instructions[instruction_index] as usize];
            if current_locations[i] & ('Z' as usize) == 'Z' as usize {
                finished[i] = steps + 1;
                println!("Hit Z ({}) for {} as step {}", to_chars(current_locations[i]), to_chars(starting_nodes[i]), steps);
            }
        }
        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }
        steps += 1;
    }

    println!("Part 2: {}", lcm(finished));
}

fn to_chars(input: usize) -> String {
    format!("{}{}{}", (input >> 16) as u8 as char, (input >> 8 & 0xFF) as u8 as char, (input & 0xFF) as u8 as char)
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

fn lcm(input: Vec<usize>) -> usize {
    let mut result = input[0];

    for i in input {
        result = (i * result) / gcd(i, result);
    }

    result
}

fn main() {
    part_1();
    part_2();
}
