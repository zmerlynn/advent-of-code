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
    let mut outside_set: HashSet<(i32, i32)> = HashSet::new();
    let max_y = grid.grid.len() as i32;
    let max_x = grid.grid[0].len() as i32;
    loop {
        let known_points_outside = outside_set.len();
        grid.scan(&mut outside_set, |outside_set, x, y, pt| {
            let pt = (x as i32, y as i32);
            if outside_set.contains(&pt) || cycle_set.contains(&pt) {
                return
            }
            for dir in vec![(0,-1),(0,1),(-1, 0),(1,0)] {
                let adj_pt = (pt.0+dir.0, pt.1+dir.1);
                if outside_set.contains(&adj_pt) || grid.get(adj_pt.0, adj_pt.1).is_none() {
                    outside_set.insert(pt);
                    return;
                }
            }
        });
        if outside_set.len() == known_points_outside {
            break;
        }
    }

    // The problem now is that we might still have islands of "outside". We're going to use the "normal"
    // of the cycle to find the interior instead, then creep the interior in the same way as above.
    // To find the normal we start by using the above inference of outside to prove whether the cycle
    // is currently "left-hand rule" or "right-hand rule" oriented (i.e. is cycle in clockwise
    // or counter-clockwise order).

    // normals maps "direction of travel" to "direction of outside" when cycle is clockwise, i.e.
    // (1, 0) => (0, -1)
    // can be read as "when travelling right, the outside is up"
    let normals = hashmap! {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
    };
    let mut cycle_direction = 0; // 1 == clockwise, -1 == counter
    for i in 1..cycle.len() {
        let dir = (cycle[i].0 - cycle[i-1].0, cycle[i].1 - cycle[i-1].1);
        let normal = normals.get(&dir).unwrap().clone();
        if outside_set.contains(&(cycle[i].0+normal.0, cycle[i].1+normal.1)) {
            assert!(cycle_direction == 1 || cycle_direction == 0);
            cycle_direction = 1;
        } else if outside_set.contains(&(cycle[i].0-normal.0, cycle[i].1-normal.1)) {
            assert!(cycle_direction == -1 || cycle_direction == 0);
            cycle_direction = -1;
        }
    }
    if cycle_direction == -1 {
        cycle.reverse();
    }
    // println!("cycle = {:?}, cycle_direction = {:?}", cycle, cycle_direction);

    let mut inside_set = HashSet::new();
    for i in 1..cycle.len() {
        let dir = (cycle[i].0 - cycle[i-1].0, cycle[i].1 - cycle[i-1].1);
        let normal = normals.get(&dir).unwrap().clone();
        let test_pt = (cycle[i].0-normal.0, cycle[i].1-normal.1); // anti-normal
        assert!(!outside_set.contains(&test_pt));
        if cycle_set.contains(&test_pt) {
            continue;
        }
        inside_set.insert(test_pt);
        let trailing_test_pt = (cycle[i-1].0-normal.0, cycle[i-1].1-normal.1); // anti-normal for previous point
        assert!(!outside_set.contains(&trailing_test_pt));
        if cycle_set.contains(&trailing_test_pt) {
            continue;
        }
        inside_set.insert(trailing_test_pt);
    }

    loop {
        let mut new_inside_set = inside_set.clone();

        for pt in &inside_set {
            assert!(!outside_set.contains(&pt));
            assert!(!cycle_set.contains(&pt));

            for dir in vec![(0,-1),(0,1),(-1, 0),(1,0),(-1,-1),(1,1),(-1, 1),(1,-1)] {
                let adj_pt = (pt.0+dir.0, pt.1+dir.1);
                assert!(!outside_set.contains(&adj_pt));
                if !cycle_set.contains(&adj_pt) {
                    new_inside_set.insert(adj_pt);
                    continue;
                }
            }
        }
        println!("inside = {} -> new_inside = {}", inside_set.len(), new_inside_set.len());
        if new_inside_set.len() == inside_set.len() {
            break;
        }
        inside_set = new_inside_set;
    }

    for y in 0..max_y {
        for x in 0..max_x {
            let pt = (x,y);
            if inside_set.contains(&pt) {
                print!("i");
            } else if outside_set.contains(&pt) {
                print!("o");
            } else if cycle_set.contains(&pt) {
                print!("C");
            } else {
                print!("?");
            }
        }
        println!("")
    }

    // let mut sorted_inside = inside_set.iter().collect_vec();
    // sorted_inside.sort();
    // println!("{:?}", sorted_inside);
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
