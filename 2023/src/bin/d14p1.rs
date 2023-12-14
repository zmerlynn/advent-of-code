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

impl<T: Copy> Grid<T> {
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let val = self.grid[y1][x1];
        self.grid[y1][x1] = self.grid[y2][x2];
        self.grid[y2][x2] = val;
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

fn run(input: String) -> String {
    let mut grid: Grid<char> = Grid::new();
    for l in input.lines() {
        grid.push(l.chars().collect_vec());        
    }
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[y].len() {
            if grid.grid[y][x] != '.' {
                continue;
            }
            for y2 in y+1..grid.grid.len() {
                match grid.grid[y2][x] {
                    '.' => continue,
                    '#' => break,
                    'O' => {
                        grid.swap(x,y, x, y2);
                        // println!("{}, {} <-> {}, {}", x, y, x, y2);
                        break;
                    },
                    _ => panic!("invalid grid point")
                }
            }
        }
    }
    // println!("{}", grid);
    let mut out = 0;
    let height = grid.grid.len(); 
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[y].len() {
            if grid.grid[y][x] == 'O' {
                out += height - y;
            }
        }
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "136";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
