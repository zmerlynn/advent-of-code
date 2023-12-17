#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashMap, cmp::{min,max}};
use ndarray::{Array, Array2, ArrayView, array, Axis, OwnedRepr};
use multimap::MultiMap;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Direction(isize, isize);

const D_RIGHT: Direction = Direction(0, 1);
const D_LEFT: Direction = Direction(0, -1);
const D_UP: Direction = Direction(-1, 0);
const D_DOWN: Direction = Direction(1, 0);

const DIRECTIONS: [Direction; 4] = [D_RIGHT, D_DOWN, D_LEFT, D_UP];

impl Direction {
    fn opposite(self) -> Direction {
        return Direction(-self.0, -self.1);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn towards(self, dir: Direction, shape: Point) -> Option<Point> {
        let (new_row, new_col) = (self.0 as isize + dir.0, self.1 as isize + dir.1);
        if new_row < 0 || new_col < 0 || new_row >= shape.0 as isize || new_col >= shape.1 as isize {
            return None;
        }
        Some(Point(new_row.try_into().unwrap(), new_col.try_into().unwrap()))
    }
}

#[derive(Debug)]
struct PathTracker {
    loc: Point,
    cost: usize,
    via: Direction,
    dist: usize,
}

fn run(input: String) -> String {
    let width = input.find("\n").unwrap();
    let mut grid = Array2::zeros((0, width));
    for l in input.lines() {
        let row: Vec<u8> = l.as_bytes().iter().map(|c| c - b'0').collect();
        grid.push_row(ArrayView::from(&row)).unwrap();
    }
    let shape = Point(grid.shape()[0], grid.shape()[1]);

    let mut new_edge = vec!(
        PathTracker{
            loc: Point(0,0),
            cost: 0,
            via: D_RIGHT, // arbitrary since dist == 0
            dist: 0,
        });
    let mut visited = HashMap::new();

    while new_edge.len() > 0 {
        println!("new_edge.len() = {:?}, visited.len() = {:?}", new_edge.len(), visited.len());

        let mut edge: Vec<PathTracker> = new_edge;
        new_edge = Vec::new();

        for pt in edge {
            assert!(pt.dist < 4);
            let visited_entry = visited.entry(pt.loc).or_insert(
                HashMap::new() as HashMap<(Direction,usize), usize>);
            let path_entry = visited_entry.entry((pt.via, pt.dist)).or_insert(usize::MAX);
            if *path_entry <= pt.cost {
                continue; // we've already visited this with an equivalent path at lower cost
            }
            *path_entry = pt.cost;

            for dir in DIRECTIONS {
                if dir == pt.via.opposite() || (dir == pt.via && pt.dist >= 3) {
                    continue; // no back-tracking, no more than 3 in same dir
                }

                if let Some(new_loc) = pt.loc.towards(dir, shape) {
                    new_edge.push(PathTracker{
                        loc: new_loc, 
                        cost: pt.cost + grid[(new_loc.0, new_loc.1)] as usize,
                        via: dir,
                        dist: if dir == pt.via { pt.dist + 1 } else { 1 },
                    })
                }
            }
        }
    }
    let costs = visited.get(&Point(shape.0-1, shape.1-1)).unwrap();
    println!("costs = {:?}", costs);
    let min_cost = costs.iter().fold(usize::MAX, |m, (_, c)| min(m, *c));
    format!("{}", min_cost)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "102";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
