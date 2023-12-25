#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use maplit::{hashmap, hashset};
use multimap::MultiMap;
use ndarray::*;
use ndarray_linalg::*;
use num::{complex::ComplexFloat, iter::RangeInclusive};
use rand::prelude::*;
use rand::*;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    ops::{Range, Sub},
    str,
};
use std::{collections::VecDeque, iter::Zip};
use tuple::*;

fn run(input: String) -> String {
    let mut edges = MultiMap::new();
    for l in input.lines() {
        let (from_str, all_to_str) = l.split(": ").collect_tuple().unwrap();
        for to_str in all_to_str.split(" ") {
            let to_str = to_str.to_string() + "+";
            let from_str = from_str.to_string() + "+";
            edges.insert(from_str.to_owned(), to_str.to_owned());
            edges.insert(to_str.to_owned(), from_str.to_owned());
        }
    }

    let vertices: HashMap<String, usize> =
        HashMap::from_iter(edges.keys().enumerate().map(|(i, v)| (v.to_owned(), i)));

    let mut adj_matrix = Array2::<usize>::zeros((vertices.len(), vertices.len()));
    for (v, v_adj) in edges.iter_all() {
        let v_idx = vertices.get(v).unwrap();
        for adj in v_adj {
            let adj_idx = vertices.get(adj).unwrap();
            adj_matrix[(*v_idx, *adj_idx)] = 1;
        }
    }
    println!("{:?}\n{}\n", vertices, adj_matrix);

    let mut rng = thread_rng();
    loop {
        // Impl of https://en.wikipedia.org/wiki/Karger%27s_algorithm
        let mut size = adj_matrix.shape()[0];
        let mut vertex_count = Array1::<usize>::ones(size); // number of contracted vertices at each index
        let mut resid: HashSet<usize> = HashSet::from_iter(0..size); // number of vertices left in graph
        let mut adj_matrix = adj_matrix.clone();

        while resid.len() > 2 {
            let resid_vec = resid.iter().map(|i| *i).collect_vec();
            let v_0 = resid_vec
                .choose_weighted(&mut rng, |i| adj_matrix.row(*i).sum() as f64)
                .unwrap()
                .to_owned();
            let v_1 = adj_matrix
                .row(v_0)
                .to_slice()
                .unwrap()
                .iter()
                .enumerate()
                .collect_vec()
                .choose_weighted(&mut rng, |(i, edge)| **edge)
                .unwrap()
                .0;
            let (v_0, v_1) = {
                if v_0 < v_1 {
                    (v_0, v_1)
                } else {
                    (v_1, v_0)
                }
            };

            // contract from row, col v_1 into v_0
            let mut new_row = &adj_matrix.row(v_0) + &adj_matrix.row(v_1);
            new_row[v_0] = 0;
            new_row[v_1] = 0;

            for i in 0..size {
                adj_matrix[(i, v_0)] = new_row[i];
                adj_matrix[(v_0, i)] = new_row[i];
                adj_matrix[(i, v_1)] = 0;
                adj_matrix[(v_1, i)] = 0;
            }
            vertex_count[v_0] += vertex_count[v_1];
            vertex_count[v_1] = 0;
            resid.remove(&v_1);

            while vertex_count[size - 1] == 0 {
                size -= 1;
            }

            if cfg!(test) {
                assert_eq!(resid.len(), vertex_count.iter().filter(|i| **i > 0).count());
                for i in 0..size {
                    let row = adj_matrix.row(i);
                    assert_eq!(adj_matrix.column(i), row);

                    for j in 0..size {
                        assert!((row[j] == 0) || resid.contains(&j));
                    }
                }
            }

            // println!("{:?}\n{}\nresid: {:?}\n", v, adj_matrix, resid);
        }

        let resid = resid.iter().collect::<Vec<&usize>>();
        let cut_size = adj_matrix[(*resid[0], *resid[1])];
        println!(
            "Found cut: {}, vertex_counts=[{}, {}]",
            cut_size, vertex_count[*resid[0]], vertex_count[*resid[1]]
        );
        if cut_size == 3 {
            return format!("{}", vertex_count[*resid[0]] * vertex_count[*resid[1]]);
        }
    }
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
        assert_eq!(result.trim(), "54");
    }

    #[test]
    fn test_input2_works() {
        let result = run(day_input("-test2.txt".to_string()));
        assert_eq!(result.trim(), "3");
    }
}
