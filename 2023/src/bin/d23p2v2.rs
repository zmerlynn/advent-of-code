#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use maplit::{hashset, hashmap};
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
    let width = input.find("\n").unwrap();
    let mut grid = Array2::<bool>::default((0, width));
    for l in input.lines() {
        let mapped = l.as_bytes().iter().map(|c| match c {
            b'#' => false,
            _ => true,
        }).collect_vec();
        grid.push_row(ArrayView::from(&mapped)).unwrap();
    }
    let shape = grid.shape();
    assert_eq!(shape[0], shape[1]);
    let size = shape[0];
    
    let start_point = (0 as usize, 1 as usize);
    let end_point = (size-1, size-2);
    let mut edges = MultiMap::new(); // V1 -> [(V2, dist), (V3, dist), ...]
    let mut traversed = hashset! {start_point};
    let mut vertices_to_explore = hashset! {start_point};

    let liberties = |pt: &(usize, usize)| {
        let mut out = Vec::new();
        for dir in [(-1,0), (0,1), (1,0), (0,-1)] {
            if dir.0 < 0 && pt.0 == 0 {
                continue
            }
            let adj = (pt.0.checked_add_signed(dir.0).unwrap(), pt.1.checked_add_signed(dir.1).unwrap());
            if grid[adj] {
                out.push(adj);
            }
        }
        out
    };

    // walk from prev_pt -> pt to the next vertex, return vertex found, mark as traversed along the way.
    let mut walk_hall = |prev_pt: &(usize, usize), pt: &(usize, usize)| {
        if traversed.contains(pt) {
            return None; // we've already visited this edge
        }

        let mut dist = 1; // we assume we just walked off a vertex, so start at one
        let mut prev_pt = prev_pt.to_owned();
        let mut pt = pt.to_owned();
        loop {
            traversed.insert(pt.to_owned());

            if pt == end_point {
                return Some((pt.to_owned(), dist));
            }

            let adj_pts = liberties(&pt);
            let forward = adj_pts
                .iter()
                .filter(|adj_pt| *adj_pt != &prev_pt)
                .collect::<Vec<&(usize, usize)>>();

            assert!(forward.len() > 0, "dead end detected");
            if forward.len() > 1 {
                return Some((pt.to_owned(), dist));
            }

            prev_pt = pt;
            pt = *forward[0];
            dist += 1;
        }
    };

    while vertices_to_explore.len() > 0 {
        let mut new_vertices_to_explore = HashSet::new();
        for v in &vertices_to_explore {
            for adj_v in liberties(v) {
                if let Some((next_vertex, dist)) = walk_hall(v, &adj_v) {
                    edges.insert(next_vertex, (*v, dist));
                    edges.insert(*v, (next_vertex, dist));

                    if next_vertex != end_point {
                        new_vertices_to_explore.insert(next_vertex);
                    }
                }
            }
        }
        vertices_to_explore = new_vertices_to_explore;
    }
    // println!("edges = {:?}", edges);
    let edges = edges;

    let mut paths = vec!((start_point.to_owned(), 0 as usize, hashset! {start_point.to_owned()}));
    let mut max_path = 0 as usize;
    while paths.len() > 0 {
        let mut new_paths = Vec::new();

        for (v, path_dist, path) in &paths {
            if v == &end_point {
                max_path = max(max_path, *path_dist);
                continue;
            }
            for (next, dist) in edges.get_vec(v).unwrap() {
                if path.contains(next) {
                    continue;
                }
                let path_set = {
                    let mut out = path.clone();
                    out.insert(next.to_owned());
                    out
                };
                new_paths.push((next.to_owned(), path_dist + dist, path_set));
            }
        }

        println!("{} paths, {} new_paths", paths.len(), new_paths.len());
        paths = new_paths;
    }

    format!("{}", max_path)
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
        assert_eq!(result.trim(), "154");
    }
}
