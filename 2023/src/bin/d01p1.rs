use aoc::*;

fn first_last(line: &str) -> i32 {
    let digits: Vec<&str> = line.matches(|c: char| c.is_ascii_digit()).collect();
    let l = digits[0].to_string() + digits[digits.len()-1];
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
const TEST_RESULT: &str = "142";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_works() {
        let result = run(test_input());
        assert_eq!(result.trim(), TEST_RESULT);
    }
}
