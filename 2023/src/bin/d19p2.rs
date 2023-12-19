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
    axis: usize, // x == 0, m == 1, a == 2, s == 3
    value: i32,
    cmp: i32, // 1 => thing[axis] > value, -1 => < value
}

#[derive(Debug)]
struct Instruction {
    condition: Option<Condition>,
    action: Action,
}

fn space_size(bounds: [(i32, i32); 4]) -> i128 {
    bounds.iter().fold(1, |accum, b| 
        accum * (b.1 as i128 - b.0 as i128))
}

// split_bounds_from_condition takes a 4-space of bounds and a condition
// and returns two optional 4-spaces: the true space and the false space,
// based on the partitioning by condition.
fn split_bounds_from_condition(bounds: [(i32, i32); 4], condition: &Condition) -> (Option<[(i32, i32); 4]>, Option<[(i32, i32); 4]>) {
    let split_axis = condition.axis;
    let split_bounds = bounds[split_axis];

    // splitting [x,y) => [x, split_point), [split_point, y)
    // but if condition is e.g. ">5", split_point needs to be 6.
    let split_point = condition.value + if condition.cmp > 0 { 1 } else { 0 };
    let (lower_axis_bounds, upper_axis_bounds) = {
        if split_bounds.0 < split_point && split_point < split_bounds.1 {
            (Some((split_bounds.0, split_point)), Some((split_point, split_bounds.1)))
        } else if split_point <= split_bounds.0 {
            (None, Some(split_bounds))
        } else {
            assert!(split_point >= split_bounds.1);
            (Some(split_bounds), None)
        }
    };

    let bounds_adjusted = |axis_bounds: Option<(i32, i32)>| -> Option<[(i32, i32); 4]> {
        if let Some(axis_bounds) = axis_bounds {
            let mut bounds = bounds.clone();
            bounds[split_axis] = axis_bounds;
            Some(bounds)
        } else {
            None
        }
    };

    if condition.cmp > 0 {
        (bounds_adjusted(upper_axis_bounds), bounds_adjusted(lower_axis_bounds))
    } else {
        (bounds_adjusted(lower_axis_bounds), bounds_adjusted(upper_axis_bounds))
    }
}

// all_possible takes:
// - workflows: the map of workflows
// - workflow: the workflow to process
// - bounds: the 4-space of possible values expressed as half-open intervals [x,y)
// and returns the number of accepted lattice points within bounds
fn all_possible(workflows: &HashMap<String, Vec<Instruction>>, workflow: &str, bounds: [(i32, i32); 4]) -> i128 {
    let possible_from_action = |a: &Action, subbounds: [(i32, i32); 4]| -> i128 {
        match a {
            Action::Reject => 0,
            Action::Accept => space_size(subbounds),
            Action::RunWorkflow(next_workflow) => all_possible(&workflows, &next_workflow, subbounds),
        }
    };

    let insts = &workflows[workflow];
    let mut tot = 0;
    let mut bounds = bounds.clone();
    for inst in insts {

        // If this is a conditional instruction, it creates a partition of the 4-space with
        // the condition as the split. The "true" portion of the space follows the action.
        if let Some(condition) = &inst.condition {
            let (true_bounds, false_bounds) = split_bounds_from_condition(bounds, condition);
            if let Some(subbounds) = true_bounds {
                tot += possible_from_action(&inst.action, subbounds);
            }
            if let Some(subbounds) = false_bounds {
                bounds = subbounds;            
                continue;
            }
            break; // If there is no false space, no need to continue.
        }
        // In the unconditional case, we just run the action.
        tot += possible_from_action(&inst.action, bounds);
    }
    tot
}

fn run(input: String) -> String {
    let mut lines_iter = input.lines();
    let workflow_re = Regex::new(r"(.*)\{([^}]+)\}").unwrap(); // matches e.g. px{a<2006:qkq,m>2090:A,rfg}
    let cond_re = Regex::new(r"([xmas])([><])(\d+)").unwrap();

    let mut workflows: HashMap<String, Vec<Instruction>> = HashMap::new();
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
                    Some(Condition{axis: value_idx, cmp, value})
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

    format!("{}", all_possible(&workflows, "in", [(1,4001),(1,4001),(1,4001),(1,4001)]))
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "167409079868000";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
