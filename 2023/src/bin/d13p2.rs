#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::cmp::min;

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn distance(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars()).fold(0, |s, (ac, bc)| s + { if ac == bc { 0 } else { 1 }})
}

fn split_row(raster: &Vec<String>) -> usize {
    // assert that row i is the split row
    for i in 1..raster.len() {
        // d is the delta off row i to check
        // at i == 1, d checks only row 0/1
        let mut tot_dist = 0;
        for d in 0..min(i, raster.len()-i) {
            let i_above = i-1-d;
            let i_below = i+d;
            tot_dist += distance(&raster[i_above], &raster[i_below]);
        }
        if tot_dist == 1 {
            return i
        }
    }
    0
}

fn reflections(raster: Vec<String>) -> usize {
    // println!("{:?}", raster);
    let horiz = split_row(&raster);
    // split occurs horiz-1 and horiz, so there are horiz rows above
    if horiz > 0 {
        return horiz * 100
    }

    let cols = raster[0].len();
    let mut transposed_raster = vec![String::new(); cols];
    for row in raster {
        for (i, c) in row.chars().enumerate() {
            transposed_raster[i] += &c.to_string();
        }
    }

    let vert = split_row(&transposed_raster);
    assert!(vert > 0);
    vert
}

fn run(input: String) -> String {
    let mut out = 0;
    let mut raster = Vec::new();
    for l in input.lines() {
        if l == "" {
            out += reflections(raster);
            raster = Vec::new();
            continue;
        }
        raster.push(l.to_string());
    }
    assert_eq!(raster.len(), 0);

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "400";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
