use aoc::*;
use itertools::Itertools;
use std::collections::HashSet;

fn parse_ints(inp: &str) -> Vec<i32> {
    inp.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap()).collect()
}


fn run(input: String) -> String {
    let mut t: u32 = 0;
    for l in input.lines() {
        let (_, sets_string) = l.split(":").collect_tuple().unwrap();
        let (wants_string, gots_string) = sets_string.split("|").collect_tuple().unwrap();
        let wants = parse_ints(wants_string);
        let wantset: HashSet<&i32> = HashSet::from_iter(wants.iter());
        let gots = parse_ints(gots_string);

        let mut value = 0;
        for got in &gots {
            if !wantset.contains(&got) {
                continue;
            }
            if value == 0 {
                value = 1
            } else {
                value *= 2
            }
        }

        // println!("wants: {:?}, gots: {:?}, value: {:?}", wantset, gots, value);

        t += value;
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "13";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
