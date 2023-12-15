use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rock {
    Round,
    Cube,
    None,
}

type Grid = Vec<Vec<Rock>>;

#[allow(dead_code)]
fn draw_grid(grid: &Grid) {
    for row in grid.iter() {
        for col in row.iter() {
            match col {
                Rock::Round => print!("O"),
                Rock::Cube => print!("#"),
                Rock::None => print!("."),
            }
        }
        println!()
    }
}

fn part_1() {
    let data = fs::read_to_string("res/day_14.txt").unwrap();

    let grid: Grid = data
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::None,
                    _ => panic!("Invalid rock type"),
                })
                .collect()
        })
        .collect();

    let rotated_grid: Grid = (0..grid[0].len())
        .map(|y| grid.iter().map(|x| x[y]).collect())
        .collect();

    let mut ordered_rotated_grid: Grid = Vec::new();
    for col in rotated_grid {
        let mut new_col: Vec<Rock> = col
            .split(|rock| rock == &Rock::Cube)
            .flat_map(|rocks| {
                let mut rocks = rocks.to_vec();
                rocks.sort();
                rocks.push(Rock::Cube);
                rocks
            })
            .collect();
        new_col.pop();
        ordered_rotated_grid.push(new_col);
    }

    let ordered_grid: Grid = (0..ordered_rotated_grid[0].len())
        .map(|y| ordered_rotated_grid.iter().map(|x| x[y]).collect())
        .collect();

    let mut sum: usize = 0;
    for (i, row) in ordered_grid.iter().rev().enumerate() {
        sum += row.iter().filter(|rock| rock == &&Rock::Round).count() * (i + 1);
    }

    println!("Part 1: {sum}");
}

fn flip_grid(grid: Grid) -> Grid {
    (0..grid[0].len())
        .map(|y| grid.iter().map(|x| x[y]).collect())
        .collect()
}

fn order_grid(grid: Grid) -> Grid {
    let mut ordered_grid: Grid = Vec::new();
    for col in grid {
        let mut new_col: Vec<Rock> = col
            .split(|rock| rock == &Rock::Cube)
            .flat_map(|rocks| {
                let mut rocks = rocks.to_vec();
                rocks.sort();
                rocks.push(Rock::Cube);
                rocks
            })
            .collect();
        new_col.pop();
        ordered_grid.push(new_col);
    }
    ordered_grid
}

fn reverse_grid(grid: Grid) -> Grid {
    let mut ordered_grid: Grid = Vec::new();
    for mut col in grid {
        col.reverse();
        let mut new_col: Vec<Rock> = col
            .split(|rock| rock == &Rock::Cube)
            .flat_map(|rocks| {
                let mut rocks = rocks.to_vec();
                rocks.sort();
                rocks.push(Rock::Cube);
                rocks
            })
            .collect();
        new_col.pop();
        new_col.reverse();
        ordered_grid.push(new_col);
    }
    ordered_grid
}

fn part_2() {
    let data = fs::read_to_string("res/day_14.txt").unwrap();

    let mut grid: Grid = data
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::None,
                    _ => panic!("Invalid rock type"),
                })
                .collect()
        })
        .collect();

    let mut cache = HashMap::new();
    let mut cycle_detected = false;
    let mut i: usize = 0;

    while i < 1_000_000_000 {
        grid = flip_grid(grid);
        grid = order_grid(grid);
        grid = flip_grid(grid);
        grid = order_grid(grid);
        grid = flip_grid(grid);
        grid = reverse_grid(grid);
        grid = flip_grid(grid);
        grid = reverse_grid(grid);

        i += 1;
        if !cycle_detected {
            if let Some(cycle) = cache.get(&grid) {
                cycle_detected = true;
                let cycle_size = i - cycle;
                let remaining_steps = 1_000_000_000 - i;
                let remaining_full_steps = remaining_steps / cycle_size;

                i += remaining_full_steps * cycle_size;
                continue;
            } else {
                cache.insert(grid.clone(), i);
            }
        }
    }

    let mut sum: usize = 0;
    for (i, row) in grid.iter().rev().enumerate() {
        sum += row.iter().filter(|rock| rock == &&Rock::Round).count() * (i + 1);
    }

    println!("Part 2: {sum}");
}

fn main() {
    part_1();
    part_2();
}
