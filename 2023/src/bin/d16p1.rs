#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashMap};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr};
use multimap::MultiMap;

// use std::str;
// use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(usize, usize);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Direction(isize, isize);

const D_RIGHT: Direction = Direction(0, 1);
const D_LEFT: Direction = Direction(0, -1);
const D_UP: Direction = Direction(-1, 0);
const D_DOWN: Direction = Direction(1, 0);

impl Point {
    fn towards(self, dir: Direction, shape: Point) -> Option<Point> {
        let (new_row, new_col) = (self.0 as isize + dir.0, self.1 as isize + dir.1);
        if new_row < 0 || new_col < 0 || new_row >= shape.0 as isize || new_col >= shape.1 as isize {
            return None;
        }
        Some(Point(new_row.try_into().unwrap(), new_col.try_into().unwrap()))
    }
}

fn run(input: String) -> String {
    let width = input.find("\n").unwrap();
    let mut grid = Array2::zeros((0, width));
    for l in input.lines() {
        grid.push_row(ArrayView::from(l.as_bytes())).unwrap();
    }
    let shape = Point(grid.shape()[0], grid.shape()[1]);

    let mut visited = MultiMap::new();
    let mut new_beam_edge = vec!((Point(0,0),D_RIGHT));

    while new_beam_edge.len() > 0 {
        let mut beam_edge: Vec<(Point,Direction)> = new_beam_edge;
        new_beam_edge = Vec::new();

        // println!("beam_edge = {:?}", beam_edge);

        for (pt, dir) in beam_edge {
            let e = visited.entry(pt).or_insert_vec(Vec::new());
            if e.contains(&dir) {
                continue;
            }
            e.push(dir);

            let mut add_to_new_edge = |maybe_pt: Option<Point>, dir: Direction| {
                if let Some(new_pt) = pt.towards(dir, shape) {
                    new_beam_edge.push((new_pt, dir));
                }
            };

            let grid_at = grid[(pt.0, pt.1)];
            match grid_at {
                b'.' => add_to_new_edge(pt.towards(dir, shape), dir),
                b'/' => {
                    let new_dir = match dir {
                        D_DOWN => D_LEFT,
                        D_LEFT => D_DOWN,
                        D_RIGHT => D_UP,
                        D_UP => D_RIGHT,
                        _ => panic!("invalid")
                    };
                    add_to_new_edge(pt.towards(new_dir, shape), new_dir)
                },
                b'\\' => {
                    let new_dir = match dir {
                        D_DOWN => D_RIGHT,
                        D_LEFT => D_UP,
                        D_RIGHT => D_DOWN,
                        D_UP => D_LEFT,
                        _ => panic!("invalid")
                    };
                    add_to_new_edge(pt.towards(new_dir, shape), new_dir)
                },
                b'-' => {
                    if dir == D_DOWN || dir == D_UP {
                        add_to_new_edge(pt.towards(D_LEFT, shape), D_LEFT);
                        add_to_new_edge(pt.towards(D_RIGHT, shape), D_RIGHT);
                    } else {
                        add_to_new_edge(pt.towards(dir, shape), dir)
                    }
                }
                b'|' => {
                    if dir == D_RIGHT || dir == D_LEFT {
                        add_to_new_edge(pt.towards(D_UP, shape), D_UP);
                        add_to_new_edge(pt.towards(D_DOWN, shape), D_DOWN);
                    } else {
                        add_to_new_edge(pt.towards(dir, shape), dir)
                    }
                }
                _ => panic!("invalid")
            }
        }
    }
    format!("{}", visited.len())
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "46";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
