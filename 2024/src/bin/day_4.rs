use std::fs;

fn search_1(grid: &[Vec<char>], row: usize, column: usize) -> usize {
    let can_search_up = row >= 3;
    let can_search_down = row < grid.len() - 3;
    let can_search_left = column >= 3;
    let can_search_right = column < grid[0].len() - 3;

    let mut total = 0;

    // Up
    if can_search_up
        && grid[row - 1][column] == 'M'
        && grid[row - 2][column] == 'A'
        && grid[row - 3][column] == 'S'
    {
        total += 1;
    }

    // Down
    if can_search_down
        && grid[row + 1][column] == 'M'
        && grid[row + 2][column] == 'A'
        && grid[row + 3][column] == 'S'
    {
        total += 1;
    }

    // Left
    if can_search_left
        && grid[row][column - 1] == 'M'
        && grid[row][column - 2] == 'A'
        && grid[row][column - 3] == 'S'
    {
        total += 1;
    }

    // Right
    if can_search_right
        && grid[row][column + 1] == 'M'
        && grid[row][column + 2] == 'A'
        && grid[row][column + 3] == 'S'
    {
        total += 1;
    }

    // Up Left
    if can_search_up
        && can_search_left
        && grid[row - 1][column - 1] == 'M'
        && grid[row - 2][column - 2] == 'A'
        && grid[row - 3][column - 3] == 'S'
    {
        total += 1;
    }

    // Up Right
    if can_search_up
        && can_search_right
        && grid[row - 1][column + 1] == 'M'
        && grid[row - 2][column + 2] == 'A'
        && grid[row - 3][column + 3] == 'S'
    {
        total += 1;
    }

    // Down Left
    if can_search_down
        && can_search_left
        && grid[row + 1][column - 1] == 'M'
        && grid[row + 2][column - 2] == 'A'
        && grid[row + 3][column - 3] == 'S'
    {
        total += 1;
    }

    // Down Right
    if can_search_down
        && can_search_right
        && grid[row + 1][column + 1] == 'M'
        && grid[row + 2][column + 2] == 'A'
        && grid[row + 3][column + 3] == 'S'
    {
        total += 1;
    }

    total
}

fn search_2(grid: &[Vec<char>], row: usize, column: usize) -> bool {
    let can_search_up = row >= 1;
    let can_search_down = row < grid.len() - 1;
    let can_search_left = column >= 1;
    let can_search_right = column < grid[0].len() - 1;

    let can_search = can_search_up && can_search_down && can_search_left && can_search_right;

    if !can_search {
        return false;
    }

    (grid[row - 1][column - 1] == 'M' && grid[row + 1][column + 1] == 'S'
        || grid[row - 1][column - 1] == 'S' && grid[row + 1][column + 1] == 'M')
        && (grid[row - 1][column + 1] == 'M' && grid[row + 1][column - 1] == 'S'
        || grid[row - 1][column + 1] == 'S' && grid[row + 1][column - 1] == 'M')
}

fn main() {
    let data = fs::read_to_string("res/day_4.txt").expect("Could not read file");

    let grid = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut xmas_total = 0;
    let mut x_mas_total: usize = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            match column {
                'X' => xmas_total += search_1(&grid, i, j),
                'A' => {
                    if search_2(&grid, i, j) {
                        x_mas_total += 1;
                    }
                }
                _ => {}
            }
        }
    }

    println!("Part 1: {xmas_total}");
    println!("Part 2: {x_mas_total}");
}
