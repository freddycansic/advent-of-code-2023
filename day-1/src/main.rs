fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input
        .split_whitespace()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap();
            let last = line.chars().rev().find(char::is_ascii_digit).unwrap();

            (first.to_string() + &last.to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .to_string()
        .split_whitespace()
        .map(String::from)
        .map(|line| {
            let mut found_digits = Vec::<u32>::new();

            for (index, char) in line.char_indices() {
                for (number_word, number) in number_words {
                    if line[index..].starts_with(number_word) {
                        found_digits.push(number);
                    }
                }

                if let Some(digit) = char.to_digit(10) {
                    found_digits.push(digit)
                }
            }

            let first = found_digits.first().unwrap();
            let last = found_digits.last().unwrap();

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        )
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        )
    }
}
