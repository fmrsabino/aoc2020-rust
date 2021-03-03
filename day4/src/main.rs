use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use regex::Regex;

struct Passport {
    _birth_year: Year,
    _issue_year: Year,
    _expiration_year: Year,
    _height: String,
    _hair_color: String,
    _eye_color: String,
    _passport_id: String,
    _country_id: Option<String>,
}

struct YearRange {
    value: (usize, usize),
}

impl YearRange {
    fn new(range: (usize, usize)) -> Option<Self> {
        return if range.0 < range.1 { Some(YearRange { value: range }) } else { None };
    }
}

struct Year {
    _value: usize,
}

impl Year {
    fn new(value: &str, range: YearRange) -> Option<Self> {
        let v = usize::from_str(value).ok()?;
        return if v >= range.value.0 && v <= range.value.1 { Some(Year { _value: v }) } else { None };
    }
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
            _birth_year: Year::new(raw_data.get("byr")?, YearRange::new((1920, 2002))?)?,
            _issue_year: Year::new(raw_data.get("iyr")?, YearRange::new((2010, 2020))?)?,
            _expiration_year: Year::new(raw_data.get("eyr")?, YearRange::new((2020, 2030))?)?,
            _height: raw_data.get("hgt")?.to_string(),
            _hair_color: raw_data.get("hcl")?.to_string(),
            _eye_color: raw_data.get("ecl")?.to_string(),
            _passport_id: raw_data.get("pid")?.to_string(),
            _country_id: None,
        }
    )
}
