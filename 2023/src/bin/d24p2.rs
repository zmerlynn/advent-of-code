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
use ndarray_linalg::*;
use num::{iter::RangeInclusive, complex::ComplexFloat};
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    ops::{Range, Sub},
    str,
};
use std::{collections::VecDeque, iter::Zip};
use tuple::*;

fn run(input: String) -> String {
    let mut positions = Array2::<f64>::zeros((3,0));
    let mut velocities = Array2::<f64>::zeros((3,0));
    for l in input.lines() {
        let (p_str, v_str) = l.split(" @ ").collect_tuple().unwrap();
        let p = parse_ints_generic(p_str, ", ").iter().map(|i| *i as f64).collect::<Vec<f64>>();
        let v = parse_ints_generic(v_str, ", ").iter().map(|i| *i as f64).collect::<Vec<f64>>();
        positions.push_column(ArrayView::from(&p)).unwrap();
        velocities.push_column(ArrayView::from(&v)).unwrap();
    }
    let stones = positions.shape()[1];

    // Each hailstone path is described by p_i + t * v_i
    // We're looking for an intersecting path, call it p_U + t * v_U
    // If each intersection occur at time t_i, we effectively have a system:
    // for all i, p_i + t_i * v_i = p_U + t_i * v_U, e.g.:
    //   p_0 + t_0 * v_0 = p_U + t_0 * v_U
    //   p_1 + t_1 * v_1 = p_U + t_1 * v_U
    //   ...
    //
    // px_0 + t_0 * vx_0 = px_U + t_0 * vx_U // unknowns t_0, px_U, vx_U
    // px_0-px_U = t_0*(vx_U-vx_0)
    // t_0 = (px_0-px_U)/(vx_U-vx_0)
    // t_0 = (py_0-py_U)/(vy_U-vy_0)

    format!("{}", 0)
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
        assert_eq!(result.trim(), "2");
    }
}
