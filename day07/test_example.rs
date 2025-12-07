use std::collections::HashSet;

fn main() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Track paths as (current_row, current_position, path_history)
    let mut paths = HashSet::new();
    
    for (x, &cell) in input[0].iter().enumerate() {
        if cell == 'S' {
            paths.insert((0, x, String::new()));
            break;
        }
    }

    for (row_idx, row) in input[1..].iter().enumerate() {
        let row_idx = row_idx + 1;
        let mut new_paths = HashSet::new();

        for (_, pos, path) in &paths {
            if row[*pos] == '^' {
                if *pos > 0 {
                    let mut p = path.clone();
                    p.push('L');
                    new_paths.insert((row_idx, pos - 1, p));
                }
                if *pos < row.len() - 1 {
                    let mut p = path.clone();
                    p.push('R');
                    new_paths.insert((row_idx, pos + 1, p));
                }
            } else {
                new_paths.insert((row_idx, *pos, path.clone()));
            }
        }

        paths = new_paths;
    }

    println!("Number of distinct paths: {}", paths.len());
    println!("Expected: 40");
}
