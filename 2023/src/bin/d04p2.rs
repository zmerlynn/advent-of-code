use aoc::*;
use itertools::Itertools;
use std::str;
use std::collections::{HashMap, HashSet};

fn parse_ints(inp: &str) -> Vec<i32> {
    inp.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap()).collect()
}

fn run(input: String) -> String {
    let mut t: u32 = 0;
    let mut cards_won: HashMap<u8, u32> = HashMap::new();
    for l in input.lines() {
        let (card_string, sets_string) = l.split(":").collect_tuple().unwrap();
        let card = str::from_utf8(&card_string.as_bytes()[5..]).unwrap().trim().parse::<u8>().unwrap();
        let (wants_string, gots_string) = sets_string.split("|").collect_tuple().unwrap();
        let wants = parse_ints(wants_string);
        let wantset: HashSet<&i32> = HashSet::from_iter(wants.iter());
        let gots = parse_ints(gots_string);
        let copies_of_card = cards_won.get(&card).unwrap_or(&0)+1;

        let mut winnings: u8 = 0;
        for got in &gots {
            if wantset.contains(got) {
                winnings += 1;
            }
        }
        for c in (card+1)..(card+1+winnings) {
            *cards_won.entry(c).or_insert(0) += copies_of_card;
        }

        // println!("card: {:?}, copies_of_card: {:?}, winnings: {:?}", card, copies_of_card, winnings);

        t += copies_of_card;
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "30";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
