use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let boxes: Vec<(i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();

    println!("Total boxes: {}", boxes.len());
    
    // Find boxes with coordinates 216,146,977 and 117,168,530
    for (idx, box_coords) in boxes.iter().enumerate() {
        if box_coords.0 == 216 && box_coords.1 == 146 && box_coords.2 == 977 {
            println!("Found 216,146,977 at index {}", idx);
        }
        if box_coords.0 == 117 && box_coords.1 == 168 && box_coords.2 == 530 {
            println!("Found 117,168,530 at index {}", idx);
        }
    }
}
