// Alternative solution for Day 2
// Use this file to experiment with different approaches

use std::fs;

#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing alternative solution for Day 2");

    let content = load_input("input.txt")?;

    println!("Part 1: {}", solve_part1(&content));
    println!("Part 2: {}", solve_part2(&content));
    Ok(())
}

fn load_input(file_path: &str) -> Result<Vec<Range>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let ranges = content
        .trim()
        .split(',')
        .map(|line| -> Result<Range, Box<dyn std::error::Error>> {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return Err("Invalid range format".into());
            }
            Ok(Range {
                start: parts[0].parse()?,
                end: parts[1].parse()?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ranges)
}

fn part1_is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    len.is_multiple_of(2) && id_str[..len / 2] == id_str[len / 2..]
}

fn part2_is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    len.is_multiple_of(3)
        && id_str[..len / 3] == id_str[len / 3..2 * len / 3]
        && id_str[..len / 3] == id_str[2 * len / 3..]
}

pub fn solve_part1(nums: &[Range]) -> i64 {
    nums.iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&id| part1_is_invalid(id))
        .sum()
}

pub fn solve_part2(nums: &[Range]) -> i64 {
    nums.iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&id| part2_is_invalid(id))
        .sum()
}
