use std::collections::HashSet;

fn main() {
    // Use the actual example from the problem
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

    let mut states = HashSet::new();
    let mut all_visited = HashSet::new();
    
    for (x, &cell) in input[0].iter().enumerate() {
        if cell == 'S' {
            states.insert(x);
            all_visited.insert((0, x));
            break;
        }
    }

    for (row_idx, row) in input[1..].iter().enumerate() {
        let row_idx = row_idx + 1;
        let mut new_states = HashSet::new();

        for &pos in &states {
            if row[pos] == '^' {
                if pos > 0 {
                    new_states.insert(pos - 1);
                    all_visited.insert((row_idx, pos - 1));
                }
                if pos < row.len() - 1 {
                    new_states.insert(pos + 1);
                    all_visited.insert((row_idx, pos + 1));
                }
            } else {
                new_states.insert(pos);
                all_visited.insert((row_idx, pos));
            }
        }

        states = new_states;
    }

    println!("Total visited: {}", all_visited.len());
    println!("In splitter rows: {}", all_visited.iter().filter(|(r, _)| input[*r].contains(&'^')).count());
    println!("Expected: 40");
}
