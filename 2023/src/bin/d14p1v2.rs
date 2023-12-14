#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::fmt::{Display, Formatter, Result};
use ndarray::{Array, ArrayView, array};

// use std::str;
// use std::collections::{HashMap, HashSet};

fn run(input: String) -> String {
    let width = input.find("\n").unwrap();
    let mut grid = Array::zeros((0, width));
    for l in input.lines() {
        let mapped = l.as_bytes().iter().map(|c| match c {
            b'#' => 8,
            b'O' => 1,
            b'.' => 0,
            _ => panic!("invalid")
        }).collect_vec();
        grid.push_row(ArrayView::from(&mapped)).unwrap();
    }
    let shape = grid.shape();
    assert_eq!(shape[0], shape[1]);
    let size = shape[0];

    // println!("{}", grid);
    for row in 0..size {
        for col in 0..size {
            if grid[(row,col)] > 0 {
                continue;
            }
            for other_row in row+1..size {
                match grid[(other_row,col)] {
                    0 => continue,
                    8 => break,
                    1 => {
                        grid.swap((row,col), (other_row,col));
                    },
                    _ => panic!("invalid grid point")
                }
            }
        }
    }
    // println!("{}", grid);

    let mut out = 0;
    for ((row, col), v) in grid.indexed_iter() {
        if *v == 1 {
            out += size - row;
        }
    }

    // for y in 0..grid.grid.len() {
    //     for x in 0..grid.grid[y].len() {
    //         if grid.grid[y][x] == 'O' {
    //             out += height - y;
    //         }
    //     }
    // }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "136";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
