use core::panic;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

struct Destination {
    left: String,
    right: String,
}

fn parse_map(input: &str) -> (&str, HashMap<String, Destination>) {
    let (directions, map) = input.trim().split_once("\n\n").unwrap();
    let map = HashMap::<String, Destination>::from_iter(map.trim().split('\n').map(|line| {
        let (from, to) = line.split_once(" = (").unwrap();
        let left = to[0..3].to_string();
        let right = to[5..to.len() - 1].to_string();

        (from.to_string(), Destination { left, right })
    }));

    (directions, map)
}

fn num_steps_until<F: Fn(&str) -> bool>(
    start: &str,
    until: F,
    map: &HashMap<String, Destination>,
    directions: &str,
) -> usize {
    let mut current_location = start;
    let mut num_steps = 0;

    while !until(current_location) {
        match directions
            .chars()
            .nth(num_steps % directions.len())
            .unwrap()
        {
            'L' => current_location = &map[current_location].left,
            'R' => current_location = &map[current_location].right,
            _ => panic!("help"),
        }

        num_steps += 1;
    }

    num_steps
}

// I stole this code
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part_1(input: &str) -> usize {
    let (directions, map) = parse_map(input);

    num_steps_until("AAA", |string| string == "ZZZ", &map, directions)
}

fn part_2(input: &str) -> usize {
    let (directions, map) = parse_map(input);

    let starting_strings = map.keys().filter(|key| key.ends_with('A'));

    let lowest_for_each = starting_strings
        .into_iter()
        .map(|starting_string| {
            num_steps_until(
                starting_string,
                |string| string.ends_with('Z'),
                &map,
                directions,
            )
        })
        .collect_vec();

    lcm(&lowest_for_each)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(
            part_1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
