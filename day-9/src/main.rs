use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

fn differences(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .tuple_windows()
        .map(|(current, next)| next - current)
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(input: &str) -> i32 {
    parse_input(input)
        .into_iter()
        .map(|sequence| {
            let mut last_differences = Vec::new();
            let mut current_sequence = sequence.clone();

            loop {
                let differences = differences(&current_sequence);

                if let Some(last) = differences.last() {
                    last_differences.push(last.clone());
                } else {
                    break;
                }

                if differences.iter().all(|num| *num == 0) {
                    break;
                }

                current_sequence = differences;
            }

            last_differences.into_iter().sum::<i32>() + sequence.last().unwrap()
        })
        .sum::<i32>()
}

fn part_2(input: &str) -> i32 {
    parse_input(input)
        .into_iter()
        .map(|sequence| {
            let mut last_differences = Vec::new();
            let mut current_sequence = sequence.iter().cloned().rev().collect_vec();

            loop {
                let differences = differences(&current_sequence);

                if let Some(last) = differences.last() {
                    last_differences.push(last.clone());
                } else {
                    break;
                }

                if differences.iter().all(|num| *num == 0) {
                    break;
                }

                current_sequence = differences;
            }

            last_differences.into_iter().sum::<i32>() + sequence.first().unwrap()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(TEST_VECTOR), 114);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(TEST_VECTOR), 2);
    }
}
