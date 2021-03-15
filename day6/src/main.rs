use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("res/input.txt")?;

    let sum: usize = input.split("\n\n")
        .map(|group|
            group.chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<HashSet<_>>()
                .len()
        )
        .sum();

    println!("Count of sum: {}", sum);
    Ok(())
}
