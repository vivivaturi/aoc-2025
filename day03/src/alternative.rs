// Alternative optimized solution for Day 3 Part 2
// Uses a more efficient stack-based approach for finding the largest subsequence

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing alternative solution for Day 3 Part 2");

    // Test with example first
    println!("=== Testing with example ===");
    let test_content = load_input("test_input.txt")?;
    println!("Test Part 1: {}", solve_part1(&test_content));
    println!("Test Part 2 (optimized): {}", solve_part2_optimized(&test_content));
    
    // Test with actual input
    println!("\n=== Testing with actual input ===");
    let content = load_input("input.txt")?;
    println!("Part 1: {}", solve_part1(&content));
    println!("Part 2 (optimized): {}", solve_part2_optimized(&content));
    
    // Benchmark approaches
    println!("\n=== Benchmarking ===");
    benchmark_approaches(&content);
    
    Ok(())
}

fn load_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<String> = content.lines().map(|line| line.trim().to_string()).collect();
    Ok(lines)
}

// Original Part 1 solution for comparison
pub fn solve_part1(banks: &[String]) -> i32 {
    let mut total_joltage = 0;

    for bank in banks {
        let max_joltage = find_max_joltage_part1(bank);
        total_joltage += max_joltage;
    }

    total_joltage
}

fn find_max_joltage_part1(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().collect();
    
    if digits.len() < 2 {
        return 0;
    }
    
    let mut max_joltage = 0;
    
    for i in 0..(digits.len() - 1) {
        let mut max_after = '0';
        for j in (i + 1)..digits.len() {
            if digits[j] > max_after {
                max_after = digits[j];
            }
        }
        
        let joltage = format!("{}{}", digits[i], max_after).parse().unwrap_or(0);
        if joltage > max_joltage {
            max_joltage = joltage;
        }
    }
    
    max_joltage
}

// Optimized Part 2 solution using stack-based approach
pub fn solve_part2_optimized(banks: &[String]) -> i64 {
    let mut total_joltage: i64 = 0;

    for bank in banks {
        let max_joltage = find_max_joltage_part2_optimized(bank);
        total_joltage += max_joltage;
    }

    total_joltage
}

fn find_max_joltage_part2_optimized(line: &str) -> i64 {
    let digits: Vec<char> = line.chars().collect();
    let n = digits.len();

    if n < 12 {
        return 0;
    }

    // Stack-based approach: maintain a stack of selected digits
    // Pop smaller digits when we find a larger one, as long as we have enough digits left
    let mut stack = Vec::new();
    let to_select = 12;
    
    for (i, &digit) in digits.iter().enumerate() {
        // While we can pop (stack not empty, current digit is larger, and we have enough digits left)
        while !stack.is_empty() 
            && digit > *stack.last().unwrap() 
            && (stack.len() - 1 + (n - i) >= to_select) {
            stack.pop();
        }
        
        // Push current digit if we still need more
        if stack.len() < to_select {
            stack.push(digit);
        }
    }
    
    // Convert stack to number
    let joltage_str: String = stack.into_iter().collect();
    joltage_str.parse().unwrap_or(0)
}

// Alternative: Dynamic Programming approach for comparison
pub fn solve_part2_dp(banks: &[String]) -> i64 {
    let mut total_joltage: i64 = 0;

    for bank in banks {
        let max_joltage = find_max_joltage_part2_dp(bank);
        total_joltage += max_joltage;
    }

    total_joltage
}

fn find_max_joltage_part2_dp(line: &str) -> i64 {
    let digits: Vec<char> = line.chars().collect();
    let n = digits.len();
    let k = 12;

    if n < k {
        return 0;
    }

    // dp[i][j] = largest number using first i digits with j digits selected
    let mut dp = vec![vec![String::new(); k + 1]; n + 1];
    
    for i in 1..=n {
        for j in 1..=k.min(i) {
            // Option 1: don't take current digit
            let option1 = dp[i - 1][j].clone();
            
            // Option 2: take current digit
            let mut option2 = dp[i - 1][j - 1].clone();
            option2.push(digits[i - 1]);
            
            // Choose the better option (lexicographically larger)
            if option2.len() == j && (option1.is_empty() || option2 > option1) {
                dp[i][j] = option2;
            } else {
                dp[i][j] = option1;
            }
        }
    }
    
    dp[n][k].parse().unwrap_or(0)
}

// Benchmark function to compare approaches
pub fn benchmark_approaches(banks: &[String]) {
    use std::time::Instant;
    
    println!("Benchmarking approaches...");
    
    // Test optimized approach
    let start = Instant::now();
    let result_opt = solve_part2_optimized(banks);
    let duration_opt = start.elapsed();
    
    // Test DP approach
    let start = Instant::now();
    let result_dp = solve_part2_dp(banks);
    let duration_dp = start.elapsed();
    
    println!("Optimized approach: {} (took {:?})", result_opt, duration_opt);
    println!("DP approach: {} (took {:?})", result_dp, duration_dp);
    println!("Results match: {}", result_opt == result_dp);
}