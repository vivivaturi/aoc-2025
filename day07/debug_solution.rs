use std::fs;
use std::collections::HashSet;

fn main() {
    let graph: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut all_states = HashSet::new();
    let mut states = HashSet::new();
    
    for (x, &cell) in graph[0].iter().enumerate() {
        if cell == 'S' {
            states.insert(x);
            all_states.insert((0, x));
            break;
        }
    }

    for (row_idx, row) in graph[1..].iter().enumerate() {
        let row_idx = row_idx + 1;
        let mut new_states = HashSet::new();

        for &pos in &states {
            if row[pos] == '^' {
                if pos > 0 {
                    new_states.insert(pos - 1);
                    all_states.insert((row_idx, pos - 1));
                }
                if pos < row.len() - 1 {
                    new_states.insert(pos + 1);
                    all_states.insert((row_idx, pos + 1));
                }
            } else {
                new_states.insert(pos);
                all_states.insert((row_idx, pos));
            }
        }

        states = new_states;
    }

    let total = all_states.len();
    let in_splitter_rows = all_states.iter().filter(|(r, _)| graph[*r].contains(&'^')).count();
    
    println!("Total visited: {}", total);
    println!("In splitter rows: {}", in_splitter_rows);
    println!("Final positions: {}", states.len());
}
