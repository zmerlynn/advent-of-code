use std::env;
use std::fs;

// day_input finds the appropriate input for the day
pub fn day_input(suffix: String) -> String {
    let mut path = env::current_exe().unwrap();
    let name = path.file_name().unwrap().to_str().unwrap()[..3].to_string() + &suffix;

    loop {
        path.pop();
        if path.file_name().unwrap() == "target" {
            break;
        }
    }
    path.pop();
    path.push("input".to_string());
    path.push(&name);

    fs::read_to_string(path.to_str().unwrap())
        .expect(format!("Should have been able to read {:?}", &path).as_str())
}

pub fn real_input() -> String {
    day_input(".txt".to_string())
}

pub fn test_input() -> String {
    day_input("-test.txt".to_string())
}

pub fn parse_ints(inp: &str) -> Vec<i64> {
    inp.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap()).collect()
}

pub fn parse_ints_generic(inp: &str, p: &str) -> Vec<i64> {
    inp.split(p).map(|s| s.trim().parse::<i64>().unwrap()).collect()
}