use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

const TREE: char = '#';

fn tree_hit(position: (usize, usize), nav_map: Vec<String>) -> Result<i32, Error> {
    let new_pos: (usize, usize) = (position.0 + 1, (position.1 + 3) % nav_map.get(position.1).ok_or(ErrorKind::InvalidData)?.len());

    if new_pos.0 >= nav_map.len() {
        // reached end of slope
        return Ok(0);
    }

    let char_at_new_pos = nav_map.get(new_pos.0).unwrap().chars().nth(new_pos.1).ok_or(ErrorKind::InvalidData)?;

    return Ok(tree_hit(new_pos, nav_map)? + if char_at_new_pos == TREE { 1 } else { 0 });
}

fn main() -> Result<(), Error> {
    let file = File::open("res/slope.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let slope: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let tree_hits = tree_hit((0, 0), slope)?;
    Ok(println!("Hit {} trees going down the slope", tree_hits))
}
