use std::fs;

fn get_hash(label: &str) -> u8 {
    let mut hash = 0;
    for char in label.chars() {
        hash += char as u8 as u32;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}

fn part_1() {
    let data = fs::read_to_string("res/day_15.txt").unwrap();
    let parts = data.split(',');

    let sum: usize = parts.map(|part| get_hash(part) as usize).sum();

    println!("Part 1: {}", sum);
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

fn part_2() {
    let data = fs::read_to_string("res/day_15.txt").unwrap();
    let steps = data.split(',');

    let mut boxes = vec![Vec::new(); 256];

    for step in steps {
        if step.ends_with('-') {
            let label = &step[..step.len() - 1];
            let hash = get_hash(label);
            if let Some(lens_index) = boxes[hash as usize]
                .iter()
                .position(|lens: &Lens| lens.label == label)
            {
                boxes[hash as usize].remove(lens_index);
            }
        } else if let Some((label, focal_length)) = step.split_once('=') {
            let focal_length = focal_length.parse().unwrap();
            let hash = get_hash(label);
            if let Some(lens_index) = boxes[hash as usize]
                .iter()
                .position(|lens| lens.label == label)
            {
                boxes[hash as usize][lens_index].focal_length = focal_length;
            } else {
                boxes[hash as usize].push(Lens {
                    label,
                    focal_length,
                });
            }
        }
    }

    let mut sum: usize = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (j, lens) in box_.iter().enumerate() {
            sum += (i + 1) * (j + 1) * lens.focal_length as usize;
        }
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part_1();
    part_2();
}
