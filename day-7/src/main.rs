use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn part_1(input: &str) -> u32 {
    let hands = input
        .split('\n')
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            Hand {
                cards: cards
                    .chars()
                    .into_iter()
                    .map(|card| {
                        if let Some(digit) = card.to_digit(10) {
                            digit
                        } else {
                            match card {
                                'A' => 1,
                                'T' => 10,
                                'J' => 11,
                                'Q' => 12,
                                'K' => 13,
                                _ => panic!("Bad card type: {}", card),
                            }
                        }
                    })
                    .collect(),
                bid: bid.parse().unwrap(),
            }
        })
        .map(|hand| {
            let cards_set = HashSet::<u32>::from_iter(hand.cards);
            let hand_type = match cards_set.len() {
                1 => HandType::FiveOfAKind, // All cards are the same
                2 => HandType::FourOfAKind || HandType::ThreeOfAKind || HandType::FullHouse,
                3 => HandType::TwoPair || HandType::ThreeOfAKind,
                4 => HandType::OnePair, // All cards are different but one
                5 => HandType::HighCard, // All cards are different
                _ => 
            };

            (hand, hand_type)
        });
    // .collect::<Vec<Hand>>();

    0
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(TEST_VECTOR), 6440);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(TEST_VECTOR), 0);
    }
}
