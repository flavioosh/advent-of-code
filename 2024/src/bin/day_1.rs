use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn count_frequency<T>(list: &[T]) -> HashMap<&T, usize>
where
    T: Hash + Eq + Clone,
{
    let mut counts = HashMap::new();
    for entry in list {
        let hashmap_entry = counts.entry(entry).or_insert(0);
        *hashmap_entry += 1;
    }

    counts
}

fn main() {
    let data = fs::read_to_string("res/day_1.txt").expect("Could not read file");

    let (mut locations_1, mut locations_2): (Vec<usize>, Vec<usize>) = data
        .lines()
        .map(|line| {
            let mut locations = line.split_ascii_whitespace();
            let location_1: usize = locations
                .next()
                .expect("Missing first location")
                .parse()
                .expect("Not a number");
            let location_2: usize = locations
                .next()
                .expect("Missing second location")
                .parse()
                .expect("Not a number");

            assert!(locations.next().is_none(), "Invalid input, must have two locations per line");

            (location_1, location_2)
        })
        .unzip();

    locations_1.sort_unstable();
    locations_2.sort_unstable();

    let distance_sum: usize = locations_1
        .iter()
        .zip(locations_2.iter())
        .map(|(location_1, location_2)| location_1.abs_diff(*location_2))
        .sum();

    println!("Part 1: {distance_sum}");

    let locations_1_frequency = count_frequency(&locations_1);
    let locations_2_frequency = count_frequency(&locations_2);

    let similarity_score_sum: usize = locations_1_frequency
        .iter()
        .map(|(location, location_1_count)| {
            let location_2_count = *locations_2_frequency.get(location).unwrap_or(&0);
            *location * location_1_count * location_2_count
        })
        .sum();

    println!("Part 2: {similarity_score_sum}");
}
