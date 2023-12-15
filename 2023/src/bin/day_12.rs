use std::fs;

use cached::proc_macro::cached;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Row {
    conditions: Vec<SpringCondition>,
    condition_counts: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SpringCondition {
    Working,
    Damaged,
    Unknown,
}

#[allow(dead_code)]
fn conditions_to_str(input: &[SpringCondition]) -> String {
    String::from_utf8(
        input
            .iter()
            .map(|condition| match condition {
                SpringCondition::Working => '.',
                SpringCondition::Damaged => '#',
                SpringCondition::Unknown => '?',
            } as u8)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

#[cached]
fn check(
    conditions: Vec<SpringCondition>,
    condition_counts: Vec<usize>,
    working_count: usize,
    damaged_count: usize,
) -> usize {
    // If we run the function on empty values, we have found a match
    if conditions.is_empty()
        && condition_counts.is_empty()
        && working_count == 0
        && damaged_count == 0
    {
        return 1;
    }

    let mut sum = 0;

    match conditions[0] {
        SpringCondition::Working => {
            // If it's a working spring, we can just ignore it and move on
            return check(
                conditions[1..].to_vec(),
                condition_counts,
                working_count,
                damaged_count,
            );
        }
        SpringCondition::Damaged => {
            let expected_contiguous_damaged_springs = condition_counts[0];

            // Check if we have at least as many springs available to check as expected
            if conditions.len() < expected_contiguous_damaged_springs {
                // Not enough contiguous springs, not a match
                return 0;
            }

            let mut consumed_unknown_damaged = 0;
            for condition in conditions
                .iter()
                .take(expected_contiguous_damaged_springs)
                .skip(1)
            {
                match condition {
                    SpringCondition::Working => {
                        // Not a match
                        return 0;
                    }
                    SpringCondition::Damaged => {
                        // Damaged spring, consume (by doing nothing)
                    }
                    SpringCondition::Unknown => {
                        // An unknown position, assume that it's damaged
                        consumed_unknown_damaged += 1;
                    }
                }
            }

            // Check that we haven't consumed more unknown damaged than are available
            if consumed_unknown_damaged > damaged_count {
                return 0;
            }

            // If this isn't the end of the conditions vector, we have to check if the next value is a damaged spring or an unknown.
            // If it's a damaged spring, we can early return.
            // If it's an unknown, we consume it and remove it from the available working.
            if conditions.len() > expected_contiguous_damaged_springs {
                match conditions[expected_contiguous_damaged_springs] {
                    SpringCondition::Working => {
                        // All good, no need to do anything, we can consume as expected, including the working spring
                    }
                    SpringCondition::Damaged => {
                        // Can't have more contiguous springs, not a valid variant
                        return 0;
                    }
                    SpringCondition::Unknown => {
                        // An unknown spring, check if we have available working springs
                        if working_count > 0 {
                            // We do, consume it
                            return check(
                                conditions[expected_contiguous_damaged_springs + 1..].to_vec(),
                                condition_counts[1..].to_vec(),
                                working_count - 1,
                                damaged_count - consumed_unknown_damaged,
                            );
                        }

                        // We don't have enough working springs, not a valid variant
                        return 0;
                    }
                }
            }

            // At this point, we have either returned because there haven't been enough contiguous
            // broken springs, or we have consumed known and/or unknown damaged springs.

            // We can run the check function again without the consumed damaged springs with all values adjusted accordingly.
            return check(
                conditions[expected_contiguous_damaged_springs..].to_vec(),
                condition_counts[1..].to_vec(),
                working_count,
                damaged_count - consumed_unknown_damaged,
            );
        }
        SpringCondition::Unknown => {
            if working_count != 0 {
                // Ignore the first spring as if it was a working spring, and move on
                sum += check(
                    conditions[1..].to_vec(),
                    condition_counts.clone(),
                    working_count - 1,
                    damaged_count,
                )
            }

            if damaged_count != 0 {
                // Replace the first spring as if it was a damaged spring
                let mut conditions = conditions.clone();
                conditions[0] = SpringCondition::Damaged;
                sum += check(
                    conditions,
                    condition_counts.clone(),
                    working_count,
                    damaged_count - 1,
                );
            }
        }
    }

    sum
}

fn get_variations(input: String) -> usize {
    let rows: Vec<Row> = input
        .split('\n')
        .map(|arrangement| {
            let (conditions, condition_counts) = arrangement.split_once(' ').unwrap();
            let conditions = conditions.trim_matches('.');
            let conditions: Vec<SpringCondition> = conditions
                .chars()
                .map(|char| match char {
                    '.' => SpringCondition::Working,
                    '#' => SpringCondition::Damaged,
                    '?' => SpringCondition::Unknown,
                    _ => panic!("Invalid spring condition"),
                })
                .collect();
            let condition_counts: Vec<usize> = condition_counts
                .split(',')
                .map(|count| count.parse().unwrap())
                .collect();

            Row {
                conditions,
                condition_counts,
            }
        })
        .collect();

    let mut sum: usize = 0;
    for row in rows {
        let damaged_count = row
            .conditions
            .iter()
            .filter(|condition| condition == &&SpringCondition::Damaged)
            .count();
        let unknown_count = row
            .conditions
            .iter()
            .filter(|condition| condition == &&SpringCondition::Unknown)
            .count();

        let actual_damaged_count: usize = row.condition_counts.iter().sum();

        let missing_damaged_count = actual_damaged_count - damaged_count;
        let missing_working_count = unknown_count - missing_damaged_count;

        let result = check(
            row.conditions,
            row.condition_counts,
            missing_working_count,
            missing_damaged_count,
        );

        sum += result;
    }

    sum
}

fn part_1() {
    let data = fs::read_to_string("res/day_12.txt").unwrap();
    let sum = get_variations(data);

    println!("Part 1: {sum}");
}

fn part_2() {
    let data = fs::read_to_string("res/day_12.txt").unwrap();
    let data = data
        .split('\n')
        .map(|line| {
            let (conditions, condition_counts) = line.split_once(' ').unwrap();
            let conditions = format!(
                "{}?{}?{}?{}?{}",
                conditions, conditions, conditions, conditions, conditions
            );
            let condition_counts = format!(
                "{},{},{},{},{}",
                condition_counts,
                condition_counts,
                condition_counts,
                condition_counts,
                condition_counts
            );
            format!("{} {}", conditions, condition_counts)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let sum = get_variations(data);

    println!("Part 2: {sum}");
}

fn main() {
    part_1();
    part_2();
}
