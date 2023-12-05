use itertools::Itertools;
use std::{collections::VecDeque, ops::Range};

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

struct Mapping {
    source: Range<u64>,
    destination_start: u64,
}

fn create_mappings(sections: &[Vec<u64>]) -> Vec<Vec<Mapping>> {
    sections
        .iter()
        .map(|section| {
            let mut map = Vec::<Mapping>::new();

            for mut chunk in &section.into_iter().chunks(3) {
                let destination_start = chunk.next().unwrap();
                let source_start = chunk.next().unwrap();
                let length = chunk.next().unwrap();

                map.push(Mapping {
                    source: *source_start..*source_start + *length,
                    destination_start: *destination_start,
                });
            }

            map
        })
        .collect::<Vec<Vec<Mapping>>>()
}

fn apply_mappings(seed: u64, mappings: &[Vec<Mapping>]) -> u64 {
    let mut current = seed;

    for map in mappings.iter() {
        for mapping in map.iter() {
            if mapping.source.contains(&current) {
                current = mapping.destination_start + current.abs_diff(mapping.source.start);
                break;
            }
        }
    }

    current
}

fn get_seeds_and_sections(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut sections = input
        .split("\n\n")
        .map(str::trim)
        .map(|section| {
            section
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect::<VecDeque<Vec<u64>>>();

    let seeds = sections.pop_front().unwrap();

    (seeds, sections.into())
}

fn part_1(input: &str) -> u64 {
    let (seeds, sections) = get_seeds_and_sections(input);

    let mappings = create_mappings(&sections);

    seeds
        .into_iter()
        .map(|seed| apply_mappings(seed, &mappings))
        .min()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let (seeds, sections) = get_seeds_and_sections(input);

    let seed_ranges = seeds.chunks(2).map(|chunk| chunk[0]..chunk[0] + chunk[1]);

    let mappings = create_mappings(&sections);

    let mut min_location = u64::MAX;

    for seed_range in seed_ranges {
        for seed in seed_range {
            let location = apply_mappings(seed, &mappings);

            min_location = min_location.min(location);
        }
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            46
        );
    }
}
