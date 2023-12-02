use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp;

fn run(input: String) -> String {
    let mut t: u32 = 0;
    for l in input.lines() {
        let (game_string, sets_string) = l.split(":").collect_tuple().unwrap();
        let _game = game_string
            .split(" ")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
            .1
            .parse::<u8>()
            .unwrap();
        let sets: Vec<&str> = sets_string.trim().split(";").map(|s| s.trim()).collect();

        let mut maximums = HashMap::new();

        for set in sets {
            let by_color: Vec<(&str, &str)> = set
                .split(",")
                .map(|s| s.trim().split(" ").collect_tuple::<(&str, &str)>().unwrap())
                .collect();
            for (count_string, color) in by_color {
                let count = count_string.parse::<u8>().unwrap();
                let m = maximums.entry(color).or_insert(count);
                *m = cmp::max(*m, count);
            }
        }
        t += *maximums.get("red").unwrap_or(&0) as u32
            * *maximums.get("green").unwrap_or(&0) as u32
            * *maximums.get("blue").unwrap_or(&0) as u32;
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "2286";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
