#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn infer_next(values: Vec<i64>) -> i64 {
    let mut levels = Vec::new();
    levels.push(values);
    while levels.last().unwrap().iter().any(|x| *x != 0) {
        let last_level = levels.last().unwrap();
        levels.push(last_level.iter().zip(last_level[1..].iter()).map(|(a, b)| b-a).collect());
    }
    let mut carry_up = 0;
    for i in (0..levels.len()-1).rev() {
        carry_up = levels[i][0] - carry_up;
    }
    println!("{:?}, {}", levels, carry_up);
    carry_up
}

fn run(input: String) -> String {
    let mut out = 0;
    for l in input.lines() {
        out += infer_next(parse_ints(l));
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "2";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
