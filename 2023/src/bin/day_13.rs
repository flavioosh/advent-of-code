use std::fs;

type Pattern = Vec<Vec<bool>>;

fn check_symmetry(pattern: &Pattern, disallowed_result: usize) -> usize {
    let mut i = 0;
    let mut offset = 0;
    while i < (pattern.len() - 1) as isize {
        let current_left = i - offset;
        let current_right = i + 1 + offset;
        if current_left < 0 || current_right >= pattern.len() as isize {
            return 0;
        }
        if pattern[current_left as usize] == pattern[current_right as usize] {
            if current_left == 0 || current_right == (pattern.len() - 1) as isize {
                if i as usize + 1 == disallowed_result {
                    i += 1;
                    offset = 0;
                    continue;
                }
                // Full symmetry match
                return i as usize + 1;
            }
            // Cells match
            offset += 1;
        } else {
            // No match
            i += 1;
            offset = 0;
        }
    }

    0
}

fn main() {
    let data = fs::read_to_string("res/day_13.txt").unwrap();

    let patterns = data.split("\n\n");
    let patterns: Vec<Pattern> = patterns
        .map(|pattern| {
            pattern
                .split('\n')
                .map(|row| row.chars().map(|char| char == '#').collect())
                .collect()
        })
        .collect();

    let mut sum_1: usize = 0;
    let mut sum_2: usize = 0;

    'outer: for pattern in patterns {
        // Check for symmetry vertically by comparing full rows
        let mut result_1 = check_symmetry(&pattern, 0) * 100;

        if result_1 != 0 {
            sum_1 += result_1;
            // We found vertical symmetry, move on to the next pattern
        } else {
            // Check for horizontal symmetry
            // Rotate pattern
            let rotated_pattern: Pattern = (0..pattern[0].len())
                .into_iter()
                .map(|y| pattern.iter().map(|x| x[y]).collect())
                .collect();

            result_1 = check_symmetry(&rotated_pattern, 0);
            sum_1 += result_1;
        }
        let result_1 = result_1;

        for row in 0..pattern.len() {
            for col in 0..pattern[row].len() {
                let mut pattern = pattern.clone();
                // Flip the field to "smudge" it
                pattern[row][col] = !pattern[row][col];
                let pattern = pattern;

                // Check for symmetry vertically by comparing full rows
                let result_2 = check_symmetry(&pattern, result_1 / 100) * 100;

                if result_2 == result_1 {
                    // We can't use the same symmetry line as in part 1
                    continue;
                }

                if result_2 != 0 {
                    sum_2 += result_2;
                    // We found vertical symmetry, move on to the next pattern
                    continue 'outer;
                }

                // Check for horizontal symmetry
                // Rotate pattern
                let rotated_pattern: Pattern = (0..pattern[0].len())
                    .into_iter()
                    .map(|y| pattern.iter().map(|x| x[y]).collect())
                    .collect();

                let result_2 = check_symmetry(&rotated_pattern, result_1);

                if result_2 == result_1 {
                    // We can't use the same symmetry line as in part 1
                    continue;
                }

                if result_2 == 0 {
                    // We have to find symmetry
                    continue;
                }

                sum_2 += result_2;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}
