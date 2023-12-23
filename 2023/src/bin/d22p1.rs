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
    let mut bricks = Vec::new();
    for l in input.lines() {
        let (low_coords, high_coords) = l.split("~").collect_tuple().unwrap();
        let low = parse_ints_generic(low_coords, ",");
        let high = parse_ints_generic(high_coords, ",");
        bricks.push((
            low[0]..high[0]+1, 
            low[1]..high[1]+1,
            low[2]..high[2]+1,
        ));
    }

    // Sort the bricks by the highest point in the brick. We'll use this as a minor optimization
    // when figuring out the brick relationships as it means that a brick at index i only has
    // interactions with bricks with index > i. 
    //
    // After this sort, index in bricks is stable, i.e. bricks[0] refers to the same brick even
    // if it's dropped.
    bricks.sort_by(|a,b| b.2.end.cmp(&a.2.end));
    let mut bricks_below = MultiMap::new(); // index -> indices below that interact.

    let range_overlap = |a: &Range<i64>, b: &Range<i64>| {
        a.start < b.end && b.start < a.end
    };

    let mut free_fall = Vec::new();
    for i in 0..bricks.len() {
        let mut any_below = false;
        for j in i+1..bricks.len() {
            if range_overlap(&bricks[i].0, &bricks[j].0) && range_overlap(&bricks[i].1, &bricks[j].1) {
                assert!(!range_overlap(&bricks[i].2, &bricks[j].2));
                bricks_below.insert(i, j);
                any_below = true;
            }
        }
        if !any_below {
            free_fall.push(i);
        }
    }
    println!("bricks: {:?}", bricks);
    println!("bricks_below: {:?}", bricks_below);

    let mut supported_by = MultiMap::new();
    let mut supports = MultiMap::new();

    let mut resolved = HashSet::new();
    while resolved.len() < bricks.len() {
        for i in &free_fall {
            let mut bricks_supporting = Vec::new(); // which bricks are directly under i
            let max_z = {
                if let Some(below) = bricks_below.get_vec(&i) {
                    below.iter().fold(1, |m, b| {
                        let other_brick_top = bricks[*b].2.end;
                        if other_brick_top > m {
                            bricks_supporting.clear();
                            bricks_supporting.push(*b);
                            other_brick_top
                        } else if other_brick_top == m {
                            bricks_supporting.push(*b);
                            other_brick_top
                        } else {
                            m
                        }
                    })
                } else {
                    1
                }
            };
            bricks[*i].2 = max_z..(max_z + bricks[*i].2.end - bricks[*i].2.start);
            supported_by.insert_many(i.to_owned(), bricks_supporting.clone());
            for j in bricks_supporting {
                supports.insert(j.to_owned(), i.to_owned());
            }
            resolved.insert(*i);
        }
        free_fall.clear();

        for i in 0..bricks.len() {
            if resolved.contains(&i) {
                continue;
            }
            if bricks_below.get_vec(&i).unwrap().iter().all(|b| resolved.contains(b)) {
                free_fall.push(i);
            }
        }
    }
    println!("bricks: {:?}", bricks);
    println!("supports: {:?}", supports);
    println!("supported_by: {:?}", supported_by);

    let can_remove: Vec<usize> = (0..bricks.len()).filter(|i| {
        if let Some(supports) = supports.get_vec(i) {
            supports.iter().all(|i| {
                supported_by.get_vec(&i).unwrap().len() > 1
            })
        } else {
            true
        }
    }).collect();

    println!("{:?}", can_remove);
    format!("{}", can_remove.len())
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
        assert_eq!(result.trim(), "5");
    }
}
