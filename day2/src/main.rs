use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use regex::Regex;

struct Policy {
    min_occurrences: u32,
    max_occurrences: u32,
    char: char,
}

fn is_password_valid(password: &String, policy: &Policy) -> bool {
    let char_count = password.chars().filter(|c| { policy.char == *c }).count();
    return char_count >= policy.min_occurrences as usize && char_count <= policy.max_occurrences as usize;
}

fn main() -> Result<(), Error> {
    let regex = Regex::new("([0-9]+)-([0-9+]) ([a-z]): (.*)").unwrap();

    let file = File::open("src/policies.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for cap in regex.captures_iter(line?.as_str()) {
            let is_valid = is_password_valid(
                &cap[4].to_string(),
                Policy {
                    min_occurrences: cap[1].parse().unwrap(),
                    max_occurrences: cap[2].parse().unwrap(),
                    char: cap[3].parse().unwrap(),
                }.borrow(),
            );
            println!("{0} is {1}", cap[4].to_string(), if is_valid { "valid" } else { "invalid" })
        }
    }

    Ok(())
}
