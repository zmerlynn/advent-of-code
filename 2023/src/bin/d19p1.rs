#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use std::iter::Zip;
use tuple::*;
use std::{fmt::{Display, Formatter, Result}, collections::{HashMap, HashSet}, cmp::{min,max}, str};
use ndarray::*;
use multimap::MultiMap;
use regex::Regex;

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    RunWorkflow(String),
}

#[derive(Debug)]
struct Condition {
    value_idx: usize, // x == 0, m == 1, a == 2, s == 3
    value: i32,
    cmp: i32, // 1 => thing[value_idx] > value, -1 => < value
}

#[derive(Debug)]
struct Instruction {
    condition: Option<Condition>,
    action: Action,
}

fn run(input: String) -> String {
    let mut lines_iter = input.lines();
    let workflow_re = Regex::new(r"(.*)\{([^}]+)\}").unwrap(); // matches e.g. px{a<2006:qkq,m>2090:A,rfg}
    let cond_re = Regex::new(r"([xmas])([><])(\d+)").unwrap();

    let mut workflows = HashMap::new();
    while let Some(l) = lines_iter.next() {
        if l == "" {
            break;
        }
        let workflow_capture = workflow_re.captures(l).unwrap();

        let mut instructions = Vec::new();
        let name = &workflow_capture[1];
        for inst_str in workflow_capture[2].split(",") {
            let pieces = inst_str.split(":").collect_vec();
            let condition = {
                if pieces.len() == 2 {
                    let cond_capture = cond_re.captures(pieces[0]).unwrap();
                    let value_idx = match &cond_capture[1] {
                        "x" => 0 as usize,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!("invalid {}", &cond_capture[1])
                    };
                    let cmp = match &cond_capture[2] {
                        ">" => 1,
                        "<" => -1,
                        _ => panic!("fsfsdfsf")
                    };
                    let value = i32::from_str_radix(&cond_capture[3], 10).unwrap();
                    Some(Condition{value_idx, cmp, value})
                } else {
                    None
                }
            };
            let action =match pieces[pieces.len()-1] {
                "A" => Action::Accept,
                "R" => Action::Reject,
                _ => Action::RunWorkflow(pieces[pieces.len()-1].to_string()),
            };
            instructions.push(Instruction{condition, action});
        }
        workflows.insert(name.to_owned(), instructions);
    }
    // println!("workflows: {:?}", workflows);

    let accepted = |values: [i32; 4]| {
        let mut workflow = "in";
        loop {
            for inst in &workflows[workflow] {
                if let Some(cond) = &inst.condition {
                    if (values[cond.value_idx] - cond.value).signum() != cond.cmp {
                        continue
                    }
                }
                match &inst.action {
                    Action::Accept => return true,
                    Action::Reject => return false,
                    Action::RunWorkflow(new_workflow) => {
                        workflow = new_workflow;
                        break;
                    }
                }
            }
        }
    };

    let input_re = Regex::new(r"\{([^}]+)\}").unwrap();
    let value_re = Regex::new(r"([xmas])=(\d+)").unwrap();
    let mut out = 0;
    for l in lines_iter {
        let input_capture = input_re.captures(l).unwrap();
        let mut values = [-1, -1, -1, -1];
        let want = ["x", "m", "a", "s"];
        for (i, value_str) in input_capture[1].split(",").enumerate() {
            let value_capture = value_re.captures(value_str).unwrap();
            assert_eq!(&value_capture[1], want[i]);
            values[i] = i32::from_str_radix(&value_capture[2], 10).unwrap();
        }

        if accepted(values) {
            out += values[0] + values[1] + values[2] + values[3];
        }
    }
    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "19114";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
