// --- Day 4: Printing Department ---
// You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).
//
// Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.
//
// "Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."
//
// If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.
//
// The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.
//
// For example:
//
// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
// The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.
//
// In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):
//
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
// Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("Part 1 solution: {}", solve_part1(&load_input(file_path)));
    println!("Part 2 solution: {}", solve_part2(&load_input(file_path)));
}

struct Points {
    row: isize,
    col: isize,
}

fn load_input(input: &str) -> Vec<Points> {
    let input = fs::read_to_string(input).expect("Failed to read input file");
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c == '@' {
                    Some(Points {
                        row: row as isize,
                        col: col as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn solve_part1(points: &Vec<Points>) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    points
        .iter()
        .filter(|point| {
            let mut count = 0;
            for (dr, dc) in directions.iter() {
                let neighbor = Points {
                    row: point.row + dr,
                    col: point.col + dc,
                };
                if points
                    .iter()
                    .any(|p| p.row == neighbor.row && p.col == neighbor.col)
                {
                    count += 1;
                }
            }
            count < 4
        })
        .count()
}

// --- Part Two ---
// Now, the Elves just need help accessing as much of the paper as they can.
//
// Once a roll of paper can be accessed by a forklift, it can be removed. Once a roll of paper is removed, the forklifts might be able to access more rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?
//
// Starting with the same example as above, here is one way you could remove as many rolls of paper as possible, using highlighted @ to indicate that a roll of paper is about to be removed, and using x to indicate that a roll of paper was just removed:
//
// Initial state:
// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
//
// Remove 13 rolls of paper:
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
//
// Remove 12 rolls of paper:
// .......x..
// .@@.x.x.@x
// x@@@@...@@
// x.@@@@..x.
// .@.@@@@.x.
// .x@@@@@@.x
// .x.@.@.@@@
// ..@@@.@@@@
// .x@@@@@@@.
// ....@@@...
//
// Remove 7 rolls of paper:
// ..........
// .x@.....x.
// .@@@@...xx
// ..@@@@....
// .x.@@@@...
// ..@@@@@@..
// ...@.@.@@x
// ..@@@.@@@@
// ..x@@@@@@.
// ....@@@...
//
// Remove 5 rolls of paper:
// ..........
// ..x.......
// .x@@@.....
// ..@@@@....
// ...@@@@...
// ..x@@@@@..
// ...@.@.@@.
// ..x@@.@@@x
// ...@@@@@@.
// ....@@@...
//
// Remove 2 rolls of paper:
// ..........
// ..........
// ..x@@.....
// ..@@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@x.
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ...@@.....
// ..x@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ...x@.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ....x.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ..........
// ...x@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
// Stop once no more rolls of paper are accessible by a forklift. In this example, a total of 43 rolls of paper can be removed.
//
// Start with your original diagram. How many rolls of paper in total can be removed by the Elves and their forklifts?

fn solve_part2(points: &Vec<Points>) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    
    let mut remaining_points: std::collections::HashSet<(isize, isize)> = points
        .iter()
        .map(|p| (p.row, p.col))
        .collect();
    
    let mut total_removed = 0;
    
    loop {
        let mut to_remove = Vec::new();
        
        for &(row, col) in &remaining_points {
            let mut count = 0;
            for (dr, dc) in directions.iter() {
                let neighbor = (row + dr, col + dc);
                if remaining_points.contains(&neighbor) {
                    count += 1;
                }
            }
            if count < 4 {
                to_remove.push((row, col));
            }
        }
        
        if to_remove.is_empty() {
            break;
        }
        
        total_removed += to_remove.len();
        for point in to_remove {
            remaining_points.remove(&point);
        }
    }
    
    total_removed
}
