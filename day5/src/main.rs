use std::fs;
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut ids = get_ids().unwrap_or(vec![]);

    match ids.iter().max() {
        None => { panic!("Could not find highest seat ID") }
        Some(max_seat_id) => { println!("Max seat id: {}", max_seat_id) }
    }

    match find_seat_number(&mut ids) {
        None => { panic!("Could not find my seat ID") }
        Some(my_seat_number) => { println!("My seat id: {}", my_seat_number) }
    }
    Ok(())
}

fn get_ids() -> Option<Vec<usize>> {
    let input = fs::read_to_string("res/input.txt").ok()?;
    let boarding_passes = input
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    Some(
        boarding_passes.iter()
            .map(|bp| {
                let row = parse_row(&bp[..7], 0..127);
                let column = parse_column(&bp[7..], 0..7);
                row * 8 + column
            }
            )
            .collect::<Vec<usize>>()
    )
}

fn find_seat_number(ids: &mut Vec<usize>) -> Option<usize> {
    ids.sort();
    for window in ids.windows(2) {
        if window[1] - window[0] == 2 { return Some(window[1] - 1); }
    }
    None
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