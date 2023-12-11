#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
// use std::str;
use std::collections::{HashMap, HashSet};
use std::cmp::max;
use num::abs;

fn run(input: String) -> String {
    let mut galaxies_rows_fixed = Vec::new();
    let mut cols_with_galaxies = HashSet::new();
    let mut row: usize = 0;
    let mut max_col = 0;
    for l in input.lines() {
        let mut col = 0;
        let mut no_galaxies_in_row = true;
        for c in l.chars() {
            if c == '#' {
                galaxies_rows_fixed.push((col, row));
                cols_with_galaxies.insert(col);
                max_col = max(max_col, col);
                no_galaxies_in_row = false;
            }
            col += 1;
        }

        row += 1;
        if no_galaxies_in_row {
            row += 1;
        }
    }

    let mut distortion = 0;
    let mut col_distortions = Vec::new();
    for col in 0..max_col+1 {
        if !cols_with_galaxies.contains(&col) {
            distortion += 1;
        }
        col_distortions.push(distortion);
    }

    let mut galaxies = Vec::new();
    for galaxy in galaxies_rows_fixed {
        galaxies.push((galaxy.0 + col_distortions[galaxy.0], galaxy.1));
    }

    println!("{:?}", galaxies);
    let mut out = 0;
    for i in 1..galaxies.len() {
        for j in 0..i {
            let a = galaxies[i];
            let b = galaxies[j];
            out += abs(a.0 as i32 - b.0 as i32) + abs(a.1 as i32 - b.1 as i32);
        }
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "374";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
