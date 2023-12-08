#![allow(dead_code)]
#[macro_use]
extern crate maplit;

use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    strength: i8, // 0 = high card, 1 = one pair, etc.
    bid: i32,
}

impl Hand {
    fn new(cards: &str, bid: i32) -> Hand {
        let card_values: HashMap<char, u8> = hashmap! {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
        };
        let cards: Vec<u8> = cards.chars().map(|c| card_values[&c]).collect();
    
        let strength = {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for c in &cards {
                *counts.entry(*c).or_insert(0) += 1;
            }
            let mut counts = counts.iter().collect::<Vec<(&u8, &u8)>>();
            counts.sort_by(
                |&(_, a), &(_, b)| b.cmp(a)
            );
            if *counts[0].1 == 1 {
                assert_eq!(counts.len(), 5);
                0 // high card
            } else if *counts[0].1 == 2 {
                if *counts[1].1 == 1 {
                    assert_eq!(counts.len(), 4);
                    1 // one pair
                } else {
                    assert_eq!(*counts[1].1, 2);
                    assert_eq!(counts.len(), 3);
                    2 // two pair
                }
            } else if *counts[0].1 == 3 {
                if *counts[1].1 == 1 {
                    assert_eq!(counts.len(), 3);
                    3 // three of a kind
                } else {
                    assert_eq!(counts.len(), 2);
                    4 // full house
                }
            } else if *counts[0].1 == 4 {
                assert_eq!(counts.len(), 2);
                5 // four of a kind
            } else {
                assert_eq!(counts.len(), 1);
                6 // five of a kind
            }
        };
        return Hand { cards: cards.to_owned(), strength: strength, bid: bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.strength.cmp(&other.strength).then(self.cards.cmp(&other.cards)))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.strength == other.strength
    }
}

fn run(input: String) -> String {
    let mut hands = Vec::new();
    for l in input.lines() {
        let (cards, bid) = l.split(" ").collect_tuple().unwrap();
        hands.push(Hand::new(cards, bid.parse().unwrap()))
    }
    hands.sort_by(|a,b| a.partial_cmp(b).unwrap()); // Implement Ord properly at some point
    let mut tot = 0;
    for i in 0..hands.len() {
        tot += ((i as i32)+1)*hands[i].bid;
    }
    format!("{}", tot)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "6440";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
