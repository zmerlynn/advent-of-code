use std::collections::{BTreeMap, HashMap};

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

struct ExtentMap {
    tree: BTreeMap<i64, (i64, i64)>
}

impl ExtentMap {
    fn new() -> ExtentMap {
        ExtentMap { tree: BTreeMap::new() }
    }
    fn insert(&mut self, from_val: i64, to_val: i64, ext_len: i64) {
        self.tree.insert(from_val, (to_val, ext_len));
    }
    fn find(&self, from_in: i64) -> Option<i64> {
        // https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.upper_bound would be better here but
        // I don't feel like switching to nightly.

        for (from_val, (to_val, ext_len)) in self.tree.iter() {
            if *from_val <= from_in && from_in < (*from_val + ext_len) {
                return Some(*to_val + (from_in-*from_val))
            }
            if *from_val > from_in {
                break
            }
        }
        None
    }
}

fn parse_ints(inp: &str) -> Vec<i64> {
    inp.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap()).collect()
}

fn run(input: String) -> String {
    let mut out: i64 = i64::MAX;
    let mut line_iter = input.lines();

    let seeds = parse_ints(line_iter.next().unwrap().split(":").collect_tuple::<(&str, &str)>().unwrap().1.trim());
    line_iter.next();
    println!("{:?}", seeds);

    let mut thing_map = HashMap::new();
    while let Some(header) = line_iter.next() {
        let (map_from, map_to) = header.split(" ").collect_tuple::<(&str, &str)>().unwrap().0.split("-to-").collect_tuple::<(&str, &str)>().unwrap();
        println!("map_from: {:?}, map_to: {:?}", map_from, map_to);

        let mut m = ExtentMap::new();
        while let Some(l) = line_iter.next() {
            if l == "" {
                break
            }
            let (dest_start, source_start, ext_len) = parse_ints(l).into_iter().collect_tuple().unwrap();
            m.insert(source_start, dest_start, ext_len);
            println!("map_from: {:?}, map_to: {:?}: {:?} -> {:?} for {:?}", map_from, map_to, dest_start, source_start, ext_len);
        }

        thing_map.insert(map_from, (map_to, m));
    }
    for seed in seeds {
        let mut what = "seed";
        let mut val = seed;
        while let Some(new_thing) = thing_map.get(what) {
            println!("what: {:?}, val: {:?}", what, val);
            let (new_what, m) = new_thing;
            val = m.find(val).unwrap_or(val);
            what = new_what;
        }
        println!("what: {:?}, val: {:?}", what, val);
        assert_eq!(what, "location");

        if val < out {
            out = val;
        }
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "35";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
