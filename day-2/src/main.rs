use core::panic;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input, 12, 13, 14));
    println!("Part 2 = {}", part_2(input));
}

fn part_1(input: &str, red: u32, green: u32, blue: u32) -> u32 {
    input
        .split("\n")
        .filter_map(|game| {
            let (game_id, game) = game.split_once(":").unwrap();
            let game_id = game_id
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            if game_possible(game, red, green, blue) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn game_possible(game: &str, red: u32, green: u32, blue: u32) -> bool {
    for subset in game.trim().split(";") {
        let colours = subset.split(",");

        for count_colour in colours {
            let (count, colour) = count_colour.trim().split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();

            let compare_against = match colour {
                "red" => red,
                "green" => green,
                "blue" => blue,
                _ => panic!("New colour discovered {}", colour),
            };

            if count > compare_against {
                return false;
            }
        }
    }

    true
}

fn part_2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|game| {
            let (_, game) = game.split_once(":").unwrap();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            game.trim().split(";").for_each(|subset| {
                let colours = subset.split(",");

                colours.for_each(|count_colour| {
                    let (count, colour) = count_colour.trim().split_once(" ").unwrap();
                    let count = count.parse::<u32>().unwrap();

                    let compare_against = match colour {
                        "red" => &mut min_red,
                        "green" => &mut min_green,
                        "blue" => &mut min_blue,
                        _ => panic!(),
                    };

                    *compare_against = (*compare_against).max(count);
                });
            });

            min_red * min_green * min_blue
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
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                12,
                13,
                14
            ),
            8
        )
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        )
    }
}
