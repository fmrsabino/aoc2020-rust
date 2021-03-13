use std::fs;
use std::ops::Range;

fn main() {
    println!("Hello, world!");
    let row = parse_row("BBFFBBF", 0..127);
    let column = parse_column("RLL", 0..7);

    let seat_id = row * 8 + column;
    println!("id: {}, Row: {}, Column: {}", seat_id, row, column);

    match binary_boarding_1() {
        None => { panic!("Could not find highest seat ID") }
        Some(max_seat_id) => { println!("Max seat id: {}", max_seat_id) }
    }
}

// Highest seat id
fn binary_boarding_1() -> Option<usize> {
    let input = fs::read_to_string("res/input.txt").ok()?;
    let boarding_passes = input
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    boarding_passes.iter()
        .map(|bp| {
            let row = parse_row(&bp[..7], 0..127);
            let column = parse_column(&bp[7..], 0..7);
            row * 8 + column
        }
        )
        .max()
}

fn parse_row(rows: &str, range: Range<usize>) -> usize {
    if range.start == range.end { return range.start; }
    let split_size = ((range.end + 1) - (range.start)) / 2;
    match rows.chars().next() {
        Some('F') => { parse_row(&rows[1..], range.start..range.end - split_size) }
        Some('B') => { parse_row(&rows[1..], range.start + split_size..range.end) }
        _ => panic!("Unrecognizable character")
    }
}

fn parse_column(columns: &str, range: Range<usize>) -> usize {
    if range.start == range.end { return range.start; }
    let split_size = ((range.end + 1) - (range.start)) / 2;
    match columns.chars().next() {
        Some('L') => { parse_column(&columns[1..], range.start..(range.end - split_size)) }
        Some('R') => { parse_column(&columns[1..], (range.start + split_size)..range.end) }
        _ => unreachable!()
    }
}