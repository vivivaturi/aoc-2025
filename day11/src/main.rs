// --- Day 11: Reactor ---
// You hear some loud beeping coming from a hatch in the floor of the factory, so you decide to check it out. Inside, you find several large electrical conduits and a ladder.
//
// Climbing down the ladder, you discover the source of the beeping: a large, toroidal reactor which powers the factory above. Some Elves here are hurriedly running between the reactor and a nearby server rack, apparently trying to fix something.
//
// One of the Elves notices you and rushes over. "It's a good thing you're here! We just installed a new server rack, but we aren't having any luck getting the reactor to communicate with it!" You glance around the room and see a tangle of cables and devices running from the server rack to the reactor. She rushes off, returning a moment later with a list of the devices and their outputs (your puzzle input).
//
// For example:
//
// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out
// Each line gives the name of a device followed by a list of the devices to which its outputs are attached. So, bbb: ddd eee means that device bbb has two outputs, one leading to device ddd and the other leading to device eee.
//
// The Elves are pretty sure that the issue isn't due to any specific device, but rather that the issue is triggered by data following some specific path through the devices. Data only ever flows from a device through its outputs; it can't flow backwards.
//
// After dividing up the work, the Elves would like you to focus on the devices starting with the one next to you (an Elf hastily attaches a label which just says you) and ending with the main output to the reactor (which is the device with the label out).
//
// To help the Elves figure out which path is causing the issue, they need you to find every path from you to out.
//
// In this example, these are all of the paths from you to out:
//
// Data could take the connection from you to bbb, then from bbb to ddd, then from ddd to ggg, then from ggg to out.
// Data could take the connection to bbb, then to eee, then to out.
// Data could go to ccc, then ddd, then ggg, then out.
// Data could go to ccc, then eee, then out.
// Data could go to ccc, then fff, then out.
// In total, there are 5 different paths leading from you to out.
//
// How many different paths lead from you to out?

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let graph = load_input(&input);

    // Optimization: Find all nodes that can reach "out"
    let reachable = find_nodes_reaching_target(&graph, "out");
    let unreachable_count =
        graph.len() - reachable.iter().filter(|n| graph.contains_key(*n)).count();
    println!(
        "Debug: {} nodes can reach 'out', {} nodes are dead ends",
        reachable.iter().filter(|n| graph.contains_key(*n)).count(),
        unreachable_count
    );

    // Part 1: Count all paths from "you" to "out"
    let mut cache = std::collections::HashMap::new();
    let path_count = count_paths(&graph, "you", "out", &mut cache, &reachable);

    println!(
        "Part 1: Number of paths from 'you' to 'out': {}",
        path_count
    );
    
    // Part 2: Count paths from "svr" to "out" that visit both "dac" and "fft"
    let part2_count = count_paths_through_both(&graph, "svr", "out", "dac", "fft", &reachable);
    println!(
        "Part 2: Paths from 'svr' to 'out' visiting both 'dac' and 'fft': {}",
        part2_count
    );
}

fn load_input(input_str: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut graph = std::collections::HashMap::new();
    for line in input_str.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();
        let edges: Vec<String> = if parts.len() > 1 {
            parts[1].split_whitespace().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        };
        graph.insert(node, edges);
    }
    graph
}

// Build reverse graph to find which nodes can reach the target
// This allows us to prune paths that lead nowhere
fn find_nodes_reaching_target(
    graph: &std::collections::HashMap<String, Vec<String>>,
    target: &str,
) -> std::collections::HashSet<String> {
    use std::collections::{HashSet, VecDeque};

    // Build reverse graph (node -> nodes that point to it)
    let mut reverse_graph: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for (node, neighbors) in graph {
        for neighbor in neighbors {
            reverse_graph
                .entry(neighbor.clone())
                .or_insert_with(Vec::new)
                .push(node.clone());
        }
    }

    // BFS backwards from target to find all nodes that can reach it
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(target.to_string());
    reachable.insert(target.to_string());

    while let Some(current) = queue.pop_front() {
        if let Some(predecessors) = reverse_graph.get(&current) {
            for pred in predecessors {
                if reachable.insert(pred.clone()) {
                    queue.push_back(pred.clone());
                }
            }
        }
    }

    reachable
}

