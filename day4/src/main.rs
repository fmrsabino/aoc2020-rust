use std::collections::HashMap;

use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fs;
use std::error::Error;

struct Passport {
    _birth_year: String,
    _issue_year: String,
    _expiration_year: String,
    _height: String,
    _hair_color: String,
    _eye_color: String,
    _passport_id: String,
    _country_id: Option<String>,
}

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
        .filter_map(|map| build_passport(&map))
        .collect::<Vec<Passport>>();

    println!("Parsed {} passports", passports.len());

    Ok(())
}

fn build_passport(raw_data: &HashMap<&str, &str>) -> Option<Passport> {
    Some(
        Passport {
            _birth_year: raw_data.get("byr")?.to_string(),
            _issue_year: raw_data.get("iyr")?.to_string(),
            _expiration_year: raw_data.get("eyr")?.to_string(),
            _height: raw_data.get("hgt")?.to_string(),
            _hair_color: raw_data.get("hcl")?.to_string(),
            _eye_color: raw_data.get("ecl")?.to_string(),
            _passport_id: raw_data.get("pid")?.to_string(),
            _country_id: None,
        }
    )
}
