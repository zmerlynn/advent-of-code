#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use std::iter::Zip;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::{HashMap, HashSet}, cmp::{min,max}, str};
use ndarray::*;
use multimap::MultiMap;

fn run(input: String) -> String {
    let d_right = arr1(&[0, 1 as i64]);
    let d_left = arr1(&[0, -1 as i64]);
    let d_up = arr1(&[-1, 0 as i64]);
    let d_down = arr1(&[1,0 as i64]);
    let directions = [&d_right, &d_left, &d_up, &d_down];
    
    let mut pos = arr1(&[0,0 as i64]);
    let mut min_bounds = arr1(&[0,0 as i64]);
    let mut max_bounds = arr1(&[0,0 as i64]);
    let mut vertices = arr2(&[[0,0]]);
    let mut perimeter = 0;

    for l in input.lines() {
        let (_, _, hex_with_parens) = l.split(" ").collect_tuple().unwrap();
        let hex_bytes = hex_with_parens.trim_matches(|c| c == '(' || c == ')' || c == '#').as_bytes();
        let len_string = str::from_utf8(&hex_bytes[0..5]).unwrap();
        let len = i64::from_str_radix(len_string, 16).unwrap();
        let d = match hex_bytes[5] {
            b'0' => &d_right,
            b'1' => &d_down,
            b'2' => &d_left,
            b'3' => &d_up,
            _ => panic!("ahhh!"),
        };
        println!("{} => len = {}, dir = {}", len_string, len, d);
        
        pos = &pos + len * d;
        perimeter += len;
        vertices.push_row(pos.view()).unwrap();
    }
    assert_eq!(pos, arr1(&[0,0]));
    println!("{}", vertices);

    let mut determinant = 0;
    for row in 0..vertices.shape()[0]-1 {
        determinant += vertices[(row, 0)]*vertices[(row+1,1)] - vertices[(row,1)]*vertices[(row+1,0)];
    }
    println!("{}, {}", determinant, perimeter);
    let twice_interior_area = determinant.abs();

    format!("{}", (twice_interior_area+perimeter)/2+1) // why is this +1 when Pick's theorem is -1?
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "952408144115";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
