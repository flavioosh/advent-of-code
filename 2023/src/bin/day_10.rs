use std::{collections::BTreeSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Grid = Vec<Vec<Pipe>>;
type Position = (usize, usize);
type Vector = (Position, Direction);

#[allow(dead_code)]
fn print_grid(
    grid: &Grid,
    highlighted_pipes: Option<&BTreeSet<Position>>,
    inside_pipes: Option<&Vec<Position>>,
) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let mut is_highlighted = false;
            if let Some(highlighted_pipes) = highlighted_pipes {
                if highlighted_pipes.contains(&(x, y)) {
                    is_highlighted = true;
                }
            }
            let mut is_inside = false;
            if let Some(inside_pipes) = inside_pipes {
                if inside_pipes.contains(&(x, y)) {
                    is_inside = true;
                }
            }
            let mut highlight_start = "";
            if is_highlighted {
                highlight_start = "\x1b[0;31m";
            }
            if is_inside {
                highlight_start = "\x1b[0;34m";
            }
            let c = match *col {
                Pipe::NorthSouth => '┃',
                Pipe::EastWest => '━',
                Pipe::NorthEast => '┗',
                Pipe::NorthWest => '┛',
                Pipe::SouthWest => '┓',
                Pipe::SouthEast => '┏',
                Pipe::Start => 'S',
                Pipe::Ground => '.',
            };
            let mut highlight_end = "";
            if is_highlighted || is_inside {
                highlight_end = "\x1b[0m";
            }
            print!("{}{}{}", highlight_start, c, highlight_end);
        }
        println!();
    }
}

fn next_pipe(grid: &Grid, vector: Vector) -> Vector {
    let pipe_segment = &grid[vector.0 .1][vector.0 .0];
    match vector.1 {
        Direction::North => match pipe_segment {
            Pipe::NorthSouth => ((vector.0 .0, vector.0 .1 + 1), Direction::North),
            Pipe::NorthWest => ((vector.0 .0 - 1, vector.0 .1), Direction::East),
            Pipe::NorthEast => ((vector.0 .0 + 1, vector.0 .1), Direction::West),
            _ => panic!("Invalid north pipe connection at {:?}", vector.0),
        },
        Direction::South => match pipe_segment {
            Pipe::NorthSouth => ((vector.0 .0, vector.0 .1 - 1), Direction::South),
            Pipe::SouthWest => ((vector.0 .0 - 1, vector.0 .1), Direction::East),
            Pipe::SouthEast => ((vector.0 .0 + 1, vector.0 .1), Direction::West),
            _ => panic!("Invalid south pipe connection at {:?}", vector.0),
        },
        Direction::East => match pipe_segment {
            Pipe::EastWest => ((vector.0 .0 - 1, vector.0 .1), Direction::East),
            Pipe::NorthEast => ((vector.0 .0, vector.0 .1 - 1), Direction::South),
            Pipe::SouthEast => ((vector.0 .0, vector.0 .1 + 1), Direction::North),
            _ => panic!("Invalid east pipe connection at {:?}", vector.0),
        },
        Direction::West => match pipe_segment {
            Pipe::EastWest => ((vector.0 .0 + 1, vector.0 .1), Direction::West),
            Pipe::NorthWest => ((vector.0 .0, vector.0 .1 - 1), Direction::South),
            Pipe::SouthWest => ((vector.0 .0, vector.0 .1 + 1), Direction::North),
            _ => panic!("Invalid west pipe connection at {:?}", vector.0),
        },
    }
}

