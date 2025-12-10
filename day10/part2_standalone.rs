// Standalone Part 2 solution: Minimize Joltage

use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
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

fn solve_gf2_min_joltage(
    matrix: Vec<Vec<bool>>,
    target: Vec<bool>,
    joltages: &[usize],
) -> Option<usize> {
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
    let mut min_joltage = usize::MAX;
    
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

fn solve_machine(machine: &Machine, minimize_joltage: bool) -> Option<usize> {
    let num_lights = machine.target.len();
    let num_buttons = machine.buttons.len();
    
    if num_lights == 0 || num_buttons == 0 {
        return None;
    }
    
    let mut matrix = vec![vec![false; num_buttons]; num_lights];
    
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light in button {
            if light < num_lights {
                matrix[light][button_idx] = true;
            }
        }
    }
    
    if minimize_joltage {
        solve_gf2_min_joltage(matrix, machine.target.clone(), &machine.joltages)
    } else {
        solve_gf2_min_presses(matrix, machine.target.clone())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let machines: Vec<Machine> = input.lines().map(|line| parse_machine(line)).collect();
    
    let mut total_presses = 0;
    let mut total_joltage = 0;
    
    println!("Comparing Part 1 (min presses) vs Part 2 (min joltage):\n");
    println!("{:>8} {:>10} {:>10}", "Machine", "Presses", "Joltage");
    println!("{}", "-".repeat(30));
    
    for (i, machine) in machines.iter().enumerate().take(20) {
        let presses = solve_machine(machine, false);
        let joltage = solve_machine(machine, true);
        
        match (presses, joltage) {
            (Some(p), Some(j)) => {
                println!("{:>8} {:>10} {:>10}", i + 1, p, j);
                total_presses += p;
                total_joltage += j;
            }
            _ => println!("{:>8} {:>10}", i + 1, "No solution"),
        }
    }
    
    println!("{}", "-".repeat(30));
    println!("{:>8} {:>10} {:>10}", "TOTAL", total_presses, total_joltage);
    
    // Now do all machines
    let mut total_presses_all = 0;
    let mut total_joltage_all = 0;
    
    for machine in &machines {
        if let Some(p) = solve_machine(machine, false) {
            total_presses_all += p;
        }
        if let Some(j) = solve_machine(machine, true) {
            total_joltage_all += j;
        }
    }
    
    println!("\n========== ALL {} MACHINES ==========", machines.len());
    println!("Part 1 (minimize button presses): {}", total_presses_all);
    println!("Part 2 (minimize joltage):        {}", total_joltage_all);
}
