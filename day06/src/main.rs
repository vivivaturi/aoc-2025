// --- Day 6: Trash Compactor ---
// After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!
//
// A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.
//
// As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her math homework.
//
// Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of problems; each problem has a group of numbers that need to either be either added (+) or multiplied (*) together.
//
// However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:
//
// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +
// Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.
//
// So, this worksheet contains four problems:
//
// 123 * 45 * 6 = 33210
// 328 + 64 + 98 = 490
// 51 * 387 * 215 = 4243455
// 64 + 23 + 314 = 401
// To check their work, cephalopod students are given the grand total of adding together all of the answers to the individual problems. In this worksheet, the grand total is 33210 + 490 + 4243455 + 401 = 4277556.
//
// Of course, the actual worksheet is much wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.
//
// Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?
use std::fs;

fn main() {
    let grand_total = solve_part1("input.txt");
    println!("Grand total: {}", grand_total);

    let grant_total_part2 = solve_part2("input.txt");
    println!("Grand total part 2: {}", grant_total_part2);
}

fn load_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("Failed to read input file")
        .lines()
        .map(str::to_string)
        .collect()
}

fn parse_problems(input: Vec<String>) -> Vec<(Vec<u64>, char)> {
    parse_problems_inner(input, false)
}

fn find_problem_ranges(column_boundaries: &[bool]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = None;

    for (i, &is_space) in column_boundaries.iter().enumerate() {
        match (start, is_space) {
            (None, false) => start = Some(i),
            (Some(s), true) => {
                ranges.push((s, i));
                start = None;
            }
            _ => {}
        }
    }

    if let Some(s) = start {
        ranges.push((s, column_boundaries.len()));
    }

    ranges
}

fn read_column(numbers_lines: &[String], col: usize) -> u64 {
    let num_str: String = numbers_lines
        .iter()
        .filter_map(|line| line.chars().nth(col).filter(|c| c.is_numeric()))
        .collect();

    num_str.parse().unwrap_or(0)
}

fn parse_problems_part2(input: Vec<String>) -> Vec<(Vec<u64>, char)> {
    if input.len() < 5 {
        return Vec::new();
    }

    let numbers_lines = &input[0..4];
    let operators_line = &input[4];
    let width = numbers_lines[0].len();

    // Find which columns are all spaces (problem boundaries)
    let column_boundaries: Vec<bool> = (0..width)
        .map(|col| {
            numbers_lines
                .iter()
                .all(|line| line.chars().nth(col).unwrap_or(' ') == ' ')
        })
        .collect();

    let problem_ranges = find_problem_ranges(&column_boundaries);

    // Process problems in reverse order (right-to-left)
    problem_ranges
        .into_iter()
        .rev()
        .filter_map(|(start, end)| {
            // Within this problem, read columns from right to left
            let numbers: Vec<u64> = (start..end)
                .rev()
                .map(|col| read_column(numbers_lines, col))
                .filter(|&num| num > 0)
                .collect();

            let operator = operators_line.chars().nth(start).unwrap_or('+');

            if !numbers.is_empty() {
                Some((numbers, operator))
            } else {
                None
            }
        })
        .collect()
}

fn parse_problems_inner(input: Vec<String>, reverse: bool) -> Vec<(Vec<u64>, char)> {
    if input.len() < 5 {
        return Vec::new();
    }

    let numbers_lines = &input[0..4];
    let operators_line = &input[4];
    let width = numbers_lines[0].len();

    // Find column boundaries (transitions between all-spaces and non-all-spaces)
    let column_boundaries: Vec<bool> = (0..width)
        .map(|i| {
            numbers_lines
                .iter()
                .all(|line| line.chars().nth(i).unwrap_or(' ') == ' ')
        })
        .collect();

    // Group consecutive columns into problems
    let mut problems = Vec::new();
    let mut start = None;

    for (i, &is_space) in column_boundaries.iter().enumerate() {
        match (start, is_space) {
            (None, false) => start = Some(i), // Found problem start
            (Some(s), true) => {
                // Found problem end
                problems.push((s, i));
                start = None;
            }
            _ => {}
        }
    }

    // Handle last problem if it extends to the end
    if let Some(s) = start {
        problems.push((s, width));
    }

    // Reverse problems if reading right-to-left
    if reverse {
        problems.reverse();
    }

    // Extract numbers and operators for each problem
    problems
        .into_iter()
        .filter_map(|(start, end)| {
            let numbers: Vec<u64> = numbers_lines
                .iter()
                .filter_map(|line| {
                    line.chars()
                        .skip(start)
                        .take(end - start)
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .ok()
                })
                .collect();

            let operator = operators_line.chars().nth(start).unwrap_or('+');

            if !numbers.is_empty() {
                Some((numbers, operator))
            } else {
                None
            }
        })
        .collect()
}

fn solve_problem(numbers: Vec<u64>, operator: char) -> u64 {
    match operator {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => 0,
    }
}

fn calculate_grand_total(problems: Vec<(Vec<u64>, char)>) -> u64 {
    problems
        .iter()
        .map(|(numbers, operator)| solve_problem(numbers.clone(), *operator))
        .sum()
}

fn solve_part1(file_path: &str) -> u64 {
    let input = load_input(file_path);
    let problems = parse_problems(input);
    calculate_grand_total(problems)
}

// --- Part Two ---
// The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.
//
// Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)
//
// Here's the example worksheet again:
//
// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +
// Reading the problems right-to-left one column at a time, the problems are now quite different:
//
// The rightmost problem is 4 + 431 + 623 = 1058
// The second problem from the right is 175 * 581 * 32 = 3253600
// The third problem from the right is 8 + 248 + 369 = 625
// Finally, the leftmost problem is 356 * 24 * 1 = 8544
// Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.
//
// Solve the problems on the math worksheet again. What is the grand total found by adding together all of the answers to the individual problems?



fn solve_part2(file_path: &str) -> u64 {
    let input = load_input(file_path);
    let problems = parse_problems_part2(input);
    calculate_grand_total(problems)
}
