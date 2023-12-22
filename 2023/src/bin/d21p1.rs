#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use maplit::hashset;
use multimap::MultiMap;
use ndarray::*;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    str,
};
use std::{collections::VecDeque, iter::Zip};
use tuple::*;

fn run(input: String) -> String {
    let d_right = arr1(&[0, 1 as i64]);
    let d_left = arr1(&[0, -1 as i64]);
    let d_up = arr1(&[-1, 0 as i64]);
    let d_down = arr1(&[1, 0 as i64]);
    let directions = [&d_right, &d_left, &d_up, &d_down];

    let width = input.find("\n").unwrap();
    let mut grid = Array::zeros((0, width));
    for l in input.lines() {
        let mapped = l
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'#' => 1,
                b'.' => 0,
                b'S' => -1,
                _ => panic!("invalid"),
            })
            .collect_vec();
        grid.push_row(ArrayView::from(&mapped)).unwrap();
    }
    let shape = grid.shape();
    let shape = [shape[0] as i64, shape[1] as i64];

    let mut starting_point = arr1(&[i64::MAX, i64::MAX]);
    for (i, v) in grid.indexed_iter_mut() {
        if *v < 0 {
            *v = 0;
            starting_point = arr1(&[i.0 as i64, i.1 as i64]);
        }
    }
    assert!(starting_point[0] != i64::MAX);

    let mut edge = hashset![starting_point.clone()];

    for i in 0..64 {
        let mut new_edge = HashSet::new();
        for pt in &edge {
            for dir in directions {
                let new_pt = pt + dir;
                if 0 <= new_pt[0]
                    && new_pt[0] < shape[0]
                    && 0 <= new_pt[1]
                    && new_pt[1] < shape[1]
                    && grid[(new_pt[0].try_into().unwrap(), new_pt[1].try_into().unwrap())] == 0
                {
                    new_edge.insert(new_pt);
                }
            }
        }
        // println!("edge = {:?}, new_edge = {:?}", edge, new_edge);
        edge = new_edge;

        // for row in 0..shape[0] {
        //     for col in 0..shape[1] {
        //         if edge.contains(&arr1(&[row, col])) {
        //             print!("O");
        //         } else {
        //             let pt = grid[(row as usize, col as usize)];
        //             if pt > 0 {
        //                 print!("#");
        //             } else {
        //                 print!(".");
        //             }
        //         }
        //     }
        //     print!("\n");
        // }
    }

    format!("{}", edge.len())
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
        assert_eq!(result.trim(), "16");
    }
}
