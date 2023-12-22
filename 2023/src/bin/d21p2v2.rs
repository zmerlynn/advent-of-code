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
    str, ops::Rem,
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
    assert_eq!(grid.shape()[0], grid.shape()[1]);
    let size = grid.shape()[0];

    let mut starting_point = arr1(&[i64::MAX, i64::MAX]);
    for (i, v) in grid.indexed_iter_mut() {
        if *v < 0 {
            *v = 0;
            starting_point = arr1(&[i.0 as i64, i.1 as i64]);
        }
    }
    assert!(starting_point[0] != i64::MAX);

    let mut edge = hashset![starting_point.clone()];
    let mut visited = [HashSet::new(), HashSet::new()];

    for i in 1..=5000 {
        let mut new_edge = HashSet::new();
        for pt in &edge {
            for dir in directions {
                let new_pt = pt + dir;
                let in_grid = new_pt.map(|x| x.rem_euclid(size as i64) as usize);
                if grid[(in_grid[0], in_grid[1])] == 0 && !visited[i % 2].contains(&new_pt) {
                    new_edge.insert(new_pt.clone());
                    visited[i % 2].insert(new_pt);
                }
            }
        }
        if i % 1000 == 0 {
            println!("i = {}, edge.len() = {}, new_edge.len() = {}, visited[0].len() = {}", i, edge.len(), new_edge.len(), visited[0].len());
        }
        edge = new_edge;
    }

    format!("{}", visited[0].len())
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
