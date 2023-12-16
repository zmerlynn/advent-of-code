#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashMap};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr};

fn run(input: String) -> String {
    let mut out: u32 = 0;
    for s in input.lines().next().unwrap().split(",") {
        let h = s.as_bytes().iter().fold(0 as u8, |accum, c| {
            accum.wrapping_add(*c).wrapping_mul(17)
        });
        out += h as u32;
        // println!("{}, {}", s, h);
    }
    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "1320";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
