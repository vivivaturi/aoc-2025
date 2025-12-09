// --- Day 9: Movie Theater ---
// You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!
//
// The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).
//
// For example:
//
// 7,1
// 11,1
// 11,7
// 9,7
// 9,5
// 2,5
// 2,3
// 7,3
// Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:
//
// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............
// You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.
//
// For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:
//
// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..OOOOOOOO....
// ..OOOOOOOO....
// ..OOOOOOOO.#..
// ..............
// Or, you could make a rectangle with area 35 between 7,1 and 11,7:
//
// ..............
// .......OOOOO..
// .......OOOOO..
// ..#....OOOOO..
// .......OOOOO..
// ..#....OOOOO..
// .......OOOOO..
// .......OOOOO..
// ..............
// You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:
//
// ..............
// .......#...#..
// ..............
// ..OOOOOO......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............
// Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:
//
// ..............
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..............
// .........#.#..
// ..............
// Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?

use std::fs;
use std::collections::HashSet;

fn main() {
    let tiles = load_input("input.txt");
    println!("Part 1 - Largest rectangle area: {}", solve_problem1(&tiles));
    println!("Part 2 - Largest rectangle area (red/green only): {}", solve_problem2(&tiles));
}

fn load_input(input: &str) -> Vec<(usize, usize)> {
    let input = fs::read_to_string(input).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts
                .next()
                .expect("Missing x coordinate")
                .parse::<usize>()
                .expect("Invalid x coordinate");
            let y = parts
                .next()
                .expect("Missing y coordinate")
                .parse::<usize>()
                .expect("Invalid y coordinate");
            (x, y)
        })
        .collect()
}

fn solve_problem1(tiles: &Vec<(usize, usize)>) -> usize {
    let mut max_area = 0;

    // Iterate through unique pairs only
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Skip pairs that share the same x or y coordinate
            // Opposite corners must be diagonal from each other
            if x1 == x2 || y1 == y2 {
                continue;
            }

            // Calculate area including both corner tiles
            let width = (x2 as isize - x1 as isize).abs() + 1;
            let height = (y2 as isize - y1 as isize).abs() + 1;
            let area = (width * height) as usize;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

// --- Part Two ---
// The Elves just remembered: they can only switch out tiles that are red or green. So, your rectangle can only include red or green tiles.
//
// In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.
//
// Using the same example as before, the tiles marked X would be green:
//
// ..............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............
// In addition, all of the tiles inside this loop of red and green tiles are also green. So, in this example, these are the green tiles:
//
// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............
// The remaining tiles are never red nor green.
//
// The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.
//
// For example, you could make a rectangle out of red and green tiles with an area of 15 between 7,3 and 11,1:
//
// ..............
// .......OOOOO..
// .......OOOOO..
// ..#XXXXOOOOO..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............
// Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:
//
// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXXOXX..
// .........OXX..
// .........OX#..
// ..............
// The largest rectangle you can make in this example using only red and green tiles has area 24. One way to do this is between 9,5 and 2,3:
//
// ..............
// .......#XXX#..
// .......XXXXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// .........XXX..
// .........#X#..
// ..............
// Using two red tiles as opposite corners, what is the largest area of any rectangle you can make using only red and green tiles?

fn solve_problem2(tiles: &Vec<(usize, usize)>) -> usize {
    // Build edge set
    let mut edge = HashSet::new();
    
    for i in 0..tiles.len() {
        let (ax, ay) = tiles[i];
        let (bx, by) = tiles[(i + 1) % tiles.len()];
        
        let dx = signum(bx as isize - ax as isize);
        let dy = signum(by as isize - ay as isize);
        
        let mut current = (ax, ay);
        while current != (bx, by) {
            edge.insert(current);
            current = ((current.0 as isize + dx) as usize, (current.1 as isize + dy) as usize);
        }
    }
    edge.insert(tiles[0]); // Ensure the last point is included
    
    // Build lines (pairs of consecutive points)
    let mut lines = Vec::new();
    for i in 0..tiles.len() {
        lines.push((tiles[i], tiles[(i + 1) % tiles.len()]));
    }
    
    // Find normals for each line
    let normals = find_normals(&lines);
    
    let mut max_area = 0;
    
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let rect_a = tiles[i];
            let rect_b = tiles[j];
            
            if rect_a.0 == rect_b.0 || rect_a.1 == rect_b.1 {
                continue;
            }
            
            let area = rect_area(rect_a, rect_b);
            if area <= max_area {
                continue;
            }
            
            if !is_rect_outside(rect_a, rect_b, &lines, &edge, &normals) {
                max_area = area;
            }
        }
    }
    
    max_area
}