// Count all paths from start to target using memoization
// The cache stores: for each node, how many paths exist from that node to the target
fn count_paths<'a>(
    graph: &'a std::collections::HashMap<String, Vec<String>>,
    current: &'a str,
    target: &str,
    cache: &mut std::collections::HashMap<&'a str, usize>,
    reachable: &std::collections::HashSet<String>,
) -> usize {
    // Base case: we reached the target
    if current == target {
        return 1;
    }

    // Early exit: if this node can't reach the target, skip it
    if !reachable.contains(current) {
        return 0;
    }

    // Check if we've already computed paths from this node
    if let Some(&cached_count) = cache.get(current) {
        return cached_count;
    }

    // Get neighbors of current node
    let neighbors = match graph.get(current) {
        Some(n) => n,
        None => {
            // Dead end - cache it as 0 paths
            cache.insert(current, 0);
            return 0;
        }
    };

    // Count paths through each neighbor (only those that can reach target)
    let mut total_paths = 0;
    for neighbor in neighbors {
        // Skip neighbors that can't reach the target
        if !reachable.contains(neighbor.as_str()) {
            continue;
        }
        total_paths += count_paths(graph, neighbor.as_str(), target, cache, reachable);
    }

    // Cache the result for this node
    cache.insert(current, total_paths);

    total_paths
}

// Count paths from start to end that visit both waypoint1 and waypoint2 (in any order)
// This counts: start -> waypoint1 -> waypoint2 -> end AND start -> waypoint2 -> waypoint1 -> end
fn count_paths_through_both(
    graph: &std::collections::HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    waypoint1: &str,
    waypoint2: &str,
    reachable: &std::collections::HashSet<String>,
) -> usize {
    // Path 1: start -> waypoint1 -> waypoint2 -> end
    let mut cache1 = std::collections::HashMap::new();
    let paths_start_to_w1 = count_paths(graph, start, waypoint1, &mut cache1, reachable);
    
    let mut cache2 = std::collections::HashMap::new();
    let paths_w1_to_w2 = count_paths(graph, waypoint1, waypoint2, &mut cache2, reachable);
    
    let mut cache3 = std::collections::HashMap::new();
    let paths_w2_to_end = count_paths(graph, waypoint2, end, &mut cache3, reachable);
    
    let route1 = paths_start_to_w1 * paths_w1_to_w2 * paths_w2_to_end;
    
    // Path 2: start -> waypoint2 -> waypoint1 -> end
    let mut cache4 = std::collections::HashMap::new();
    let paths_start_to_w2 = count_paths(graph, start, waypoint2, &mut cache4, reachable);
    
    let mut cache5 = std::collections::HashMap::new();
    let paths_w2_to_w1 = count_paths(graph, waypoint2, waypoint1, &mut cache5, reachable);
    
    let mut cache6 = std::collections::HashMap::new();
    let paths_w1_to_end = count_paths(graph, waypoint1, end, &mut cache6, reachable);
    
    let route2 = paths_start_to_w2 * paths_w2_to_w1 * paths_w1_to_end;
    
    route1 + route2
}

// --- Part Two ---
// Thanks in part to your analysis, the Elves have figured out a little bit about the issue. They now know that the problematic data path passes through both dac (a digital-to-analog converter) and fft (a device which performs a fast Fourier transform).
//
// They're still not sure which specific path is the problem, and so they now need you to find every path from svr (the server rack) to out. However, the paths you find must all also visit both dac and fft (in any order).
//
// For example:
//
// svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out
// This new list of devices contains many paths from svr to out:
//
// svr,aaa,fft,ccc,ddd,hub,fff,ggg,out
// svr,aaa,fft,ccc,ddd,hub,fff,hhh,out
// svr,aaa,fft,ccc,eee,dac,fff,ggg,out
// svr,aaa,fft,ccc,eee,dac,fff,hhh,out
// svr,bbb,tty,ccc,ddd,hub,fff,ggg,out
// svr,bbb,tty,ccc,ddd,hub,fff,hhh,out
// svr,bbb,tty,ccc,eee,dac,fff,ggg,out
// svr,bbb,tty,ccc,eee,dac,fff,hhh,out
// However, only 2 paths from svr to out visit both dac and fft.
//
// Find all of the paths that lead from svr to out. How many of those paths visit both dac and fft?
