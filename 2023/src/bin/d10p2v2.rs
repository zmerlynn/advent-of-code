#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use aoc::*;
use itertools::Itertools;
// use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::HashSet};
use std::cmp::max;

// use std::str;
// use std::collections::{HashMap, HashSet};

struct Grid<T> {
    grid: Vec<Vec<T>>
}

impl<T> Grid<T> {
    fn new() -> Grid<T> {
        Grid { grid: Vec::new() }
    }
    fn push(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        let row = self.grid.get(y)?;
        row.get(x)
    }
    fn scan<A, F>(&self, accum: &mut A, f: F) where F: Fn(&mut A, usize, usize, &T) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                f(accum, x, y, &self.grid[y][x]);
            }
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn find_next_point(grid: &Grid<Vec<(i32, i32)>>, point: (i32, i32), prev_point: (i32, i32)) -> Option<(i32, i32)> {
    if let Some(dirs) = grid.get(point.0, point.1) {
        let mut prev_point_found = false;
        let mut next_point: Option<(i32, i32)> = None;
        for dir in dirs {
            let maybe_next = (point.0 + dir.0, point.1 + dir.1);
            if maybe_next == prev_point {
                prev_point_found = true;
            } else {
                next_point = Some(maybe_next);
            }
        }
        if prev_point_found && next_point.is_some() {
            return next_point
        }
    }
    None
}

fn run(input: String) -> String {
    let mut grid: Grid<Vec<(i32, i32)>> = Grid::new();
    for l in input.lines() {
        grid.push(l.chars().map(|c| { match c {
            '.' => vec![],
            '|' => vec![(0,-1),(0,1)],
            '-' => vec![(-1, 0),(1,0)],
            'L' => vec![(0,-1),(1,0)],
            'J' => vec![(0,-1),(-1,0)],
            '7' => vec![(0,1),(-1,0)],
            'F' => vec![(0,1),(1,0)],
            'S' => vec![(0,-1),(0,1),(-1, 0),(1,0)],
            _ => panic!("invalid grid point")
        }}).collect_vec());
    }
    let mut starting_points: Vec<(i32, i32)> = Vec::new();
    grid.scan(&mut starting_points, |points, x, y, pt| {
        if pt.len() == 4 {
            points.push((x as i32,y as i32));
        }
    });
    assert_eq!(starting_points.len(), 1);
    let starting_point = starting_points[0];

    let mut cycle = Vec::new();
    'found_cycle: for dir in vec![(0,-1),(0,1),(-1, 0),(1,0)] {
        let mut prev_point = starting_point;
        let mut point = (starting_point.0+dir.0, starting_point.1+dir.1);
        cycle = vec![starting_point, point];
        while let Some(next_point) = find_next_point(&grid, point, prev_point) {
            if next_point == starting_point {
                break 'found_cycle;
            }
            cycle.push(next_point);
            (prev_point, point) = (point, next_point);
        }
    }

    let cycle_set: HashSet<(i32, i32)> = HashSet::from_iter(cycle.clone());
    let mut inside_set: HashSet<(i32, i32)> = HashSet::new();
    grid.scan(&mut inside_set, |inside_set, x, y, pt| {
        let mut outside = true;
        if cycle_set.contains(&(x as i32, y as i32)) {
            return;
        }

        for x_scan in 0..x {
            let pt = (x_scan as i32, y as i32);
            if cycle_set.contains(&pt) {
                let what = grid.get(pt.0, pt.1).unwrap();
                if what[0] == (0,-1) && what.len() != 4 {
                    outside = !outside;
                }
            }
        }
        if !outside {
            // println!("{}, {}", x, y);
            inside_set.insert((x as i32, y as i32));
        }
    });
    format!("{}", inside_set.len())
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test3_works() {
        let result = run(day_input("-test3.txt".to_string()));
        assert_eq!(result.trim(), "4");
    }
    #[test]
    fn test4_works() {
        let result = run(day_input("-test4.txt".to_string()));
        assert_eq!(result.trim(), "10");
    }
    #[test]
    fn test5_works() {
        let result = run(day_input("-test5.txt".to_string()));
        assert_eq!(result.trim(), "8");
    }
}
