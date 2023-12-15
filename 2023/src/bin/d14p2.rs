#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashMap};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr};

// use std::str;
// use std::collections::{HashMap, HashSet};

fn tilt_up(grid: &mut Array2<u8>) {
    let shape = grid.shape();
    let (height, width) = (shape[0], shape[1]);
    for row in 0..height {
        for col in 0..width {
            if grid[(row,col)] > 0 {
                continue;
            }
            for other_row in row+1..height {
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
}

fn load(grid: &Array2<u8>) -> usize {
    let height = grid.shape()[0];
    let mut tot = 0;
    for ((row, col), v) in grid.indexed_iter() {
        if *v == 1 {
            tot += height - row;
        }
    }
    tot
}

fn run(input: String) -> String {
    let width = input.find("\n").unwrap();
    let mut grid = Array2::zeros((0, width));
    for l in input.lines() {
        let mapped = l.as_bytes().iter().map(|c| match c {
            b'#' => 8 as u8,
            b'O' => 1,
            b'.' => 0,
            _ => panic!("invalid")
        }).collect_vec();
        grid.push_row(ArrayView::from(&mapped)).unwrap();
    }

    // println!("{}", grid);
    // println!("{}", grid);

    // for y in 0..grid.grid.len() {
    //     for x in 0..grid.grid[y].len() {
    //         if grid.grid[y][x] == 'O' {
    //             out += height - y;
    //         }
    //     }
    // }

    let mut visited = HashMap::new();

    let mut residual = -1;
    let mut cycle_len = 0;
    let target_cycles = 1000000000;
    for cycle_count in 1..target_cycles {
        for dir in 0..4 {
            tilt_up(&mut grid);
            // println!("after tilt:\n{}\n", grid);
            grid.invert_axis(Axis(0));
            grid.swap_axes(0, 1);
            // println!("after rotation:\n{}\n", grid);
        }    
        println!("after cycle {}:\n{}\n", cycle_count, grid);
        if cycle_len > 0 {
            println!("cycle_count % cycle_len = {}", cycle_count % cycle_len);
            if cycle_count % cycle_len == residual {
                break
            }
        } else if let Some(cycle_start) = visited.insert(grid.clone(), cycle_count) {
            cycle_len = cycle_count - cycle_start;
            residual = (target_cycles) % cycle_len;

            println!("== cycle_start: {}, cycle_len: {}, residual: {} ==\n", cycle_start, cycle_len, residual);
        }
    }

    format!("{}", load(&grid))
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "64";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
