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
    println!("{}\n\n{}", positions, velocities);
    let positions = positions.slice(s![0..2, ..]);
    let velocities = velocities.slice(s![0..2, ..]);

    let stones = positions.shape()[1];
    let care_about = [200000000000000f64,400000000000000f64];
    // let care_about = [7f64,27f64];
    let mut out = 0;
    for i in 0..stones {
        let p_i = positions.column(i);
        let v_i = velocities.column(i);

        for j in i+1..stones {
            let p_j = positions.column(j);
            let v_j = velocities.column(j);

            // based on https://stackoverflow.com/questions/3252194/numpy-and-line-intersections
            let d_p = &p_i-&p_j;

            let mut perp_v_i = Array1::<f64>::zeros(2);
            perp_v_i[0] = -v_i[1];
            perp_v_i[1] = v_i[0];

            let denom = perp_v_i.dot(&v_j);
            let num = perp_v_i.dot(&d_p);

            if denom.abs() < f64::EPSILON {
                continue;
            }
            let isect = (&num / &denom)*&v_j + &p_j;
            let when_i = (&isect-&p_i) / &v_i;
            let when_j = (&isect-&p_j) / &v_j;
            if when_i[0] > f64::EPSILON && when_j[0] > f64::EPSILON && care_about[0] < isect[0] && isect[0] < care_about[1] && care_about[0] < isect[1] && isect[1] < care_about[1] {
                println!("({}, {}) denom = {}, isect = {}, when_i = {}, when_j = {}", i, j, denom, isect, when_i, when_j);
                out += 1;
            }
        }
    }

    format!("{}", out)
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
