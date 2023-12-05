use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let mut sections = input
        .split("\r\n\r")
        // .split("\n\n")
        .map(str::trim)
        .map(|section| {
            // dbg!(section);

            section
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(|num| {
                    // dbg!(num);
                    num.parse::<u64>()
                })
                .map(Result::unwrap)
                .collect()
        })
        .collect::<VecDeque<Vec<u64>>>();

    let seeds = sections.pop_front().unwrap();

    let mappings = sections
        .into_iter()
        .map(|section| {
            let mut map = Vec::<(std::ops::Range<u64>, std::ops::Range<u64>)>::new();

            for mut chunk in &section.into_iter().chunks(3) {
                let dest_range_start = chunk.next().unwrap();
                let source_range_start = chunk.next().unwrap();
                let range_length = chunk.next().unwrap();

                map.push((
                    source_range_start..source_range_start + range_length,
                    dest_range_start..dest_range_start + range_length,
                ));
            }

            map
        })
        .collect::<Vec<Vec<(std::ops::Range<u64>, std::ops::Range<u64>)>>>();

    let min_location = seeds
        .into_iter()
        .map(|seed| {
            let mut current = seed;

            for (index, map) in mappings.iter().enumerate() {
                dbg!(index);
                for (source, dest) in map.iter() {
                    if source.contains(&current) {
                        dbg!(current);

                        current = dest.clone().nth(0).unwrap()
                            + current.abs_diff(source.clone().nth(0).unwrap());
                        dbg!(current);
                        break;
                    }
                }
            }

            current
        })
        .min()
        .unwrap();

    min_location
}

fn part_2(input: &str) -> u64 {
    let mut sections = input
        .split("\r\n\r")
        // .split("\n\n")
        .map(str::trim)
        .map(|section| {
            dbg!(section);

            section
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<u64>())
                .map(Result::unwrap)
                .collect()
        })
        .collect::<VecDeque<Vec<u64>>>();

    let seeds = sections.pop_front().unwrap();

    // let seeds = seeds
    //     .iter()
    //     .chunks(2)
    //     .map(|chunk| (*start..start + length).collect::<Vec<u64>>())
    //     .flatten();
    let mut seeds2 = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let range = seeds[i + 1];

        seeds2.push(start..start + range);
    }

    let mappings = sections
        .into_iter()
        .map(|section| {
            let mut map = Vec::<(std::ops::Range<u64>, std::ops::Range<u64>)>::new();

            for mut chunk in &section.into_iter().chunks(3) {
                let dest_range_start = chunk.next().unwrap();
                let source_range_start = chunk.next().unwrap();
                let range_length = chunk.next().unwrap();

                map.push((
                    source_range_start..source_range_start + range_length,
                    dest_range_start..dest_range_start + range_length,
                ));
            }

            map
        })
        .collect::<Vec<Vec<(std::ops::Range<u64>, std::ops::Range<u64>)>>>();

    let mut min_location = u64::MAX;

    for seed_range in seeds2 {
        for seed in seed_range {
            let mut current = seed;

            if current % 1_000_000 == 0 {
                println!("{current}");
            }

            for (index, map) in mappings.iter().enumerate() {
                // dbg!(index);
                for (source, dest) in map.iter() {
                    if source.contains(&current) {
                        // dbg!(current);

                        current = dest.clone().nth(0).unwrap()
                            + current.abs_diff(source.clone().nth(0).unwrap());
                        // dbg!(current);
                        break;
                    }
                }
            }

            min_location = min_location.min(current);
        }

        println!("New seed range");
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
