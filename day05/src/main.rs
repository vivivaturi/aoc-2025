// --- Day 5: Cafeteria ---
// As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.
//
// You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.
//
// "If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.
//
// The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).
//
// The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:
//
// 3-5
// 10-14
// 16-20
// 12-18
//
// 1
// 5
// 8
// 11
// 17
// 32
// The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.
//
// The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:
//
// Ingredient ID 1 is spoiled because it does not fall into any range.
// Ingredient ID 5 is fresh because it falls into range 3-5.
// Ingredient ID 8 is spoiled.
// Ingredient ID 11 is fresh because it falls into range 10-14.
// Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
// Ingredient ID 32 is spoiled.
// So, in this example, 3 of the available ingredient IDs are fresh.
//
// Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

use std::fs;

fn main() {
    let filename = "input.txt";
    println!("Part 1 solution: {}", solve_part1(&load_input(filename)));
    println!("Part 2 solution: {}", solve_part2(&load_input(filename)));
}

fn load_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = fs::read_to_string(input).expect("Unable to read file");
    file.split("\n\n").collect::<Vec<&str>>().iter().fold(
        (Vec::new(), Vec::new()),
        |(mut ranges, mut ids), section| {
            if ranges.is_empty() {
                for line in section.lines() {
                    let parts: Vec<u64> =
                        line.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
                    ranges.push((parts[0], parts[1]));
                }
            } else {
                for line in section.lines() {
                    ids.push(line.parse::<u64>().unwrap());
                }
            }
            (ranges, ids)
        },
    )
}

fn solve_part1(input: &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    let (ranges, ids) = input;
    ids.iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count()
}

// --- Part Two ---
// The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.
//
// So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.
//
// Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:
//
// 3-5
// 10-14
// 16-20
// 12-18
// The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.
//
// Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?

fn solve_part2(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let (ranges, _) = input;
    
    // Sort ranges by start value
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|&(start, _)| start);
    
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    for &(start, end) in &sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if start <= last.1 + 1 {
                // Overlapping or adjacent ranges - merge them
                last.1 = last.1.max(end);
                continue;
            }
        }
        // Non-overlapping range - add as new
        merged_ranges.push((start, end));
    }

    merged_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum()
}
