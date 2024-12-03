use std::fs;

fn main() {
    let data = fs::read_to_string("res/day_2.txt").expect("Could not read file");

    let safe_reports = data
        .lines()
        .map(|report| {
            let levels: Vec<usize> = report
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();

            let mut is_increasing: Option<bool> = None;

            for window in levels.windows(2) {
                let level = window[0];
                let next_level = window[1];

                if level == next_level || level.abs_diff(next_level) > 3 {
                    return false;
                }

                if let Some(is_increasing) = is_increasing {
                    if is_increasing && level >= next_level || !is_increasing && level <= next_level
                    {
                        return false;
                    }
                } else {
                    is_increasing = Some(level < next_level);
                }
            }

            true
        })
        .filter(|safe| *safe)
        .count();

    println!("Part 1: {safe_reports}");

    let tolerated_safe_reports = data
        .lines()
        .map(|report| {
            let levels: Vec<usize> = report
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();

            'outer: for i in 0..levels.len() {
                let mut trimmed_levels = levels.clone();
                trimmed_levels.remove(i);

                let mut is_increasing: Option<bool> = None;

                for window in trimmed_levels.windows(2) {
                    let level = window[0];
                    let next_level = window[1];

                    if level == next_level || level.abs_diff(next_level) > 3 {
                        continue 'outer;
                    }

                    if let Some(is_increasing) = is_increasing {
                        if is_increasing && level >= next_level
                            || !is_increasing && level <= next_level
                        {
                            continue 'outer;
                        }
                    } else {
                        is_increasing = Some(level < next_level);
                    }
                }

                return true;
            }

            false
        })
        .filter(|safe| *safe)
        .count();

    println!("Part 2: {tolerated_safe_reports}");
}
