mod alternative;

use alternative::*;

fn main() {
    let input = include_str!("../input.txt");
    let machines = parse_input(input);
    
    let mut total_presses = 0;
    let mut total_joltage = 0;
    
    println!("Comparing Part 1 (min presses) vs Part 2 (min joltage):\n");
    
    for (i, machine) in machines.iter().enumerate().take(10) {
        let presses = solve_machine_presses(machine);
        let joltage = solve_machine_joltage(machine);
        
        match (presses, joltage) {
            (Some(p), Some(j)) => {
                println!("Machine {:3}: {} presses, {} joltage", i + 1, p, j);
                total_presses += p;
                total_joltage += j;
            }
            _ => println!("Machine {:3}: No solution", i + 1),
        }
    }
    
    println!("\n--- First 10 Machines ---");
    println!("Part 1 (min presses):  {}", total_presses);
    println!("Part 2 (min joltage):  {}", total_joltage);
    
    // Now do all machines
    let mut total_presses_all = 0;
    let mut total_joltage_all = 0;
    
    for machine in &machines {
        if let Some(p) = solve_machine_presses(machine) {
            total_presses_all += p;
        }
        if let Some(j) = solve_machine_joltage(machine) {
            total_joltage_all += j;
        }
    }
    
    println!("\n--- All {} Machines ---", machines.len());
    println!("Part 1 (min presses):  {}", total_presses_all);
    println!("Part 2 (min joltage):  {}", total_joltage_all);
}
