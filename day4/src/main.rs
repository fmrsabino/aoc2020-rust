use std::collections::HashMap;

use regex::Regex;

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

fn main() {
    let regex = Regex::new("([^: .]+):([^: .]+)").unwrap();
    let raw_passport_1 = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm");

    let raw_data: HashMap<&str, &str> = regex.captures_iter(raw_passport_1.as_str())
        .filter_map(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        .collect();

    match build_passport(&raw_data) {
        None => { println!("Passport is not valid") }
        Some(_) => { println!("Passport is valid") }
    }
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
