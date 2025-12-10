// --- Day 10: Factory ---
// Part 1: Lights-out puzzle (GF(2))
// Part 2: Counter increment problem (integer linear programming)

use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,        // target state of lights (Part 1)
    buttons: Vec<Vec<usize>>, // which lights/counters each button affects
    joltages: Vec<usize>,     // target counter values (Part 2)
}

fn main() {
    let machines = load_input("input.txt");
    println!("Part 1: {}", solve_problem1(&machines));
    println!("Part 2: {}", solve_problem2(&machines));
}

fn load_input(input: &str) -> Vec<Machine> {
    let input = fs::read_to_string(input).expect("Failed to read input file");
    input.lines().map(|line| parse_machine(line)).collect()
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Parse target lights from [.##.]
    let lights_str = parts[0].trim_matches(|c| c == '[' || c == ']');
    let target: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

    // Parse buttons from (1,3) (2) etc
    let mut buttons = Vec::new();
    let mut i = 1;
    while i < parts.len() {
        if parts[i].starts_with('(') && parts[i].ends_with(')') {
            let button_str = parts[i].trim_matches(|c| c == '(' || c == ')');
            if !button_str.is_empty() {
                let lights: Vec<usize> =
                    button_str.split(',').map(|s| s.parse().unwrap()).collect();
                buttons.push(lights);
            }
            i += 1;
        } else if parts[i].starts_with('{') {
            break;
        } else {
            i += 1;
        }
    }

    // Parse joltages from {35,47,29,...}
    let joltage_part = parts
        .iter()
        .find(|p| p.starts_with('{'))
        .expect("No joltage values found");

    let joltage_str = joltage_part.trim_matches(|c| c == '{' || c == '}');
    let joltages: Vec<usize> = joltage_str.split(',').map(|s| s.parse().unwrap()).collect();

    Machine {
        target,
        buttons,
        joltages,
    }
}

fn solve_problem1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|machine| solve_lights_out(machine))
        .sum()
}

fn solve_problem2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|machine| solve_joltage(machine))
        .sum()
}

// Part 1: Gaussian elimination over GF(2)
fn solve_lights_out(machine: &Machine) -> Option<usize> {
    let num_lights = machine.target.len();
    let num_buttons = machine.buttons.len();

    if num_lights == 0 || num_buttons == 0 {
        return None;
    }

    // Build matrix: matrix[light][button] = 1 if button toggles light
    let mut matrix = vec![vec![false; num_buttons]; num_lights];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light in button {
            if light < num_lights {
                matrix[light][button_idx] = true;
            }
        }
    }

    solve_gf2_min(matrix, machine.target.clone())
}

fn solve_gf2_min(matrix: Vec<Vec<bool>>, target: Vec<bool>) -> Option<usize> {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    if rows == 0 || cols == 0 {
        return None;
    }

    let mut aug_matrix = matrix.clone();
    for i in 0..rows {
        aug_matrix[i].push(target[i]);
    }

    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..cols {
        let mut found_pivot = false;
        for row in pivot_row..rows {
            if aug_matrix[row][col] {
                aug_matrix.swap(pivot_row, row);
                found_pivot = true;
                break;
            }
        }

        if !found_pivot {
            continue;
        }

        pivot_cols.push(col);

        for row in 0..rows {
            if row != pivot_row && aug_matrix[row][col] {
                for c in 0..=cols {
                    aug_matrix[row][c] ^= aug_matrix[pivot_row][c];
                }
            }
        }

        pivot_row += 1;
    }

    for row in pivot_row..rows {
        let all_zero = aug_matrix[row][..cols].iter().all(|&x| !x);
        if all_zero && aug_matrix[row][cols] {
            return None;
        }
    }

    let mut free_vars = Vec::new();
    for col in 0..cols {
        if !pivot_cols.contains(&col) {
            free_vars.push(col);
        }
    }

    let num_free = free_vars.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; cols];

        for (i, &col) in free_vars.iter().enumerate() {
            solution[col] = (mask & (1 << i)) != 0;
        }

        for (i, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut val = aug_matrix[i][cols];
            for col in 0..cols {
                if col != pivot_col && solution[col] {
                    val ^= aug_matrix[i][col];
                }
            }
            solution[pivot_col] = val;
        }

        let presses = solution.iter().filter(|&x| *x).count();
        min_presses = min_presses.min(presses);
    }

    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}

// Part 2: Solve integer linear system A*x = b (minimize button presses)
fn solve_joltage(machine: &Machine) -> Option<usize> {
    let num_counters = machine.joltages.len();
    let num_buttons = machine.buttons.len();

    if num_counters == 0 || num_buttons == 0 {
        return None;
    }

    // Build matrix: matrix[counter][button] = 1 if button increments counter
    let mut matrix = vec![vec![0i64; num_buttons]; num_counters];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter in button {
            if counter < num_counters {
                matrix[counter][button_idx] = 1;
            }
        }
    }

    let target: Vec<i64> = machine.joltages.iter().map(|&x| x as i64).collect();

    solve_integer_system(&matrix, &target)
}

