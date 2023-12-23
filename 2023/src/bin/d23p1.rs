#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use maplit::{hashset, hashmap};
use multimap::MultiMap;
use ndarray::*;
use num::iter::RangeInclusive;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    ops::Range,
    str,
};
use std::{collections::VecDeque, iter::Zip};
use tuple::*;

fn run(input: String) -> String {
    let d_right = arr1(&[0, 1 as i64]);
    let d_left = arr1(&[0, -1 as i64]);
    let d_up = arr1(&[-1, 0 as i64]);
    let d_down = arr1(&[1,0 as i64]);
    let directions = vec!(
        (0b1000, &d_up),
        (0b0100, &d_right),
        (0b0010, &d_down),
        (0b0001, &d_left));

    let width = input.find("\n").unwrap();
    let mut grid = Array::zeros((0, width));
    for l in input.lines() {
        let mapped = l.as_bytes().iter().map(|c| match c {
            // direction options in binary, mapping to directions above.
            b'#' => 0b0000,
            b'.' => 0b1111,
            b'^' => 0b1000,
            b'>' => 0b0100,
            b'v' => 0b0010,
            b'<' => 0b0001,
            _ => panic!("invalid")
        }).collect_vec();
        grid.push_row(ArrayView::from(&mapped)).unwrap();
    }
    let shape = grid.shape();
    assert_eq!(shape[0], shape[1]);
    let size = shape[0];

    assert_eq!(grid[(0,1)], 0b1111);
    grid[(0,1)] = 0b0010; // only allow down, just prevent the negative case
    let start_point = arr1(&[0, 1 as i64]);

    assert_eq!(grid[(size-1, size-2)], 0b1111);
    let end_point = arr1(&[size as i64 - 1, size as i64 - 2]);

    let grid_point = |p: &Array1<i64>| grid[(p[0] as usize, p[1] as usize)];

    let mut paths = Vec::new();
    paths.push((start_point.to_owned(), hashset! {start_point.to_owned()}));
    let mut max_path = 0;
    while paths.len() > 0 {
        let mut new_paths = Vec::new();

        for (pt, path) in &paths {
            if pt == end_point {
                max_path = max(max_path, path.len());
                continue;
            }
            let options = grid_point(pt);
            for (mask, dir) in &directions {
                if options & mask > 0 {
                    let next = pt+*dir;
                    if !path.contains(&next) && grid_point(&next) > 0 {
                        let mut new_path = path.clone();
                        new_path.insert(next.clone());
                        new_paths.push((next, new_path));
                    }
                }
            }
        }
        paths = new_paths;
    }
    format!("{}", max_path-1)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), "94");
    }
}
