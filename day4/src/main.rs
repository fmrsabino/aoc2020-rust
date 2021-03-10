use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let regex = Regex::new("([^: .]+):([^: .]+)")?;
    let input = fs::read_to_string("res/input.txt")?;

    let raw_passports = input
        .split("\n\n") // split by empty lines
        .map(|p| p.replace("\n", " "))// each passport entry in one line
        .collect::<Vec<String>>();


    let passports = raw_passports.iter()
        .map(|p|
            regex.captures_iter(p)
                .filter_map(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
                .collect::<HashMap<&str, &str>>()
        )
        .filter(|map| is_passport_valid(map))
        .count();

    println!("Valid passports: {}", passports);

    Ok(())
}

fn is_passport_valid(passports: &HashMap<&str, &str>) -> bool {
    passports.iter()
        .all(|(&k, v)|
            match k {
                "byr" => match v.parse::<i32>() {
                    Ok(v) => (1920..=2002).contains(&v),
                    _ => false
                },
                "iyr" => match v.parse::<i32>() {
                    Ok(b) => (2010..=2020).contains(&b),
                    _ => false
                },
                "eyr" => match v.parse::<i32>() {
                    Ok(b) => (2020..=2030).contains(&b),
                    _ => false
                },
                "hgt" => is_height_valid(v).unwrap_or(false),
                "hcl" => is_hair_color_valid(v).unwrap_or(false),
                "ecl" => is_eye_color_valid(v).unwrap_or(false),
                "pid" => v.len() <= 9 && usize::from_str(v).is_ok(),
                _ => false,
            }
        )
}

fn is_height_valid(height: &str) -> Option<bool> {
    let regex = Regex::new("([0-9]+)(cm|in)").ok()?;
    let captures = regex.captures(height)?;
    match (captures.get(2)?.as_str(), captures.get(1).unwrap().as_str().parse::<i32>().ok()?) {
        ("cm", v) => Some((150..=193).contains(&v)),
        ("in", v) => Some((59..=76).contains(&v)),
        _ => None
    }
}

fn is_hair_color_valid(hair_color: &str) -> Option<bool> {
    let regex = Regex::new("#[a-f0-9]{6}").ok()?;
    Some(regex.is_match(hair_color))
}

fn is_eye_color_valid(eye_color: &str) -> Option<bool> {
    let regex = Regex::new("(amb|blu|brn|gry|grn|hzl|oth)").ok()?;
    Some(regex.is_match(eye_color))
}
