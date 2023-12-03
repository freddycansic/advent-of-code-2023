use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

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

    find_numbers(&input)
        .iter()
        .filter_map(|number| {
            let mut adjacent = HashSet::<(usize, usize)>::new();

            for (magnitude, _) in number.digits.iter().enumerate() {
                for adjacent_square in
                    in_bounds_adjacent(number.row, number.col + magnitude, width, height)
                {
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
    row_index: usize,
    col_index: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let cols = col_index.checked_sub(1).unwrap_or(col_index)..=(col_index + 1).min(width - 1);
    let rows = row_index.checked_sub(1).unwrap_or(row_index)..=(row_index + 1).min(height - 1);

    rows.into_iter()
        .map(move |row| cols.clone().map(move |col| (row, col)))
        .flatten()
        .collect()
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
                        if (number.col as f32 + num_digit as f32 - asterisk_col as f32)
                            .hypot(number.row as f32 - asterisk_row as f32)
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
