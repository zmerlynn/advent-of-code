#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
use tuple::*;
use std::fmt::{Display, Formatter, Result};

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

fn find_next_point(grid: &Grid<Vec<T2<i32, i32>>>, point: &T2<i32, i32>, prev_point: &T2<i32, i32>) -> Option<T2<i32, i32>> {
    if let Some(dirs) = grid.get(point.0, point.1) {
        let mut prev_point_found = false;
        let mut next_point: Option<T2<i32, i32>> = None;
        for dir in dirs {
            let maybe_next = *point + *dir;
            if maybe_next == *prev_point {
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
    let mut grid: Grid<Vec<T2<i32, i32>>> = Grid::new();
    for l in input.lines() {
        grid.push(l.chars().map(|c| { match c {
            '.' => vec![],
            '|' => vec![T2(0,-1),T2(0,1)],
            '-' => vec![T2(-1, 0),T2(1,0)],
            'L' => vec![T2(0,-1),T2(1,0)],
            'J' => vec![T2(0,-1),T2(-1,0)],
            '7' => vec![T2(0,1),T2(-1,0)],
            'F' => vec![T2(0,1),T2(1,0)],
            'S' => vec![T2(0,-1),T2(0,1),T2(-1, 0),T2(1,0)],
            _ => panic!("invalid grid point")
        }}).collect_vec());
    }
    let mut starting_points: Vec<T2<i32, i32>> = Vec::new();
    grid.scan(&mut starting_points, |points, x, y, pt| {
        if pt.len() == 4 {
            points.push(T2(x as i32,y as i32));
        }
    });
    assert_eq!(starting_points.len(), 1);
    let starting_point = starting_points[0];

    let mut cycle = Vec::new();
    'found_cycle: for dir in vec![T2(0,-1),T2(0,1),T2(-1, 0),T2(1,0)] {
        let mut prev_point = starting_point;
        let mut point = starting_point+dir;
        cycle = vec![starting_point, point];
        while let Some(next_point) = find_next_point(&grid, &point, &prev_point) {
            if next_point == starting_point {
                break 'found_cycle;
            }
            cycle.push(next_point);
            (prev_point, point) = (point, next_point);
        }
    }

    format!("{}", cycle.len() / 2)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "8";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
