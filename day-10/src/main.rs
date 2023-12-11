use core::panic;
use std::collections::HashMap;

use derive_more::Constructor;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

#[derive(Constructor, Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn direction_to(&self, other: &Point) -> Direction {
        if self.row > other.row {
            Direction::North
        } else if self.row < other.row {
            Direction::South
        } else if self.col > other.col {
            Direction::West
        } else {
            Direction::East
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_starting_point(pipes: &[Vec<char>]) -> Option<Point> {
    for (row_index, row) in pipes.iter().enumerate() {
        for (col_index, pipe) in row.iter().enumerate() {
            if *pipe == 'S' {
                return Some(Point {
                    row: row_index,
                    col: col_index,
                });
            }
        }
    }

    None
}

fn find_cycle(pipes: &[Vec<char>], starting_point: &Point) -> Vec<Point> {
    let mut cycle = Vec::<Point>::new();
    let mut last_point = Point {
        row: usize::MAX,
        col: usize::MAX,
    };
    let mut current_point = starting_point.clone();
    let rows = pipes.len();
    let cols = pipes[0].len();

    // Idk, it works
    let allowed_next = HashMap::from([
        (
            'S',
            [
                vec!['F', '7', '|'],
                vec!['-', '7', 'J'],
                vec!['|', 'L', 'J'],
                vec!['-', 'L', 'F'],
            ],
        ),
        (
            '|',
            [vec!['|', 'F', '7'], vec![], vec!['|', 'L', 'J'], vec![]],
        ),
        (
            '-',
            [vec![], vec!['-', '7', 'J'], vec![], vec!['-', 'L', 'F']],
        ),
        (
            'F',
            [vec![], vec!['-', '7', 'J'], vec!['|', 'L', 'J'], vec![]],
        ),
        (
            '7',
            [vec![], vec![], vec!['|', 'L', 'J'], vec!['-', 'L', 'F']],
        ),
        (
            'L',
            [vec!['|', 'F', '7'], vec!['-', '7', 'J'], vec![], vec![]],
        ),
        (
            'J',
            [vec!['|', 'F', '7'], vec![], vec![], vec!['-', 'F', 'L']],
        ),
    ]);

    loop {
        if cycle.contains(&current_point) {
            break;
        }

        cycle.push(current_point.clone());

        let current_pipe = pipes[current_point.row][current_point.col];

        let adjacent = adjacent(&current_point, rows, cols)
            .into_iter()
            .filter(|adjacent| *adjacent != last_point);

        for possible_next in adjacent {
            let possible_next_pipe = pipes[possible_next.row][possible_next.col];

            let possible_next_direction = current_point.direction_to(&possible_next) as usize;

            if allowed_next[&current_pipe][possible_next_direction].contains(&possible_next_pipe) {
                last_point = current_point.clone();
                current_point = possible_next.clone();
                break;
            }
        }
    }

    cycle
}

fn adjacent(point: &Point, rows: usize, cols: usize) -> Vec<Point> {
    let mut adjacent = Vec::new();

    if point.row + 1 < rows {
        adjacent.push(Point::new(point.row + 1, point.col));
    }

    if let Some(new) = point.row.checked_sub(1) {
        adjacent.push(Point::new(new, point.col));
    }

    if point.col + 1 < cols {
        adjacent.push(Point::new(point.row, point.col + 1));
    }

    if let Some(new) = point.col.checked_sub(1) {
        adjacent.push(Point::new(point.row, new));
    }

    adjacent
}

fn part_1(input: &str) -> u32 {
    let pipes = input
        .split('\n')
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let starting_point = find_starting_point(&pipes).unwrap();

    let cycle = find_cycle(&pipes, &starting_point);

    (cycle.len() / 2) as u32
}

fn part_2(input: &str) -> u32 {
    let mut pipes = input
        .split('\n')
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let starting_point = find_starting_point(&pipes).unwrap();

    let cycle = find_cycle(&pipes, &starting_point);

    let start_to_first_direction = starting_point.direction_to(&cycle[1]);
    let start_to_last_direction = starting_point.direction_to(&cycle[cycle.len() - 1]);

    // Replace starting pipe with relevant symbol
    pipes[starting_point.row][starting_point.col] =
        match (start_to_first_direction, start_to_last_direction) {
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => 'L',
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => '|',
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => 'J',
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => '-',
            (Direction::East, Direction::South) | (Direction::South, Direction::East) => 'F',
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => '7',
            _ => panic!("Jeff"),
        };

    let mut num_inside = 0;

    for (row_index, row) in pipes.iter().enumerate() {
        let mut crossings = 0;
        let mut last_corner = ' ';
        for (col_index, pipe) in row.iter().enumerate() {
            let current_point = Point::new(row_index, col_index);

            if cycle.contains(&current_point) {
                match pipe {
                    'L' | 'F' | '|' => crossings += 1,
                    '7' => {
                        if last_corner == 'F' {
                            crossings += 1;
                        }
                    }
                    'J' => {
                        if last_corner == 'L' {
                            crossings += 1;
                        }
                    }
                    _ => (),
                }

                if ['F', 'L', 'J', '7'].contains(pipe) {
                    last_corner = *pipe;
                }
            } else if crossings % 2 == 1 {
                num_inside += 1;
            }
        }
    }

    num_inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(
            part_2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }
}
