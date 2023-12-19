#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use std::iter::Zip;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::{HashMap, HashSet}, cmp::{min,max}};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr, arr1, Array1};
use multimap::MultiMap;

fn run(input: String) -> String {
    let d_right = arr1(&[0, 1]);
    let d_left = arr1(&[0, -1]);
    let d_up = arr1(&[-1, 0]);
    let d_down = arr1(&[1,0]);
    let directions = [&d_right, &d_left, &d_up, &d_down];
    
    let mut pos = arr1(&[0,0]);
    let mut min_bounds = arr1(&[0,0]);
    let mut max_bounds = arr1(&[0,0]);
    let mut pos_set = HashSet::new();
    for l in input.lines() {
        let (dir, len_str, _) = l.split(" ").collect_tuple().unwrap();
        let len = len_str.parse::<isize>().unwrap();
        let d = match dir {
            "R" => &d_right,
            "D" => &d_down,
            "L" => &d_left,
            "U" => &d_up,
            _ => panic!("ahhh!"),
        };
        for i in 0..len {
            pos = &pos + d;
            pos_set.insert(pos.clone());
            min_bounds.zip_mut_with(&pos, |a,b| *a = min(*a,*b));
            max_bounds.zip_mut_with(&pos, |a,b| *a = max(*a,*b));
        }
        println!("pos: {}, min: {}, max: {}", pos, min_bounds, max_bounds);
    }
    max_bounds = max_bounds + &d_down * 2 + &d_right * 2;
    min_bounds = min_bounds + &d_up + &d_left;
    println!("adjusted bounds min {}, max {}", max_bounds, min_bounds);

    let mut outside = HashSet::new();
    for row in min_bounds[0]..max_bounds[0] {
        outside.insert(arr1(&[row, min_bounds[1]]));
        outside.insert(arr1(&[row, max_bounds[1]-1]));
    }
    for col in min_bounds[1]..max_bounds[1] {
        outside.insert(arr1(&[min_bounds[0], col]));
        outside.insert(arr1(&[max_bounds[0]-1, col]));
    }
    let print_grid = |outside: &HashSet<Array1<i32>>| {
        for row in min_bounds[0]..max_bounds[0] {
            for col in min_bounds[1]..max_bounds[1] {
                if outside.contains(&arr1(&[row, col])) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
        println!("len = {}", outside.len());
    };
    // print_grid(&outside);
    loop {
        let mut new_outside = outside.clone();

        for outside_point in &outside {
            for dir in &directions {
                let away = outside_point + *dir;
                if away[0] < min_bounds[0] || away[0] >= max_bounds[0] || away[1] < min_bounds[1] || away[1] >= max_bounds[1] {
                    continue;
                }
                if !pos_set.contains(&away) {
                    if new_outside.insert(away.clone()) {
                        // println!("{}", away);
                    }
                }
            }
        }

        println!("{}, {}", new_outside.len(), outside.len());
        if new_outside.len() == outside.len() {
            break
        }
        outside = new_outside;
        // print_grid(&outside);
    }

    format!("{}", (max_bounds[0] - min_bounds[0]) * (max_bounds[1] - min_bounds[1]) - (outside.len() as i32))
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "62";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