fn solve_integer_system(matrix: &[Vec<i64>], target: &[i64]) -> Option<usize> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    // Use Gaussian elimination to find solution space
    let mut aug = matrix.to_vec();
    for i in 0..rows {
        aug[i].push(target[i]);
    }
    
    // Perform Gaussian elimination
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;
    
    for col in 0..cols {
        // Find non-zero pivot
        let mut found = false;
        for row in pivot_row..rows {
            if aug[row][col] != 0 {
                aug.swap(pivot_row, row);
                found = true;
                break;
            }
        }
        
        if !found {
            continue;
        }
        
        pivot_cols.push(col);
        
        // Eliminate below (for reduced row echelon, we'd also eliminate above)
        for row in 0..rows {
            if row == pivot_row {
                continue;
            }
            
            if aug[row][col] != 0 {
                // Use cross multiplication to avoid fractions
                let factor_row = aug[row][col];
                let factor_pivot = aug[pivot_row][col];
                
                for c in 0..=cols {
                    aug[row][c] = aug[row][c] * factor_pivot - aug[pivot_row][c] * factor_row;
                }
            }
        }
        
        pivot_row += 1;
    }
    
    // Check for inconsistency
    for row in pivot_row..rows {
        let all_zero = aug[row][..cols].iter().all(|&x| x == 0);
        if all_zero && aug[row][cols] != 0 {
            return None; // No solution
        }
    }
    
    // Find free variables
    let mut free_vars = Vec::new();
    for col in 0..cols {
        if !pivot_cols.contains(&col) {
            free_vars.push(col);
        }
    }
    
    // Try to find minimum solution
    if free_vars.is_empty() {
        // Unique solution - check if it's valid (non-negative integers)
        let mut solution = vec![0i64; cols];
        
        for (i, &col) in pivot_cols.iter().enumerate() {
            let denom = aug[i][col];
            let num = aug[i][cols];
            
            if denom == 0 || num % denom != 0 {
                return None;
            }
            
            let val = num / denom;
            if val < 0 {
                return None;
            }
            
            solution[col] = val;
        }
        
        return Some(solution.iter().map(|&x| x as usize).sum());
    }
    
    // Multiple solutions - search for minimum
    enumerate_integer_solutions(&aug, &pivot_cols, &free_vars, cols)
}

fn enumerate_integer_solutions(
    aug: &[Vec<i64>],
    pivot_cols: &[usize],
    free_vars: &[usize],
    cols: usize,
) -> Option<usize> {
    let max_target = aug.iter().map(|row| row[cols].abs()).max().unwrap_or(0);
    let max_search = (max_target as usize).min(1000);
    
    let mut min_total = None;
    
    // Use DFS with pruning
    fn search(
        depth: usize,
        free_vars: &[usize],
        solution: &mut Vec<i64>,
        aug: &[Vec<i64>],
        pivot_cols: &[usize],
        cols: usize,
        max_search: usize,
        current_sum: usize,
        min_total: &mut Option<usize>,
    ) {
        // Prune if already exceeded minimum
        if let Some(min) = *min_total {
            if current_sum >= min {
                return;
            }
        }
        
        if depth == free_vars.len() {
            // Calculate dependent variables
            let mut valid = true;
            let mut dependent_sum = 0;
            
            for (i, &col) in pivot_cols.iter().enumerate() {
                let mut num = aug[i][cols];
                
                // Subtract contributions from free variables
                for &fv in free_vars {
                    num -= aug[i][fv] * solution[fv];
                }
                
                let denom = aug[i][col];
                if denom == 0 || num % denom != 0 {
                    valid = false;
                    break;
                }
                
                let val = num / denom;
                if val < 0 {
                    valid = false;
                    break;
                }
                
                solution[col] = val;
                dependent_sum += val as usize;
            }
            
            if valid {
                let total = current_sum + dependent_sum;
                *min_total = Some(match *min_total {
                    None => total,
                    Some(m) => m.min(total),
                });
            }
            return;
        }
        
        let fv = free_vars[depth];
        for val in 0..=max_search {
            if let Some(min) = *min_total {
                if current_sum + val >= min {
                    break; // Prune - can't improve
                }
            }
            
            solution[fv] = val as i64;
            search(
                depth + 1,
                free_vars,
                solution,
                aug,
                pivot_cols,
                cols,
                max_search,
                current_sum + val,
                min_total,
            );
        }
    }
    
    let mut solution = vec![0i64; cols];
    search(0, free_vars, &mut solution, aug, pivot_cols, cols, max_search, 0, &mut min_total);
    
    min_total
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(input);
        assert_eq!(solve_joltage(&machine), Some(10));
    }

    #[test]
    fn test_part2_example2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(input);
        assert_eq!(solve_joltage(&machine), Some(12));
    }

    #[test]
    fn test_part2_example3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(input);
        assert_eq!(solve_joltage(&machine), Some(11));
    }
}
