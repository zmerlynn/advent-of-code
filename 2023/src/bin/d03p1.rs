use aoc::*;
use std::collections::HashSet;
// use itertools::Itertools;
// use std::cmp;
use std::fmt::{Display, Formatter, Result};

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

#[derive(Debug)]
enum Piece {
    Numeric(i32),
    Symbol,
    Empty,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Numeric(i) => { write!(f, "{}", i)?; },
            Self::Symbol => { write!(f, "*")?; },
            Self::Empty => { write!(f, ".")?; }
        }
        Ok(())
    }
}

fn run(input: String) -> String {
    let mut t = 0;
    let mut grid = Grid::new();
    // let mut part_map = HashMap::new();
    for l in input.lines() {
        grid.push(l.chars().map(|c| {
            if c == '.' {
                Piece::Empty
            } else if c.is_numeric() {
                Piece::Numeric(c.to_digit(10).unwrap() as i32)
            } else {
                Piece::Symbol
            }
        }).collect());
    }
    let mut part_map: HashSet<(i32, i32)> = HashSet::new();
    grid.scan(&mut part_map, |accum, x, y, what| {
        if let Piece::Symbol = what {
            println!("x: {}, y: {}", x, y);
            for delta_x in -1..2 {
                for delta_y in -1..2 {
                    println!("  x: {}, y: {}", (x as i32)+delta_x, (y as i32)+delta_y);
                    if let Piece::Numeric(_) = grid.get((x as i32)+delta_x, (y as i32)+delta_y).unwrap_or(&Piece::Empty) {
                        let mut left_x = -1;
                        while let Piece::Numeric(_) = grid.get((x as i32)+delta_x+left_x, (y as i32)+delta_y).unwrap_or(&Piece::Empty) {
                            left_x -= 1;
                        }
                        accum.insert(((x as i32)+delta_x+left_x+1, (y as i32)+delta_y));
                    }
                }
            }
        }
    });
    for (x, y) in part_map {
        let mut num = 0;
        let mut delta_x = 0;
        while let Piece::Numeric(d) = grid.get((x as i32)+delta_x, y as i32).unwrap_or(&Piece::Empty) {
            num = num * 10 + d;
            delta_x += 1;
        }
        println! {"({}, {}): {}", x, y, num};
        t += num;
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "4361";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
