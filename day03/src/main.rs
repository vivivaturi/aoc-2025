// --- Day 3: Lobby ---
// You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.
//
// "Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."
//
// You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."
//
// "But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."
//
// There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:
//
// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
// The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
//
// You'll need to find the largest possible joltage each bank can produce. In the above example:
//
// In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
// In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
// In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
// In 818181911112111, the largest joltage you can produce is 92.
// The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
//
// There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?
//

use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("Part 1: {}", solve_part1(file_path).unwrap());
    println!("Part 2: {}", solve_part2(file_path).unwrap());
}

fn load_input(_input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(_input).expect("Failed to read input file");
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

    Ok(lines)
}

fn solve_part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let banks = load_input(input)?;
    let mut total_joltage = 0;

    for bank in banks {
        let max_joltage = find_max_joltage(&bank);
        total_joltage += max_joltage;
    }

    Ok(total_joltage)
}

fn find_max_joltage(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().collect();

    if digits.len() < 2 {
        return 0;
    }

    // Track the best joltage found so far
    let mut max_joltage = 0;

    // For each position, find the best digit that can come after it
    for i in 0..(digits.len() - 1) {
        // Find the largest digit that comes after position i
        let mut max_after = '0';
        for j in (i + 1)..digits.len() {
            if digits[j] > max_after {
                max_after = digits[j];
            }
        }

        // Form the joltage and update if it's better
        let joltage = format!("{}{}", digits[i], max_after).parse().unwrap_or(0);
        if joltage > max_joltage {
            max_joltage = joltage;
        }
    }

    max_joltage
}

// --- Part Two ---
// The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.
//
// Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.
//
// The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.
//
// Consider again the example from before:
//
// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
// Now, the joltages are much larger:
//
// In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
// In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
// In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
// In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
// The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.
//

fn solve_part2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let banks = load_input(input)?;
    let mut total_joltage: i64 = 0;

    for bank in banks {
        let max_joltage = find_max_joltage_part2(&bank);
        total_joltage += max_joltage;
    }

    Ok(total_joltage)
}

fn find_max_joltage_part2(line: &str) -> i64 {
    let digits: Vec<char> = line.chars().collect();
    let n = digits.len();

    if n < 12 {
        return 0;
    }

    // Find the lexicographically largest subsequence of length 12
    let mut selected_digits = Vec::new();
    let mut start_index = 0;

    for i in 0..12 {
        // For the i-th digit, we can only look up to position n - (12 - i) + 1
        // because we need to leave enough digits for the remaining selections
        let max_search_index = n - (12 - i);
        
        let mut max_digit = '0';
        let mut max_index = start_index;

        for j in start_index..=max_search_index {
            if digits[j] > max_digit {
                max_digit = digits[j];
                max_index = j;
            }
        }

        selected_digits.push(max_digit);
        start_index = max_index + 1;
    }

    // Convert selected digits to an integer
    let joltage_str: String = selected_digits.into_iter().collect();
    joltage_str.parse().unwrap_or(0)
}