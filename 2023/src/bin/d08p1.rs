#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
// use std::str;
use std::collections::HashMap;

fn run(input: String) -> String {
    let mut out = 0;
    let mut line_iter = input.lines();
    let dirs = line_iter.next().unwrap();
    line_iter.next();

    let mut tree = HashMap::new();
    for l in line_iter {
        let (source, dests) = l.split(" = ").collect_tuple().unwrap();
        let (left_with, right_with) = dests.split(", ").collect_tuple().unwrap();
        let left = left_with.split("(").collect_tuple::<(&str, &str)>().unwrap().1;
        let right = right_with.split(")").collect_tuple::<(&str, &str)>().unwrap().0;

        tree.insert(source, (left, right));
    }

    let mut loc = "AAA";
    loop {
        for dir in dirs.chars() {
            let node = tree.get(loc).unwrap();
            // println!("loc: {}, node: ({}, {})", loc, node.0, node.1);

            if dir == 'L' {
                loc = node.0;
            } else {
                loc = node.1;
            }
            out += 1;
        }
        if loc == "ZZZ" {
            break;
        }
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
