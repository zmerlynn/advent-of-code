use aoc::*;
use itertools::Itertools;

fn parse_ints(inp: &str) -> Vec<i64> {
    inp.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap()).collect()
}

fn run(input: String) -> String {
    let mut out = 1;
    let mut input_iter = input.lines();
    let race_times = parse_ints(input_iter.next().unwrap().split(":").collect_tuple::<(&str,&str)>().unwrap().1);
    let race_record_distances = parse_ints(input_iter.next().unwrap().split(":").collect_tuple::<(&str,&str)>().unwrap().1);
    println!("{:?} {:?}", race_times, race_record_distances);
    assert_eq!(race_times.len(), race_record_distances.len());

    // r = race time
    // p = press time
    // d = race distance
    // d = p * (r - p)

    for i in 0..race_times.len() {
        let race_time = race_times[i];
        let race_record = race_record_distances[i];

        let mut count = 0;
        for press_time in 1..race_time {
            if press_time * (race_time - press_time) > race_record {
                count += 1;
            }
        }
        println!("count: {}", count);
        out *= count;
    }

    format!("{}", out)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "288";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