fn signum(x: isize) -> isize {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

fn rect_area(a: (usize, usize), b: (usize, usize)) -> usize {
    let width = (a.0 as isize - b.0 as isize).abs() + 1;
    let height = (a.1 as isize - b.1 as isize).abs() + 1;
    (width * height) as usize
}

fn get_angle(p1: (usize, usize), p2: (usize, usize)) -> i32 {
    if p1.0 == p2.0 {
        // vertical
        if p1.1 > p2.1 { return 0; }
        if p1.1 < p2.1 { return 180; }
    } else if p1.1 == p2.1 {
        // horizontal
        if p1.0 > p2.0 { return 270; }
        if p1.0 < p2.0 { return 90; }
    }
    -1
}

fn find_normals(lines: &Vec<((usize, usize), (usize, usize))>) -> Vec<(isize, isize)> {
    let norm360 = |a: i32| -> i32 { ((a % 360) + 360) % 360 };
    let signed_delta = |from: i32, to: i32| -> i32 {
        let mut d = norm360(to - from);
        if d > 180 { d -= 360; }
        d
    };
    
    let rotations: Vec<i32> = lines.iter().map(|&(p1, p2)| get_angle(p1, p2)).collect();
    
    let mut total_rotation = 0;
    for i in 1..rotations.len() {
        total_rotation += signed_delta(rotations[i - 1], rotations[i]);
    }
    if !rotations.is_empty() {
        total_rotation += signed_delta(rotations[rotations.len() - 1], rotations[0]);
    }
    
    let normal_diff = if total_rotation > 0 { -90 } else { 90 };
    
    rotations.iter().map(|&rotation| {
        let normal_angle = norm360(rotation + normal_diff);
        match normal_angle {
            0 => (0, -1),
            90 => (1, 0),
            180 => (0, 1),
            270 => (-1, 0),
            _ => (0, 0),
        }
    }).collect()
}

fn is_rect_outside(
    rect_a: (usize, usize),
    rect_b: (usize, usize),
    lines: &Vec<((usize, usize), (usize, usize))>,
    edge: &HashSet<(usize, usize)>,
    normals: &Vec<(isize, isize)>
) -> bool {
    for (li, &(line_a, line_b)) in lines.iter().enumerate() {
        let normal_dir = normals[li];
        
        if is_intersecting(line_a, line_b, rect_a, rect_b) {
            // Check points on a line just outside the current line
            let n_line_a = ((line_a.0 as isize + normal_dir.0) as usize, 
                           (line_a.1 as isize + normal_dir.1) as usize);
            let n_line_b = ((line_b.0 as isize + normal_dir.0) as usize, 
                           (line_b.1 as isize + normal_dir.1) as usize);
            
            let line_dir = (
                signum(n_line_b.0 as isize - n_line_a.0 as isize),
                signum(n_line_b.1 as isize - n_line_a.1 as isize)
            );
            
            let mut p = n_line_a;
            while p != n_line_b {
                if contains(rect_a, rect_b, p) && !edge.contains(&p) {
                    return true;
                }
                p = ((p.0 as isize + line_dir.0) as usize, 
                     (p.1 as isize + line_dir.1) as usize);
            }
        }
    }
    false
}

fn contains(rect_a: (usize, usize), rect_b: (usize, usize), p: (usize, usize)) -> bool {
    let ax = rect_a.0.min(rect_b.0);
    let bx = rect_a.0.max(rect_b.0);
    let ay = rect_a.1.min(rect_b.1);
    let by = rect_a.1.max(rect_b.1);
    p.0 >= ax && p.0 <= bx && p.1 >= ay && p.1 <= by
}

fn is_intersecting(
    line_a: (usize, usize),
    line_b: (usize, usize),
    rect_a: (usize, usize),
    rect_b: (usize, usize)
) -> bool {
    let lax = line_a.0.min(line_b.0);
    let lbx = line_a.0.max(line_b.0);
    let rax = rect_a.0.min(rect_b.0);
    let rbx = rect_a.0.max(rect_b.0);
    
    let lay = line_a.1.min(line_b.1);
    let lby = line_a.1.max(line_b.1);
    let ray = rect_a.1.min(rect_b.1);
    let rby = rect_a.1.max(rect_b.1);
    
    if lay == lby && lay >= ray && lay <= rby {
        // horizontal line
        (lax <= rax && lbx >= rax) || (lax <= rbx && lbx >= rbx) || (lax >= rax && lbx <= rbx)
    } else if lax == lbx && lax >= rax && lax <= rbx {
        // vertical line
        (lay <= ray && lby >= ray) || (lay <= rby && lby >= rby) || (lay >= ray && lby <= rby)
    } else {
        false
    }
}
