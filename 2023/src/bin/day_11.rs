use std::fs;

fn main() {
    let data = fs::read_to_string("res/day_11.txt").unwrap();
    let grid: Vec<Vec<bool>> = data
        .split('\n')
        .map(|line| line.chars().map(|char| char == '#').collect::<Vec<bool>>())
        .collect();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut empty_rows = vec![false; grid_height];
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|v| !*v) {
            empty_rows[i] = true;
        }
    }

    let mut empty_columns = vec![false; grid_height];
    'outer: for x in 0..grid_width {
        for y in 0..grid_height {
            if grid[y][x] {
                continue 'outer;
            }
        }

        empty_columns[x] = true;
    }

    let galaxies: Vec<(usize, usize)> = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, is_galaxy)| *is_galaxy)
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut accumulator_1: usize = 0;
    let mut accumulator_2: usize = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let galaxy_1 = galaxies[i];
            let galaxy_2 = galaxies[j];

            let start_x = galaxy_1.0.min(galaxy_2.0);
            let start_y = galaxy_1.1.min(galaxy_2.1);

            let x_diff = (galaxy_1.0 as isize - galaxy_2.0 as isize).abs() as usize;
            let y_diff = (galaxy_1.1 as isize - galaxy_2.1 as isize).abs() as usize;

            let x_expanses: usize = empty_columns[start_x..start_x + x_diff]
                .into_iter()
                .filter(|empty| **empty)
                .count();
            let y_expanses: usize = empty_rows[start_y..start_y + y_diff]
                .into_iter()
                .filter(|empty| **empty)
                .count();

            accumulator_1 +=
                (x_diff - x_expanses) + (y_diff - y_expanses) + (y_expanses * 2) + (x_expanses * 2);
            accumulator_2 += (x_diff - x_expanses)
                + (y_diff - y_expanses)
                + (y_expanses * 1_000_000)
                + (x_expanses * 1_000_000);
        }
    }

    println!("Part 1: {}", accumulator_1);
    println!("Part 2: {}", accumulator_2);
}
