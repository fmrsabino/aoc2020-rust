use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("res/input.txt")?;

    let sum1: usize = input.split("\n\n")
        .map(|group|
            group.chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<HashSet<_>>()
                .len()
        )
        .sum();

    let sum2 = input.split("\n\n")
        .map(|group| {
            let counts: HashMap<char, usize> = group.chars().counts();
            let required = group.lines().count();

            counts.iter()
                .filter(|&(k, v)| *k != '\n' && *v >= required)
                .count()
        })
        .sum::<usize>();


    println!("Part 1 count: {}", sum1);
    println!("Part 2 count: {}", sum2);
    Ok(())
}
