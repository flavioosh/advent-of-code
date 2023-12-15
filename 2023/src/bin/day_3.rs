use std::{collections::BTreeMap, fs};

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Digit(u8),
    Symbol,
    Gear(usize, usize),
    Empty,
}

#[inline]
fn is_symbol_adjacent(grid: &Vec<Vec<Cell>>, row: usize, col: usize) -> bool {
    if row > 0 && col > 0 && grid[row - 1][col - 1] == Cell::Symbol {
        return true;
    }
    if row > 0 && grid[row - 1][col] == Cell::Symbol {
        return true;
    }
    if row > 0 && col + 1 < grid[row].len() && grid[row - 1][col + 1] == Cell::Symbol {
        return true;
    }
    if col > 0 && grid[row][col - 1] == Cell::Symbol {
        return true;
    }
    if col + 1 < grid[row].len() && grid[row][col + 1] == Cell::Symbol {
        return true;
    }
    if row + 1 < grid.len() && col > 0 && grid[row + 1][col - 1] == Cell::Symbol {
        return true;
    }
    if row + 1 < grid.len() && grid[row + 1][col] == Cell::Symbol {
        return true;
    }
    if row + 1 < grid.len() && col + 1 < grid[row].len() && grid[row + 1][col + 1] == Cell::Symbol {
        return true;
    }

    false
}

#[inline]
fn is_gear_adjacent(grid: &Vec<Vec<Cell>>, row: usize, col: usize) -> Option<(usize, usize)> {
    if row > 0 {
        if col > 0 {
            if let Cell::Gear(x, y) = grid[row - 1][col - 1] {
                return Some((x, y));
            }
        }
        if let Cell::Gear(x, y) = grid[row - 1][col] {
            return Some((x, y));
        }
        if col + 1 < grid[row].len() {
            if let Cell::Gear(x, y) = grid[row - 1][col + 1] {
                return Some((x, y));
            }
        }
    }

    if col > 0 {
        if let Cell::Gear(x, y) = grid[row][col - 1] {
            return Some((x, y));
        }
    }
    if col + 1 < grid[row].len() {
        if let Cell::Gear(x, y) = grid[row][col + 1] {
            return Some((x, y));
        }
    }

    if row + 1 < grid.len() {
        if col > 0 {
            if let Cell::Gear(x, y) = grid[row + 1][col - 1] {
                return Some((x, y));
            }
        }
        if let Cell::Gear(x, y) = grid[row + 1][col] {
            return Some((x, y));
        }
        if col + 1 < grid[row].len() {
            if let Cell::Gear(x, y) = grid[row + 1][col + 1] {
                return Some((x, y));
            }
        }
    }

    None
}

fn main() {
    let data = fs::read_to_string("res/day_3.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();

    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(lines.len());

    for (i, &line) in lines.iter().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (j, char) in line.chars().enumerate() {
            row.push(match char {
                c if c.is_ascii_digit() => Cell::Digit(c.to_digit(10).unwrap() as u8),
                '*' => Cell::Gear(i, j),
                '.' => Cell::Empty,
                _ => Cell::Symbol,
            });
        }
        grid.push(row);
    }

    let grid = grid;

    let mut sum_1: u32 = 0;
    let mut numbers: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    let mut is_number = false;
    let mut current_number: u32 = 0;
    let mut is_adjacent = false;
    let mut adjacent_gear: Option<(usize, usize)> = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Digit(d) => {
                    if is_number {
                        current_number = 10 * current_number + *d as u32;
                    } else {
                        current_number = *d as u32;
                    }
                    is_number = true;
                    if adjacent_gear.is_none() {
                        adjacent_gear = is_gear_adjacent(&grid, i, j);
                    }
                    is_adjacent =
                        is_adjacent || adjacent_gear.is_some() || is_symbol_adjacent(&grid, i, j);
                }
                _ => {
                    if is_number {
                        if let Some(gear_location) = adjacent_gear {
                            numbers
                                .entry(gear_location)
                                .and_modify(|entries: &mut Vec<u32>| entries.push(current_number))
                                .or_insert(vec![current_number]);
                        }
                        if is_adjacent {
                            sum_1 += current_number;
                        }
                        current_number = 0;
                        is_number = false;
                        adjacent_gear = None;
                        is_adjacent = false;
                    }
                }
            }
        }
        if is_number {
            if let Some(gear_location) = adjacent_gear {
                numbers
                    .entry(gear_location)
                    .and_modify(|entries: &mut Vec<u32>| entries.push(current_number))
                    .or_insert(vec![current_number]);
            }
            if is_adjacent {
                sum_1 += current_number;
            }
            current_number = 0;
            is_number = false;
            adjacent_gear = None;
            is_adjacent = false;
        }
    }

    let mut sum_2: u32 = 0;
    for (_, numbers) in numbers.iter() {
        if numbers.len() > 1 {
            sum_2 += numbers[0] * numbers[1];
        }
    }

    println!("Part 1: {sum_1}");
    println!("Part 2: {sum_2}");
}
