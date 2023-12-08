// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use aoc::*;
use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

fn run(input: String) -> String {
    let mut line_iter = input.lines();
    let dirs = line_iter.next().unwrap();
    line_iter.next();

    let mut tree = HashMap::new();
    let mut locs = Vec::new();
    for l in line_iter {
        let (source, dests) = l.split(" = ").collect_tuple().unwrap();
        let (left_with, right_with) = dests.split(", ").collect_tuple().unwrap();
        let left = left_with
            .split("(")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
            .1;
        let right = right_with
            .split(")")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
            .0;

        tree.insert(source, (left, right));
        if source.ends_with("A") {
            locs.push(source);
        }
    }

    let mut seen = vec![HashMap::new(); locs.len()];
    let mut cycles = vec![0; locs.len()];
    let mut steps = 0;
    loop {
        for dir in dirs.chars() {
            let mut new_locs = Vec::new();
            for loc in locs {
                let node = tree.get(loc).unwrap();

                new_locs.push(if dir == 'L' { node.0 } else { node.1 });
            }
            locs = new_locs;
            steps += 1;
        }
        for i in 0..locs.len() {
            if cycles[i] > 0 {
                continue;
            }
            if let Some(seen_before) = seen[i].get(locs[i]) {
                cycles[i] = steps - seen_before;
            } else {
                seen[i].insert(locs[i], steps);
            }
        }
        if steps % 1000 == 0 {
            println!(
                "cycles: {:?} locs: {:?}, start_locs: {:?}",
                cycles, locs, seen
            );
        }
        if cycles.iter().all(|len| *len > 0) {
            break;
        }
    }
    println!("{:?}", cycles);
    let mut i: i128 = 1;
    for cycle in cycles {
        i = lcm(i, cycle as i128);
    }
    format!("{}", i)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "6";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(day_input("-test2.txt".to_string()));
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
