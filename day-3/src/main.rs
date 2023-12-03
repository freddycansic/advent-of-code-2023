use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

#[derive(Debug)]
struct Number {
    row: usize,
    col: usize,
    digits: Vec<u32>,
}

impl Number {
    fn value(&self) -> u32 {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |total, (magnitude, digit)| {
                total + 10_u32.pow(magnitude as u32) * digit
            })
    }
}

fn part_1(input: &str) -> u32 {
    let input = input.split('\n').map(str::trim).collect::<Vec<&str>>();
    let width = input[0].len();
    let height = input.len();

    // find numbers
    find_numbers(&input)
        .iter()
        .filter_map(|number| {
            let mut adjacent = HashSet::<(usize, usize)>::new();

            for (magnitude, _) in number.digits.iter().enumerate() {
                for adjacent_square in in_bounds_adjacent(
                    number.row as isize,
                    number.col as isize + magnitude as isize,
                    width,
                    height,
                ) {
                    adjacent.insert(adjacent_square);
                }
            }

            for coord in adjacent {
                let square = input[coord.0].chars().nth(coord.1).unwrap();
                if !square.is_digit(10) && square != '.' {
                    return Some(number.value());
                }
            }

            None
        })
        .sum()
}

fn find_numbers(input: &[&str]) -> Vec<Number> {
    input
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            let mut numbers_in_row = Vec::new();

            let mut row_chars_iter = row.char_indices();
            while let Some((col_index, current)) = row_chars_iter.next() {
                if current.is_ascii_digit() {
                    let mut digits = vec![current.to_digit(10).unwrap()];

                    // seek until hit non digit
                    loop {
                        let next = row_chars_iter.next();

                        // if we've reached the end of the line or the next char is not a digit
                        if next.is_none() || !next.unwrap().1.is_ascii_digit() {
                            numbers_in_row.push(Number {
                                row: row_index,
                                col: col_index,
                                digits,
                            });

                            break;
                        }

                        digits.push(next.unwrap().1.to_digit(10).unwrap());
                    }
                }
            }

            numbers_in_row
        })
        .flatten()
        .collect::<Vec<Number>>()
}

fn in_bounds_adjacent(
    row_index: isize,
    col_index: isize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let coords = [
        (row_index, col_index + 1),
        (row_index, col_index - 1),
        (row_index + 1, col_index),
        (row_index - 1, col_index),
        (row_index + 1, col_index - 1),
        (row_index - 1, col_index - 1),
        (row_index + 1, col_index + 1),
        (row_index - 1, col_index + 1),
    ];

    coords
        .into_iter()
        .filter(|coord| in_bounds(coord.0, coord.1, width, height))
        .map(|coord| (coord.0 as usize, coord.1 as usize))
        .collect::<Vec<(usize, usize)>>()
}

fn in_bounds(row_index: isize, col_index: isize, width: usize, height: usize) -> bool {
    row_index >= 0 && row_index < height as isize && col_index >= 0 && col_index < width as isize
}

fn part_2(input: &str) -> u32 {
    let input = input.split('\n').map(str::trim).collect::<Vec<&str>>();
    let numbers = find_numbers(&input);

    let asterisk_coords = input
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .positions(|char| char == '*')
                .map(|col_index| (row_index, col_index))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    asterisk_coords
        .into_iter()
        .filter_map(|(asterisk_row, asterisk_col)| {
            let adjacent_numbers = numbers
                .iter()
                .filter_map(|number| {
                    // if any digit of number has pythagorean distance of <=sqrt(2) to asterisk then it is adjacent
                    for num_digit in 0..number.digits.len() {
                        if ((number.col as f32 + num_digit as f32 - asterisk_col as f32)
                            .powf(2_f32)
                            + (number.row as f32 - asterisk_row as f32).powf(2_f32))
                        .sqrt() as f32
                            <= 2_f32.sqrt()
                        {
                            return Some(number.value());
                        }
                    }

                    None
                })
                .collect::<Vec<u32>>();

            if adjacent_numbers.len() == 2 {
                Some(adjacent_numbers[0] * adjacent_numbers[1])
            } else {
                None
            }
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
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }
}
