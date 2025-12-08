// --- Day 8: Playground ---
// Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.
//
// You rematerialize on an unfamiliar teleporter pad and find yourself in a vast underground space which contains a giant playground!
//
// Across the playground, a group of Elves are working on setting up an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes.
//
// Their plan is to connect the junction boxes with long strings of lights. Most of the junction boxes don't provide electricity; however, when two junction boxes are connected by a string of lights, electricity can pass between those two junction boxes.
//
// The Elves are trying to figure out which junction boxes to connect so that electricity can reach every junction box. They even have a list of all of the junction boxes' positions in 3D space (your puzzle input).
//
// For example:
//
// 162,817,812
// 57,618,57
// 906,360,560
// 592,479,940
// 352,342,300
// 466,668,158
// 542,29,236
// 431,825,988
// 739,650,466
// 52,470,668
// 216,146,977
// 819,987,18
// 117,168,530
// 805,96,715
// 346,949,466
// 970,615,88
// 941,993,340
// 862,61,35
// 984,92,344
// 425,690,689
// This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.
//
// To save on string lights, the Elves would like to focus on connecting pairs of junction boxes that are as close together as possible according to straight-line distance. In this example, the two junction boxes which are closest together are 162,817,812 and 425,690,689.
//
// By connecting these two junction boxes together, because electricity can flow between them, they become part of the same circuit. After connecting them, there is a single circuit which contains two junction boxes, and the remaining 18 junction boxes remain in their own individual circuits.
//
// Now, the two junction boxes which are closest together but aren't already directly connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812 is already connected to another junction box, there is now a single circuit which contains three junction boxes and an additional 17 circuits which contain one junction box each.
//
// The next two junction boxes to connect are 906,360,560 and 805,96,715. After connecting them, there is a circuit containing 3 junction boxes, a circuit containing 2 junction boxes, and 15 circuits which contain one junction box each.
//
// The next two junction boxes are 431,825,988 and 425,690,689. Because these two junction boxes were already in the same circuit, nothing happens!
//
// This process continues for a while, and the Elves are concerned that they don't have enough extension cables for all these circuits. They would like to know how big the circuits will be.
//
// After making the ten shortest connections, there are 11 circuits: one circuit which contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits which contain 2 junction boxes each, and seven circuits which each contain a single junction box. Multiplying together the sizes of the three largest circuits (5, 4, and one of the circuits of size 2) produces 40.
//
// Your list contains many junction boxes; connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits?
use std::collections::HashMap;
use std::fs;

fn main() {
    let boxes = load_input();
    let answer1 = solve_part1(&boxes);
    let answer2 = solve_part2(&boxes);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn load_input() -> Vec<(i32, i32, i32)> {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}

fn euclidean_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false; // Already in same set
        }

        self.parent[px] = py;
        true
    }
}

fn solve_part1(junction_boxes: &Vec<(i32, i32, i32)>) -> usize {
    let n = junction_boxes.len();

    // Generate all pairs with distances
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let distance = euclidean_distance(junction_boxes[i], junction_boxes[j]);
            pairs.push((distance, i, j));
        }
    }

    // Sort by distance
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Process the 1000 shortest pairs
    let mut uf = UnionFind::new(n);
    for (_, i, j) in pairs.iter().take(1000) {
        uf.union(*i, *j);
    }

    // Count circuit sizes
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    // Get the three largest circuit sizes
    let mut sizes: Vec<usize> = circuit_sizes.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    sizes[0] * sizes[1] * sizes[2]
}

// --- Part Two ---
// The Elves were right; they definitely don't have enough extension cables. You'll need to keep connecting junction boxes together until they're all in one large circuit.
//
// Continuing the above example, the first connection which causes all of the junction boxes to form a single circuit is between the junction boxes at 216,146,977 and 117,168,530. The Elves need to know how far those junction boxes are from the wall so they can pick the right extension cable; multiplying the X coordinates of those two junction boxes (216 and 117) produces 25272.
//
// Continue connecting the closest unconnected pairs of junction boxes together until they're all in the same circuit. What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect?

fn solve_part2(junction_boxes: &Vec<(i32, i32, i32)>) -> i64 {
    let n = junction_boxes.len();

    // Generate all pairs with distances
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let distance = euclidean_distance(junction_boxes[i], junction_boxes[j]);
            pairs.push((distance, i, j));
        }
    }

    // Sort by distance
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Process pairs until all are connected
    let mut uf = UnionFind::new(n);
    let mut last_pair: Option<(usize, usize)> = None;

    for (_, i, j) in pairs {
        if uf.union(i, j) {
            last_pair = Some((i, j));
            // Check if all are connected by seeing if the number of unique roots is 1
            let mut root_set = std::collections::HashSet::new();
            for k in 0..n {
                root_set.insert(uf.find(k));
            }
            if root_set.len() == 1 {
                break;
            }
        }
    }

    if let Some((i, j)) = last_pair {
        (junction_boxes[i].0 as i64) * (junction_boxes[j].0 as i64)
    } else {
        0
    }
}
