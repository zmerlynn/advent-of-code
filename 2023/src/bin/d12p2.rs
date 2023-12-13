#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn viable(row: &[u8], runs: &Vec<i64>) -> (bool, bool) {
    let mut run_idx = 0;
    let mut run = 0;
    for c in row.iter() {
        if *c == b'#' {
            run += 1;
            if run_idx >= runs.len() || run > runs[run_idx] {
                return (false, false);
            }
        } else if *c == b'.' {
            if run > 0 {
                if run != runs[run_idx] {
                    return (false, false)
                }
                run_idx += 1;
            }
            run = 0;
        } else {
            assert_eq!(*c, b'?');
            return (true, run_idx < runs.len()-1 || runs[runs.len()-1] - run > 0);
        }
    }
    (run_idx == runs.len(), false)
}

fn arrangements(row: &mut [u8], unknowns: &[usize], runs: &Vec<i64>) -> i32 {
    let (is_viable, place_more) = viable(row, runs);
    if !is_viable {
        // println!("- {} {:?}", std::str::from_utf8(row).unwrap(), runs);
        return 0;
    }
    if unknowns.len() == 0 {
        // println!("+ {} {:?}", std::str::from_utf8(row).unwrap(), runs);
        return 1;
    }
    let idx = unknowns[0];
    let mut tot = 0;
    if place_more {
        row[idx] = b'#';
        tot += arrangements(row, &unknowns[1..], runs);
    }
    row[idx] = b'.';
    tot += arrangements(row, &unknowns[1..], runs);
    row[idx] = b'?';
    tot
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
        let mut row = row.to_owned() + "?" + row + "?" + row + "?" + row + "?" + row + ".";
        let mut unknowns = Vec::new();
        for (i, c) in row.chars().enumerate() {
            if c == '?' {
                unknowns.push(i);
            }
        }
        println!("=== {} ; {:?} ; {}", row, runs, unknowns.len());
        unsafe {
            let mut row = row.as_bytes_mut();
            out += arrangements(&mut row, &unknowns, &runs);
        }
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
}
