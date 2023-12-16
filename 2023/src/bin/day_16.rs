use std::{collections::HashSet, fs, thread};

#[derive(Debug, Clone, Copy)]
enum CellType {
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_type: CellType,
    energized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    position: (usize, usize),
    direction: Direction,
}

type Grid = Vec<Vec<Cell>>;

#[allow(dead_code)]
fn draw_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            let char = match cell.cell_type {
                CellType::RightMirror => '/',
                CellType::LeftMirror => '\\',
                CellType::VerticalSplitter => '|',
                CellType::HorizontalSplitter => '-',
                CellType::None => '·',
            };

            if cell.energized {
                print!("\x1b[0;41m{char}\x1b[0m");
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    println!();
}

fn go_north(queue: &mut Vec<Vector>, vector: &Vector) {
    if vector.position.1 != 0 {
        queue.push(Vector {
            position: (vector.position.0, vector.position.1 - 1),
            direction: Direction::North,
        });
    }
}

fn go_south(grid: &Grid, queue: &mut Vec<Vector>, vector: &Vector) {
    if vector.position.1 != grid.len() - 1 {
        queue.push(Vector {
            position: (vector.position.0, vector.position.1 + 1),
            direction: Direction::South,
        });
    }
}

fn go_east(grid: &Grid, queue: &mut Vec<Vector>, vector: &Vector) {
    if vector.position.0 != grid[0].len() - 1 {
        queue.push(Vector {
            position: (vector.position.0 + 1, vector.position.1),
            direction: Direction::East,
        });
    }
}

fn go_west(queue: &mut Vec<Vector>, vector: &Vector) {
    if vector.position.0 != 0 {
        queue.push(Vector {
            position: (vector.position.0 - 1, vector.position.1),
            direction: Direction::West,
        });
    }
}

fn parse_grid(input: &str) -> Grid {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|char| {
                    let cell_type = match char {
                        '/' => CellType::RightMirror,
                        '\\' => CellType::LeftMirror,
                        '|' => CellType::VerticalSplitter,
                        '-' => CellType::HorizontalSplitter,
                        '.' => CellType::None,
                        _ => panic!("Invalid cell type"),
                    };
                    Cell {
                        cell_type,
                        energized: false,
                    }
                })
                .collect()
        })
        .collect()
}

fn run(mut grid: Grid, starting_vector: Vector) -> usize {
    let mut queue = vec![starting_vector];
    let mut visited = HashSet::new();

    while let Some(vector) = queue.pop() {
        if visited.contains(&vector) {
            continue;
        } else {
            visited.insert(vector);
        }

        let cell = grid[vector.position.1][vector.position.0];
        grid[vector.position.1][vector.position.0].energized = true;
        match cell.cell_type {
            CellType::RightMirror => match vector.direction {
                Direction::North => {
                    go_east(&grid, &mut queue, &vector);
                }
                Direction::South => {
                    go_west(&mut queue, &vector);
                }
                Direction::East => {
                    go_north(&mut queue, &vector);
                }
                Direction::West => {
                    go_south(&grid, &mut queue, &vector);
                }
            },
            CellType::LeftMirror => match vector.direction {
                Direction::North => {
                    go_west(&mut queue, &vector);
                }
                Direction::South => {
                    go_east(&grid, &mut queue, &vector);
                }
                Direction::East => {
                    go_south(&grid, &mut queue, &vector);
                }
                Direction::West => {
                    go_north(&mut queue, &vector);
                }
            },
            CellType::VerticalSplitter => match vector.direction {
                Direction::North => go_north(&mut queue, &vector),
                Direction::South => go_south(&grid, &mut queue, &vector),
                Direction::East | Direction::West => {
                    go_north(&mut queue, &vector);
                    go_south(&grid, &mut queue, &vector);
                }
            },
            CellType::HorizontalSplitter => match vector.direction {
                Direction::East => go_east(&grid, &mut queue, &vector),
                Direction::West => go_west(&mut queue, &vector),
                Direction::North | Direction::South => {
                    go_east(&grid, &mut queue, &vector);
                    go_west(&mut queue, &vector);
                }
            },
            CellType::None => match vector.direction {
                Direction::North => go_north(&mut queue, &vector),
                Direction::South => go_south(&grid, &mut queue, &vector),
                Direction::East => go_east(&grid, &mut queue, &vector),
                Direction::West => go_west(&mut queue, &vector),
            },
        }
    }

    grid.iter().flatten().filter(|cell| cell.energized).count()
}

fn part_1() {
    let data = fs::read_to_string("res/day_16.txt").unwrap();

    let grid = parse_grid(&data);
    let starting_vector = Vector {
        position: (0, 0),
        direction: Direction::East,
    };
    let energized_cells = run(grid, starting_vector);

    println!("Part 1: {energized_cells}");
}

fn part_2() {
    let data = fs::read_to_string("res/day_16.txt").unwrap();

    let grid = parse_grid(&data);

    let mut threads = Vec::with_capacity(grid.len() + grid[0].len());

    for (x, _) in grid[0].iter().enumerate() {
        let grid = grid.clone();
        let thread = thread::spawn(move || {
            let north_run = run(
                grid.clone(),
                Vector {
                    position: (x, grid.len() - 1),
                    direction: Direction::North,
                },
            );

            let south_run = run(
                grid.clone(),
                Vector {
                    position: (x, 0),
                    direction: Direction::South,
                },
            );

            if south_run > north_run {
                return south_run;
            }

            north_run
        });
        threads.push(thread);
    }

    for (y, _) in grid.iter().enumerate() {
        let grid = grid.clone();
        let thread = thread::spawn(move || {
            let east_run = run(
                grid.clone(),
                Vector {
                    position: (0, y),
                    direction: Direction::East,
                },
            );

            let west_run = run(
                grid.clone(),
                Vector {
                    position: (grid[0].len() - 1, y),
                    direction: Direction::West,
                },
            );

            if west_run > east_run {
                return west_run;
            }

            east_run
        });
        threads.push(thread);
    }

    let max_energized = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .max()
        .unwrap();

    println!("Part 2: {max_energized}");
}

fn main() {
    part_1();
    part_2();
}
