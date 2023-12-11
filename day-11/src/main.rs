use core::iter::Iterator;
use derive_more::Constructor;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input, 1_000_000));
}

#[derive(Constructor, Debug)]
struct Vec2 {
    row: usize,
    col: usize,
}

impl Vec2 {
    fn manhattan_distance(&self, other: &Vec2) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn find_rows_to_expand(universe: &[Vec<char>]) -> Vec<usize> {
    universe
        .iter()
        .positions(|row| row.iter().all(|char| *char == '.'))
        .collect_vec()
}

fn find_cols_to_expand(universe: &[Vec<char>]) -> Vec<usize> {
    (0..universe[0].len())
        .into_iter()
        .filter(|col_index| {
            (0..universe.len()).all(|row_index| universe[row_index][*col_index] == '.')
        })
        .collect_vec()
}

fn find_galaxies(universe: &[Vec<char>]) -> Vec<Vec2> {
    universe
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .positions(|char| *char == '#')
                .map(move |col_index| Vec2::new(row_index, col_index))
        })
        .flatten()
        .collect_vec()
}

// After solving part 2, the solution to part 1 becomes easier
fn part_1(input: &str) -> usize {
    part_2(input, 2)
}

fn part_2(input: &str, gap: usize) -> usize {
    let universe = input
        .split('\n')
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let galaxies = find_galaxies(&universe);

    let rows_to_expand = find_rows_to_expand(&universe);
    let cols_to_expand = find_cols_to_expand(&universe);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(x, y)| {
            let mut distance = x.manhattan_distance(y);

            for col_to_expand in cols_to_expand.iter() {
                if (x.col + 1..=y.col).contains(col_to_expand)
                    || (y.col + 1..=x.col).contains(col_to_expand)
                {
                    distance += gap - 1;
                }
            }

            for row_to_expand in rows_to_expand.iter() {
                if (x.row + 1..=y.row).contains(row_to_expand)
                    || (y.row + 1..=x.row).contains(row_to_expand)
                {
                    distance += gap - 1;
                }
            }

            distance
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(TEST_VECTOR), 374);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(TEST_VECTOR, 10), 1030);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2(TEST_VECTOR, 100), 8410);
    }
}
