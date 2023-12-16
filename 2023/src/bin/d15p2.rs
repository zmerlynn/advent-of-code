#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashMap};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr};

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0 as u8, |accum, c| {
        accum.wrapping_add(*c).wrapping_mul(17)
    }) as usize
}
fn run(input: String) -> String {
    let mut hash_map: HashMap<usize, Vec<(&str, u8)>> = HashMap::new();
    for s in input.lines().next().unwrap().split(",") {
        let mut action_idx = 0;
        if let Some(idx) = s.find("=") {
            action_idx = idx;
        } else if let Some(idx) = s.find("-") {
            action_idx = idx;
        }
        assert!(action_idx > 0);

        let label = &s[0..action_idx];
        let label_hash = hash(label);
        let e = hash_map.entry(label_hash).or_insert(Vec::new());

        let mut label_at = None;
        for (i, (l, v)) in e.iter().enumerate() {
            if *l == label {
                label_at = Some(i);
            }
        }

        
        let action = &s[action_idx..action_idx+1];
        if action == "=" {
            let new_value = *&s[action_idx+1..action_idx+2].parse::<u8>().unwrap();
            if let Some(label_at) = label_at {
                e[label_at] = (label, new_value);
            } else {
                e.push((label, new_value));
            }
        } else {
            assert_eq!(action, "-", "{} is weird", s);
            if let Some(label_at) = label_at {
                e.remove(label_at);
            }
        }
    }
    println!("{:?}", hash_map);

    let mut out = 0;
    for (box_number, e) in hash_map {
        for (i, (_, v)) in e.iter().enumerate() {
            out += (box_number+1) * (i+1) * (*v as usize);
        }
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "145";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
