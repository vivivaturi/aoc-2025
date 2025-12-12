use std::fs;

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required_pieces: Vec<usize>, // count of each shape type needed
}

#[derive(Debug, PartialEq)]
enum Classification {
    No,
    Definitely,
    Maybe,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");
    
    // Parse shapes and regions
    let (shape_strings, regions) = parse_input(&input);
    
    // Calculate density (number of # in each shape)
    let densities: Vec<usize> = shape_strings
        .iter()
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .collect();
    
    println!("Parsed {} shapes and {} regions", densities.len(), regions.len());
    
    // Classify regions
    let mut no_count = 0;
    let mut definitely_count = 0;
    let mut maybe_count = 0;
    
    for (idx, region) in regions.iter().enumerate() {
        let classification = classify_region(region, &densities);
        
        match classification {
            Classification::No => {
                no_count += 1;
                println!("Region {}: NO (impossible)", idx + 1);
            }
            Classification::Definitely => {
                definitely_count += 1;
                println!("Region {}: DEFINITELY (easy fit)", idx + 1);
            }
            Classification::Maybe => {
                maybe_count += 1;
                println!("Region {}: MAYBE (needs checking)", idx + 1);
            }
        }
    }
    
    println!("\n=== RESULTS ===");
    println!("NO (impossible): {}", no_count);
    println!("Definitely (can fit): {}", definitely_count);
    println!("Maybe (needs testing): {}", maybe_count);
    println!("\nPart 1 answer (Definitely + Maybe): {}", definitely_count + maybe_count);
}

fn classify_region(region: &Region, densities: &[usize]) -> Classification {
    // Calculate minimum space needed
    let min_space: usize = region
        .required_pieces
        .iter()
        .zip(densities.iter())
        .map(|(&count, &density)| count * density)
        .sum();
    
    let total_area = region.width * region.height;
    
    // Check if impossible (not enough space)
    if min_space > total_area {
        return Classification::No;
    }
    
    // Calculate total number of presents
    let total_presents: usize = region.required_pieces.iter().sum();
    
    // If each present can fit in its own 3x3 grid, it's definitely possible
    if total_presents <= (region.width / 3) * (region.height / 3) {
        return Classification::Definitely;
    }
    
    // Otherwise, we'd need to actually test it
    Classification::Maybe
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Region>) {
    // Split by double newlines to get groups
    let groups: Vec<&str> = input.split("\n\n").collect();
    
    // All groups except last are presents (shapes)
    let presents: Vec<String> = groups[..groups.len() - 1]
        .iter()
        .map(|g| g.to_string())
        .collect();
    
    // Last group contains regions
    let region_lines = groups[groups.len() - 1].lines();
    let mut regions = Vec::new();
    
    for line in region_lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let dims: Vec<&str> = parts[0].trim().split('x').collect();
            if dims.len() == 2 {
                let width = dims[0].parse().unwrap();
                let height = dims[1].parse().unwrap();
                let counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                
                regions.push(Region {
                    width,
                    height,
                    required_pieces: counts,
                });
            }
        }
    }
    
    (presents, regions)
}

