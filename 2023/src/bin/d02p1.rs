#[macro_use]
extern crate maplit;

use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;

fn run(input: String) -> String {
    let maximums: HashMap<&str, u8> = hashmap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    };

    let mut t: u32 = 0;
    for l in input.lines() {
        let (game_string, sets_string) = l.split(":").collect_tuple().unwrap();
        let game = game_string
            .split(" ")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
            .1
            .parse::<u8>()
            .unwrap();
        let sets: Vec<&str> = sets_string.trim().split(";").map(|s| s.trim()).collect();

        let mut good = true;
        for set in sets {
            let by_color: Vec<(&str, &str)> = set
                .split(",")
                .map(|s| s.trim().split(" ").collect_tuple::<(&str, &str)>().unwrap())
                .collect();
            for (count_string, color) in by_color {
                if count_string.parse::<u8>().unwrap() > *maximums.get(color).unwrap() {
                    good = false;
                }
            }
        }
        if good {
            t += game as u32
        }
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "8";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
