#![feature(iter_repeat_n)]

use core::panic;
use itertools::Itertools;
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
    hand_type: HandType,
}

impl Hand {
    fn new(cards: Vec<u32>, bid: u32) -> Self {
        let hand_type = HandType::from(cards.as_slice());

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl From<&[u32]> for HandType {
    fn from(cards: &[u32]) -> Self {
        let cards_set = HashSet::<&u32>::from_iter(cards);

        match cards_set.len() {
            1 => HandType::FiveOfAKind, // All cards are the same
            2 => {
                // HandType::FourOfAKind || HandType::FullHouse
                let first_card_type_count = cards.iter().filter(|card| **card == cards[0]).count();

                match first_card_type_count {
                    2 | 3 => HandType::FullHouse,
                    1 | 4 => HandType::FourOfAKind,
                    _ => panic!("Bad bad news..."),
                }
            }
            3 => {
                // HandType::TwoPair || HandType::ThreeOfAKind
                let mut card_type_counts = cards_set
                    .iter()
                    .map(|card_type| cards.iter().filter(|card| card_type == card).count() as u32)
                    .collect::<Vec<u32>>();

                card_type_counts.sort();

                match card_type_counts.as_slice() {
                    [1, 2, 2] => HandType::TwoPair,
                    [1, 1, 3] => HandType::ThreeOfAKind,
                    _ => panic!("jeff"),
                }
            }
            4 => HandType::OnePair,  // All cards are different but one
            5 => HandType::HighCard, // All cards are different
            _ => panic!("HELP"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .split('\n')
        .map(|line| {
            let (cards, bid) = line.trim().split_once(" ").unwrap();

            Hand::new(
                cards
                    .chars()
                    .into_iter()
                    .map(|card| {
                        if let Some(digit) = card.to_digit(10) {
                            digit
                        } else {
                            match card {
                                'T' => 10,
                                'J' => 11,
                                'Q' => 12,
                                'K' => 13,
                                'A' => 14,
                                _ => panic!("Bad card type: {}", card),
                            }
                        }
                    })
                    .collect(),
                bid.parse().unwrap(),
            )
        })
        .collect_vec()
}

fn sort_hands(hands: &mut [Hand]) {
    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
                if a_card == b_card {
                    continue;
                }

                return a_card.cmp(b_card);
            }
        }

        (a.hand_type as u8).cmp(&(b.hand_type as u8))
    });
}

fn calculate_bid_total(hands: &[Hand]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1) as u32)
        .sum()
}

fn calculate_highest_card_type_with_jokers(hand: &mut Hand) {
    const JOKER: u32 = 11;

    let joker_indices = hand
        .cards
        .iter_mut()
        .positions(|card| *card == JOKER)
        .collect_vec();

    if joker_indices.is_empty() {
        return;
    }

    for joker_index in joker_indices.iter() {
        hand.cards[*joker_index] = 1;
    }

    let mut new_cards = hand.cards.clone();

    // Cartesian product in real life
    // Brute force with blazingly fast LLVM rust code
    // I am a rustacean
    for new_card_values in
        std::iter::repeat_n(2..=14, joker_indices.len()).multi_cartesian_product()
    {
        for (joker_index, new_card_value) in new_card_values.into_iter().enumerate() {
            new_cards[joker_indices[joker_index]] = new_card_value;

            let new_hand_type = HandType::from(new_cards.as_slice());

            if new_hand_type as u8 > hand.hand_type as u8 {
                hand.hand_type = new_hand_type;
            }

            if new_hand_type == HandType::FiveOfAKind {
                return;
            }
        }
    }
}

fn part_1(input: &str) -> u32 {
    let mut hands = parse_hands(input);
    sort_hands(&mut hands);
    calculate_bid_total(&hands)
}

fn part_2(input: &str) -> u32 {
    let mut hands = parse_hands(input);

    for hand in hands.iter_mut() {
        calculate_highest_card_type_with_jokers(hand);
    }

    sort_hands(&mut hands);
    calculate_bid_total(&hands)
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
        assert_eq!(part_2(TEST_VECTOR), 5905);
    }
}
