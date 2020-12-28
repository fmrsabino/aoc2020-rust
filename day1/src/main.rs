fn main() {
    let expenses: [i32; 6] = [1721, 979, 366, 299, 675, 1456, ];
    match find_solution(&expenses) {
        None => println!("No solution found"),
        Some(solution) => print!("Solution is {}", solution)
    }
}

fn find_solution(expenses: &[i32]) -> Option<i32> {
    for e1 in expenses.iter() {
        for e2 in expenses.split_first().unwrap().1 {
            if e1 + e2 == 2020 {
                return Some(e1 * e2);
            }
        }
    }
    return None;
}
