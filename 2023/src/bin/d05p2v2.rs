use core::ops::Range;
use rangemap::RangeMap;
use std::cmp::{min, max};

use aoc::*;
use itertools::Itertools;
// use std::str;
// use std::collections::{HashMap, HashSet};

fn parse_ints(inp: &str) -> Vec<i64> {
    inp.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap()).collect()
}

fn intersect(r1: &Range<i64>, r2: &Range<i64>) -> Range<i64> {
    max(r1.start, r2.start)..min(r1.end, r2.end)
}

fn compose_rangemaps(f: &RangeMap<i64, i64>, g: &RangeMap<i64, i64>) -> RangeMap<i64, i64> {
    let mut out = RangeMap::new();

    for (f_range, g_start) in f.iter() {
        let g_outer_range = *g_start..*g_start+(f_range.end-f_range.start);
        // println!("outer_range: {:?}", g_outer_range);
        for (g_range, h_start) in g.overlapping(&g_outer_range) {
            let g_in_outer = intersect(g_range, &g_outer_range);
            // println!("g_in_outer: {:?}, h_start: {:?}", g_in_outer, h_start);
            let f_overlap = f_range.start+g_in_outer.start-g_outer_range.start..f_range.start+g_in_outer.end-g_outer_range.start;
            let h_in_outer = h_start+g_in_outer.start-g_range.start;
            out.insert(f_overlap, h_in_outer);
        }
        for g_gap in g.gaps(&g_outer_range) {
            let f_gap = f_range.start+g_gap.start-g_outer_range.start..f_range.start+g_gap.end-g_outer_range.start;
            out.insert(f_gap.clone(), g_gap.start);
        }
    }

    out
}

fn run(input: String) -> String {
    let mut out: i64 = i64::MAX;
    let mut line_iter = input.lines();

    let seed_ranges: Vec<std::ops::Range<i64>> = parse_ints(line_iter.next().unwrap().split(":").collect_tuple::<(&str, &str)>().unwrap().1.trim()).into_iter().tuples().map(|(start, len)| start..start+len).collect();
    line_iter.next();

    let mut aggregate_map = RangeMap::new();
    for seed_range in &seed_ranges {
        aggregate_map.insert(seed_range.clone(), seed_range.start);
    }
    // println!("{:?}", aggregate_map);
    while let Some(_) = line_iter.next() {
        let mut rm = RangeMap::new();
        while let Some(l) = line_iter.next() {
            if l == "" {
                break
            }
            let (dest_start, source_start, ext_len) = parse_ints(l).into_iter().collect_tuple().unwrap();
            rm.insert(source_start..(source_start+ext_len), dest_start);
        }
        aggregate_map = compose_rangemaps(&aggregate_map, &rm);
        // println!("o {:?} -> {:?}\n", rm, aggregate_map);
    }
    for (_, val) in aggregate_map.iter() {
        out = min(out, *val);
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