fn main() {
    let data = fs::read_to_string("res/day_10.txt").unwrap();
    let lines = data.split('\n');

    let mut starting_position = (0, 0);
    let grid: Grid = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '|' => Pipe::NorthSouth,
                    '-' => Pipe::EastWest,
                    'L' => Pipe::NorthEast,
                    'J' => Pipe::NorthWest,
                    '7' => Pipe::SouthWest,
                    'F' => Pipe::SouthEast,
                    'S' => {
                        starting_position = (x, y);
                        Pipe::Start
                    }
                    '.' => Pipe::Ground,
                    _ => panic!("Invalid pipe segment"),
                })
                .collect()
        })
        .collect();
    let starting_position = starting_position;

    let mut pipe_1: Option<(Position, Direction)> = None;
    let mut pipe_2: Option<(Position, Direction)> = None;
    if starting_position.1 > 0 {
        let north_pipe = &grid[starting_position.1 - 1][starting_position.0];
        if north_pipe == &Pipe::NorthSouth
            || north_pipe == &Pipe::SouthEast
            || north_pipe == &Pipe::SouthWest
        {
            pipe_1 = Some((
                (starting_position.0, starting_position.1 - 1),
                Direction::South,
            ));
        }
    }
    if starting_position.1 + 1 < grid.len() {
        let south_pipe = &grid[starting_position.1 + 1][starting_position.0];
        if south_pipe == &Pipe::NorthSouth
            || south_pipe == &Pipe::NorthEast
            || south_pipe == &Pipe::NorthWest
        {
            if pipe_1.is_none() {
                pipe_1 = Some((
                    (starting_position.0, starting_position.1 + 1),
                    Direction::North,
                ));
            } else {
                pipe_2 = Some((
                    (starting_position.0, starting_position.1 + 1),
                    Direction::North,
                ));
            }
        }
    }
    if starting_position.0 > 0 {
        let east_pipe = &grid[starting_position.1][starting_position.0 - 1];
        if east_pipe == &Pipe::EastWest
            || east_pipe == &Pipe::NorthEast
            || east_pipe == &Pipe::SouthEast
        {
            if pipe_1.is_none() {
                pipe_1 = Some((
                    (starting_position.0 - 1, starting_position.1),
                    Direction::East,
                ));
            } else {
                pipe_2 = Some((
                    (starting_position.0 - 1, starting_position.1),
                    Direction::East,
                ));
            }
        }
    }
    if starting_position.0 + 1 < grid[starting_position.1].len() {
        let west_pipe = &grid[starting_position.1][starting_position.0 + 1];
        if west_pipe == &Pipe::EastWest
            || west_pipe == &Pipe::NorthWest
            || west_pipe == &Pipe::SouthWest
        {
            if pipe_1.is_none() {
                pipe_1 = Some((
                    (starting_position.0 + 1, starting_position.1),
                    Direction::West,
                ));
            } else {
                pipe_2 = Some((
                    (starting_position.0 + 1, starting_position.1),
                    Direction::West,
                ));
            }
        }
    }

    let mut pipe_1 = pipe_1.unwrap();
    let mut pipe_2 = pipe_2.unwrap();

    let mut steps: usize = 1;

    let mut pipes = BTreeSet::new();
    pipes.insert(starting_position);
    pipes.insert(pipe_1.0);
    pipes.insert(pipe_2.0);

    loop {
        let next_pipe_1 = next_pipe(&grid, pipe_1);
        let next_pipe_2 = next_pipe(&grid, pipe_2);

        steps += 1;

        pipes.insert(next_pipe_1.0);
        pipes.insert(next_pipe_2.0);

        if next_pipe_1.0 .0 == next_pipe_2.0 .0 && next_pipe_1.0 .1 == next_pipe_2.0 .1 {
            break;
        }

        pipe_1 = next_pipe_1;
        pipe_2 = next_pipe_2;
    }

    let mut inside_pipes: Vec<Position> = Vec::new();
    for (y, _) in grid.iter().enumerate() {
        let mut inside = false;
        let mut consuming_pipe = None;
        for x in 0..grid[y].len() {
            if pipes.contains(&(x, y)) {
                if grid[y][x] == Pipe::NorthSouth {
                    inside = !inside;
                } else if grid[y][x] == Pipe::NorthEast || grid[y][x] == Pipe::SouthEast {
                    consuming_pipe = Some(grid[y][x]);
                } else if grid[y][x] == Pipe::NorthWest || grid[y][x] == Pipe::SouthWest {
                    if let Some(Pipe::NorthEast) = consuming_pipe {
                        if grid[y][x] == Pipe::SouthWest {
                            inside = !inside;
                            consuming_pipe = None;
                        }
                    } else if let Some(Pipe::SouthEast) = consuming_pipe {
                        if grid[y][x] == Pipe::NorthWest {
                            inside = !inside;
                            consuming_pipe = None;
                        }
                    }
                }
            } else if inside {
                inside_pipes.push((x, y));
            }
        }
    }

    println!("Part 1: {steps}");
    println!("Part 2: {}", inside_pipes.len());
}
