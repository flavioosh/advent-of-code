use std::{fs, thread};

#[derive(Clone, Debug)]
struct Mapping {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<Mapping>>) {
    let mut groups = input.split("\n\n");

    let seeds: Vec<usize> = groups
        .next()
        .unwrap()
        .replacen("seeds: ", "", 1)
        .split_ascii_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let maps = groups
        .map(|group| {
            let mut lines = group.split('\n');
            lines.next();
            lines
                .map(|line| {
                    let mut parts = line.split_ascii_whitespace();
                    Mapping {
                        destination_range_start: parts.next().unwrap().parse().unwrap(),
                        source_range_start: parts.next().unwrap().parse().unwrap(),
                        range_length: parts.next().unwrap().parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn part_1() {
    let data = fs::read_to_string("res/day_5.txt").unwrap();
    let (seeds, maps) = parse_input(&data);

    let mut sources = seeds;
    let mut destinations = sources.clone();

    for map in maps {
        for mapping in map {
            for (i, source) in sources.iter().enumerate() {
                // Check is in range of mapping
                if *source < mapping.source_range_start
                    || *source > mapping.source_range_start + mapping.range_length
                {
                    continue;
                }

                let result = *source as isize
                    + (mapping.destination_range_start as isize
                        - mapping.source_range_start as isize);

                if result < 0 {
                    continue;
                }

                destinations[i] = result as usize;
            }
        }

        sources = destinations.clone();
    }

    println!("Part 1: {:?}", sources.iter().min().unwrap());
}

fn part_2() {
    let data = fs::read_to_string("res/day_5.txt").unwrap();
    let (seeds, maps) = parse_input(&data);

    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    let mut current_start = None;
    for seed in seeds {
        if current_start.is_none() {
            current_start = Some(seed);
        } else {
            seed_ranges.push((current_start.unwrap(), seed));
            current_start = None;
        }
    }

    let mut threads = Vec::new();

    for seed_range in seed_ranges {
        let seed_range = seed_range.clone();
        let maps = maps.clone();
        let result = thread::spawn(move || {
            let result = (seed_range.0..(seed_range.0 + seed_range.1))
                .map(|seed| {
                    let mut source: usize = seed;
                    let mut destination = usize::MAX;
                    for map in maps.iter() {
                        for mapping in map.iter() {
                            if source < mapping.source_range_start
                                || source > mapping.source_range_start + mapping.range_length
                            {
                                continue;
                            }

                            let result = source as isize
                                + (mapping.destination_range_start as isize
                                    - mapping.source_range_start as isize);
                            if result < 0 {
                                continue;
                            }

                            let result = result as usize;
                            if result < destination {
                                destination = result;
                            }
                        }

                        if destination != usize::MAX {
                            source = destination;
                            destination = usize::MAX;
                        }
                    }

                    source
                })
                .min()
                .unwrap();
            result
        });
        threads.push(result);
    }

    let min = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .min()
        .unwrap();

    println!("Part 2: {}", min);
}

fn main() {
    part_1();
    part_2();
}
