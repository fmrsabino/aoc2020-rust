use std::{fmt, fs};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("res/input.txt")?;
    let regex = Regex::new("(.+) bags contain (.+)")?;

    let x = regex.captures_iter(&input)
        .filter_map(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        .filter_map(|(bag, contains)|
            Some((bag, format_bag_contents(contains)?))
        ).collect::<HashMap<&str, Vec<BagContent>>>();

    for (q, e) in x {
        println!("{:?} -> {:?}", q, e)
    }

    Ok(())
}

struct BagContent {
    quantity: usize,
    bag_type: String,
}

impl Debug for BagContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BagContent")
            .field("quantity", &self.quantity)
            .field("bag_type", &self.bag_type)
            .finish()
    }
}

fn format_bag_contents(raw_contents: &str) -> Option<Vec<BagContent>> {
    let regex = Regex::new("([0-9]+) (.+) bags?").ok()?;

    Some(
        raw_contents.trim_end_matches(|c| c == '.')
            .split(",")
            .map(|content| content.trim())
            .flat_map(|content| regex.captures_iter(content))
            .filter_map(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
            .filter_map(|(quantity, value)|
                match quantity.parse::<usize>() {
                    Ok(quantity) => Some(BagContent { quantity, bag_type: value.to_string() }),
                    _ => None
                }
            )
            .collect::<Vec<BagContent>>()
    )
}