#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn arrangements(row: &mut [u8], unknowns: &[usize], counts: &Vec<i64>) -> i32 {
    if unknowns.len() == 0 {
        let mut counts_idx = 0;
        let mut run = 0;
        for c in row.iter() {
            if *c == b'#' {
                run += 1;
            } else {
                if run > 0 {
                    if counts_idx >= counts.len() || run != counts[counts_idx] {
                        return 0;
                    }
                    counts_idx += 1;
                }
                run = 0;
            }
        }
        if counts_idx != counts.len() {
            return 0;
        }
        // println!("{} {:?}", std::str::from_utf8(row).unwrap(), counts);
        return 1;
    }
    let idx = unknowns[0];
    let mut tot = 0;
    row[idx] = b'#';
    tot += arrangements(row, &unknowns[1..], counts);
    row[idx] = b'.';
    tot += arrangements(row, &unknowns[1..], counts);
    tot
}

fn run(input: String) -> String {
    let mut out = 0;
    for l in input.lines() {
        let (row, counts_str) = l.split(" ").collect_tuple().unwrap();
        let mut unknowns = Vec::new();
        for (i, c) in row.chars().enumerate() {
            if c == '?' {
                unknowns.push(i);
            }
        }
        println!("{}", unknowns.len());
        let counts = parse_ints_generic(counts_str, ",");
        unsafe {
            let mut row = row.to_owned() + ".";
            let mut row = row.as_bytes_mut();
            out += arrangements(&mut row, &unknowns, &counts);
        }
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "21";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
