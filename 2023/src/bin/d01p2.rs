use aoc::*;
use std::str;

fn translate_digits(line: &str) -> String {
    let mut out = String::new();
    let mut s = line;
    while s.len() > 0 {
        let (first, rest) = s.split_at(1);
        if s.starts_with(|c: char| c.is_ascii_digit()) {
            out += first;
        } else if s.starts_with("zero") {
            out += "0";
        } else if s.starts_with("one") {
            out += "1";
        } else if s.starts_with("two") {
            out += "2";
        } else if s.starts_with("three") {
            out += "3";
        } else if s.starts_with("four") {
            out += "4";
        } else if s.starts_with("five") {
            out += "5";
        } else if s.starts_with("six") {
            out += "6";
        } else if s.starts_with("seven") {
            out += "7";
        } else if s.starts_with("eight") {
            out += "8";
        } else if s.starts_with("nine") {
            out += "9";
        }
        // println!("s: {}, out: {}", s, out);
        s = rest;
    }
    out
}

fn first_last(line: &str) -> i32 {
    // println!("before: {}", line);
    let line = translate_digits(line);
    // println!("after: {}", line);
    let digits: Vec<&str> = line.matches(|c: char| c.is_ascii_digit()).collect();
    let l = digits[0].to_string() + digits[digits.len() - 1];
    l.parse::<i32>().unwrap()
}

fn run(input: String) -> String {
    let mut t = 0;
    for l in input.lines() {
        let x = first_last(l);
        // println!("{} {}", l, x);
        t += x
    }
    format!("{}", t)
}

fn main() {
    println!("{}", run(real_input()));
}

#[cfg(test)]
const TEST_RESULT: &str = "281";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(day_input("-test2.txt".to_string()));
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
