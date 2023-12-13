#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn check_match(pattern: &str, guess: &str) -> bool {
    assert_eq!(pattern.len(), guess.len());
    // println!("{} vs {}", pattern, guess);
    pattern.chars().zip(guess.chars()).all(|(c_p, c_g)| (c_p == '?' || c_p == c_g))
}

// .??..??...?##. => len 14 => [a,1,b,1,c,3,*]
// 
fn arrangements<'a>(mut cache: &mut HashMap<(&'a str, &'a[i64]), i64>, row: &'a str, runs: &'a[i64]) -> i64 {
    if let Some(x) = cache.get(&(&row, runs)) {
        return *x;
    }
    if runs.len() == 0 {
        if row.contains("#") {
            return 0;
        } else {
            return 1;
        }
    }
    // For each layer, pick a dot_length to precede runs[0]. Then we'll see
    // if what's in the row pattern could possibly match before iterating.
    let runs_total: i64 = runs.iter().sum();
    let max_space = 1 + row.len() as i64 - (runs_total + runs.len() as i64);
    let mut total_arrangements = 0;
    for space_length in 1..max_space {
        // here we check for a variable number of spaces, the run length in blocks, then a space
        // we present all of that to check_match to make sure we end with a space, but when
        // we recurse we take the ending space back off
        let row_start = std::iter::repeat(".").take(space_length as usize).collect::<String>()
         + &std::iter::repeat("#").take(runs[0] as usize).collect::<String>() + ".";
        if check_match(&row[0..row_start.len()], &row_start) {
            total_arrangements += arrangements(&mut cache, &row[row_start.len()-1..], &runs[1..])
        }
    }
    cache.insert((row, runs), total_arrangements);
    total_arrangements
}

fn run(input: String) -> String {
    let mut out = 0;
    for l in input.lines() {
        let (row, counts_str) = l.split(" ").collect_tuple().unwrap();
        let runs_initial = parse_ints_generic(counts_str, ",");
        let mut runs = Vec::new();
        for i in 0..5 {
            runs.extend(runs_initial.iter());
        }
        let row = ".".to_string() + row + "?" + row + "?" + row + "?" + row + "?" + row + ".";
        let mut cache: HashMap<(&str, &[i64]), i64> = HashMap::new();
        out += arrangements(&mut cache, &row, &runs);
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "525152";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }

    #[test]
    fn part1_works() {
        let mut cache: HashMap<(&str, &[i64]), i64> = HashMap::new();
        let runs: Vec<i64> = vec![1,1,3];
        assert_eq!(arrangements(&mut cache, ".??..??...?##.", &runs), 4);
        let runs: Vec<i64> = vec![1,3,1,6];
        assert_eq!(arrangements(&mut cache, ".?#?#?#?#?#?#?#?.", &runs), 1);
        let runs: Vec<i64> = vec![3,2,1];
        assert_eq!(arrangements(&mut cache, ".?###????????.", &runs), 10);
    }
}
