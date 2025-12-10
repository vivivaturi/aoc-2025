fn main() {
    // Example 1: should be 10
    let ex1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    // Buttons: (3), (1,3), (2), (2,3), (0,2), (0,1)
    // Targets: counter 0=3, 1=5, 2=4, 3=7
    // Solution: press (3) once, (1,3) 3 times, (2,3) 3 times, (0,2) once, (0,1) twice = 10
    
    println!("Example 1:");
    println!("Button (3): affects counter 3");
    println!("Button (1,3): affects counters 1 and 3");
    println!("Button (2): affects counter 2");
    println!("Button (2,3): affects counters 2 and 3");
    println!("Button (0,2): affects counters 0 and 2");
    println!("Button (0,1): affects counters 0 and 1");
    println!();
    println!("Target: [3, 5, 4, 7]");
    println!();
    println!("Solution:");
    println!("Press (3) 1x:     [0, 0, 0, 1]");
    println!("Press (1,3) 3x:   [0, 3, 0, 4]");
    println!("Press (2,3) 3x:   [0, 3, 3, 7]");
    println!("Press (0,2) 1x:   [1, 3, 4, 7]");
    println!("Press (0,1) 2x:   [3, 5, 4, 7]");
    println!("Total: 1+3+3+1+2 = 10");
}
