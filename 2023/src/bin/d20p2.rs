#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use aoc::*;
use itertools::Itertools;
use maplit::hashmap;
use multimap::MultiMap;
use ndarray::*;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    str,
};
use std::{collections::VecDeque, iter::Zip};
use tuple::*;

#[derive(Debug)]
enum Module {
    FlipFlop {
        to: Vec<String>,
        state: bool,
    },
    Conjunction {
        to: Vec<String>,
        state: HashMap<String, bool>,
    },
    Broadcast {
        to: Vec<String>,
    },
}

#[derive(Debug)]
struct Pulse {
    from: String,
    to: String,
    high: bool, // low == false
}

fn run(input: String) -> String {
    let spec_re = Regex::new(r"(.*) -> (.*)").unwrap(); // matches e.g. &qx -> cb, cv, bx, xz, vm, zl

    let mut module_map = HashMap::new();
    for l in input.lines() {
        let spec_match = spec_re.captures(l).unwrap();
        let to_modules = spec_match[2]
            .split(", ")
            .map(|s| s.to_owned())
            .collect_vec();
        let decl = &spec_match[1];

        // println!("{}, {:?}", &spec_match[1], to_modules);

        let (name, module) = {
            if decl == "broadcaster" {
                (decl.to_owned(), Module::Broadcast { to: to_modules })
            } else if decl.starts_with("%") {
                (
                    decl.trim_start_matches("%").to_owned(),
                    Module::FlipFlop {
                        to: to_modules,
                        state: false,
                    },
                )
            } else if decl.starts_with("&") {
                (
                    decl.trim_start_matches("&").to_owned(),
                    Module::Conjunction {
                        to: to_modules,
                        state: HashMap::new(),
                    },
                )
            } else {
                panic!("unknown spec {}", decl)
            }
        };

        module_map.insert(name, module);
    }

    let mut input_map = MultiMap::new();
    for (name, module) in &module_map {
        let mut add_edges = |to: &Vec<String>| {
            for to in to {
                input_map.insert(to.to_owned(), name.to_owned());
            }
        };
        match module {
            Module::Broadcast { to } => add_edges(&to),
            Module::Conjunction { to, .. } => add_edges(&to),
            Module::FlipFlop { to, .. } => add_edges(&to),
        }
    }

    for (name, module) in &mut module_map {
        if let Module::Conjunction { state, .. } = module {
            for input in input_map.get_vec(name).unwrap() {
                state.insert(input.to_owned(), false);
            }
        }
    }

    let mut care_about = hashmap! {
        "zl" => i64::MAX,
        "xf" => i64::MAX,
        "qn" => i64::MAX,
        "xn" => i64::MAX,
    };
    for i in 1..100000000000i64 {
        if i % 1000000 == 0 {
            println!("step {}M care_about={:?}", i / 1000000, care_about);
        }
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            from: "button".to_owned(),
            to: "broadcaster".to_owned(),
            high: false,
        });

        while let Some(p) = pulses.pop_front() {
            let mut send_to = |to: &Vec<String>, high: bool| {
                for to in to {
                    pulses.push_back(Pulse {
                        from: p.to.to_owned(),
                        to: to.to_owned(),
                        high,
                    })
                }
            };

            if let Some(dest) = module_map.get_mut(&p.to) {
                match dest {
                    Module::Broadcast { to } => {
                        send_to(to, p.high);
                    }
                    Module::FlipFlop { to, state } => {
                        if !p.high {
                            *state = !*state;
                            send_to(to, *state);
                        }
                    }
                    Module::Conjunction { to, state } => {
                        state.insert(p.from.to_owned(), p.high);
                        send_to(to, !state.iter().all(|(_, v)| *v));

                        if p.high && p.to == "th" {
                            let mut presses = care_about.get_mut(p.from.as_str()).unwrap();
                            *presses = min(*presses, i);

                            if care_about.iter().all(|(_, presses)| *presses < i64::MAX) {
                                println!("{:?}", care_about);
                                return format!("");
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("iterated out")
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
        assert_eq!(result.trim(), "32000000");
    }

    #[test]
    fn test_input2_works() {
        let result = run(day_input("-test2.txt".to_string()));
        assert_eq!(result.trim(), "11687500");
    }
}
