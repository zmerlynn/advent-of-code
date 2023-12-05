use std::collections::HashMap;
use rangemap::RangeMap;

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn parse_ints(inp: &str) -> Vec<i64> {
    inp.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap()).collect()
}

fn run(input: String) -> String {
    let mut out: i64 = i64::MAX;
    let mut line_iter = input.lines();

    let seed_ranges: Vec<std::ops::Range<i64>> = parse_ints(line_iter.next().unwrap().split(":").collect_tuple::<(&str, &str)>().unwrap().1.trim()).into_iter().tuples().map(|(start, len)| start..start+len).collect();
    line_iter.next();
    println!("{:?}", seed_ranges);

    let mut thing_map = HashMap::new();
    while let Some(header) = line_iter.next() {
        let (map_from, map_to) = header.split(" ").collect_tuple::<(&str, &str)>().unwrap().0.split("-to-").collect_tuple::<(&str, &str)>().unwrap();
        // println!("map_from: {:?}, map_to: {:?}", map_from, map_to);

        let mut rm = RangeMap::new();
        while let Some(l) = line_iter.next() {
            if l == "" {
                break
            }
            let (dest_start, source_start, ext_len) = parse_ints(l).into_iter().collect_tuple().unwrap();
            rm.insert(source_start..(source_start+ext_len), dest_start);
            // println!("map_from: {:?}, map_to: {:?}: {:?} -> {:?} for {:?}", map_from, map_to, dest_start, source_start, ext_len);
        }

        thing_map.insert(map_from, (map_to, rm));
    }
    let start = std::time::SystemTime::now();
    for seed_range in seed_ranges {
        println!("elapsed: {:?}: starting range {:?} (len {:?})", std::time::SystemTime::now().duration_since(start).unwrap(), seed_range, seed_range.end-seed_range.start, );
        for seed in seed_range {
            let mut what = "seed";
            let mut val = seed;
            while let Some(new_thing) = thing_map.get(what) {
                // println!("what: {:?}, val: {:?}", what, val);
                let (new_what, m) = new_thing;
                if let Some((range, dest)) = m.get_key_value(&val) {
                    val = dest+(val-range.start);
                }
                what = new_what;
            }
            // println!("what: {:?}, val: {:?}", what, val);
            assert_eq!(what, "location");

            if val < out {
                out = val;
            }
        }
    }

    format!("{}", out)
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
