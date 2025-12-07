// --- Day 7: Laboratories - Idiomatic Rust Solution ---
// This solution uses functional programming patterns and iterator combinators
// for a more Rust-idiomatic approach.

use std::fs;
use std::collections::HashSet;

pub fn main() {
    let input = load_input("input.txt");
    println!("Part 1: {}", solve_part1(&input));
}

fn load_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve_part1(graph: &[Vec<char>]) -> usize {
    let mut beams = initialize_beams(graph);
    
    graph[1..]
        .iter()
        .map(|row| process_row(&mut beams, row))
        .sum()
}

/// Initialize beam positions from the starting row
fn initialize_beams(graph: &[Vec<char>]) -> HashSet<usize> {
    graph[0]
        .iter()
        .enumerate()
        .filter_map(|(x, &cell)| if cell == 'S' { Some(x) } else { None })
        .collect()
}

/// Process a single row and return the number of splits that occurred
fn process_row(beams: &mut HashSet<usize>, row: &[char]) -> usize {
    let (new_beams, split_count) = beams
        .iter()
        .fold(
            (HashSet::new(), 0),
            |(mut acc_beams, mut acc_splits), &pos| {
                match row.get(pos) {
                    Some(&'^') => {
                        // Beam hits a splitter, create new beams to left and right
                        if pos > 0 {
                            acc_beams.insert(pos - 1);
                        }
                        if pos < row.len() - 1 {
                            acc_beams.insert(pos + 1);
                        }
                        acc_splits += 1;
                    }
                    Some(_) => {
                        // Beam continues downwards
                        acc_beams.insert(pos);
                    }
                    None => {} // Out of bounds (shouldn't happen)
                }
                (acc_beams, acc_splits)
            },
        );

    *beams = new_beams;
    split_count
}
