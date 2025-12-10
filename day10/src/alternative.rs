// Alternative solution for Part 2: Minimize Joltage instead of Button Presses

#[derive(Debug, Clone)]
pub struct Machine {
    pub target: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltages: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(|line| parse_machine(line)).collect()
}

pub fn parse_machine(line: &str) -> Machine {
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
                let lights: Vec<usize> = button_str
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
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
    let joltage_part = parts.iter()
        .find(|p| p.starts_with('{'))
        .expect("No joltage values found");
    
    let joltage_str = joltage_part.trim_matches(|c| c == '{' || c == '}');
    let joltages: Vec<usize> = joltage_str
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    
    Machine { target, buttons, joltages }
}

// Solve for minimum joltage (instead of minimum presses)
pub fn solve_gf2_min_joltage(
    matrix: Vec<Vec<bool>>,
    target: Vec<bool>,
    joltages: &[usize],
) -> Option<usize> {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    
    if rows == 0 || cols == 0 {
        return None;
    }
    
    // Create augmented matrix
    let mut aug_matrix = matrix.clone();
    for i in 0..rows {
        aug_matrix[i].push(target[i]);
    }
    
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;
    
    // Forward elimination - reduced row echelon form
    for col in 0..cols {
        // Find pivot
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
        
        // Eliminate all other rows
        for row in 0..rows {
            if row != pivot_row && aug_matrix[row][col] {
                for c in 0..=cols {
                    aug_matrix[row][c] ^= aug_matrix[pivot_row][c];
                }
            }
        }
        
        pivot_row += 1;
    }
    
    // Check for inconsistency
    for row in pivot_row..rows {
        let all_zero = aug_matrix[row][..cols].iter().all(|&x| !x);
        if all_zero && aug_matrix[row][cols] {
            return None;
        }
    }
    
    // Find free variables (columns without pivots)
    let mut free_vars = Vec::new();
    for col in 0..cols {
        if !pivot_cols.contains(&col) {
            free_vars.push(col);
        }
    }
    
    // Try all combinations of free variables to find minimum JOLTAGE
    let num_free = free_vars.len();
    let mut min_joltage = usize::MAX;
    
    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; cols];
        
        // Set free variables according to mask
        for (i, &col) in free_vars.iter().enumerate() {
            solution[col] = (mask & (1 << i)) != 0;
        }
        
        // Calculate pivot variables based on free variables
        for (i, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut val = aug_matrix[i][cols];
            for col in 0..cols {
                if col != pivot_col && solution[col] {
                    val ^= aug_matrix[i][col];
                }
            }
            solution[pivot_col] = val;
        }
        
        // Calculate total JOLTAGE for this solution
        let joltage: usize = solution
            .iter()
            .enumerate()
            .filter(|(_, &pressed)| pressed)
            .map(|(idx, _)| joltages[idx])
            .sum();
        
        min_joltage = min_joltage.min(joltage);
    }
    
    if min_joltage == usize::MAX {
        None
    } else {
        Some(min_joltage)
    }
}

pub fn solve_machine_joltage(machine: &Machine) -> Option<usize> {
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
    
    // Solve for minimum joltage
    solve_gf2_min_joltage(matrix, machine.target.clone(), &machine.joltages)
}

pub fn solve_machine_presses(machine: &Machine) -> Option<usize> {
    let num_lights = machine.target.len();
    let num_buttons = machine.buttons.len();
    
    if num_lights == 0 || num_buttons == 0 {
        return None;
    }
    
    // Build matrix
    let mut matrix = vec![vec![false; num_buttons]; num_lights];
    
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light in button {
            if light < num_lights {
                matrix[light][button_idx] = true;
            }
        }
    }
    
    // Solve for minimum presses (same as Part 1 but now we have joltage data)
    solve_gf2_min_presses(matrix, machine.target.clone())
}

fn solve_gf2_min_presses(matrix: Vec<Vec<bool>>, target: Vec<bool>) -> Option<usize> {
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
        
        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }
    
    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_with_joltage() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7,8,9}";
        let machine = parse_machine(input);
        assert_eq!(machine.target, vec![false, true, true, false]);
        assert_eq!(machine.buttons.len(), 6);
        assert_eq!(machine.joltages, vec![3, 5, 4, 7, 8, 9]);
    }
}
