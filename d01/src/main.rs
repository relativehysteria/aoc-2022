use std::fs::read_to_string;

fn main() {
    // Read the input
    let input = read_to_string("input").unwrap();

    // Create a vector tracking the total amount of cal for each elf
    let mut elves: Vec<usize> = vec![0, 0];

    // Current elf (index into `elves`)
    let mut current = 1;

    // Go through each line
    for line in input.lines() {
        // If we have an empty line, go to the next elf
        if line == "" {
            // Create an entry for another elf
            elves.push(0);
            current += 1;
            continue;
        }

        // Add the calories to the current elf's total
        elves[current] += line.parse::<usize>().unwrap();
    }

    // Sort the calories ascendingly
    elves.sort_unstable();

    // First part
    println!("{}", elves[elves.len()-1]);

    // Second part
    println!("{}", elves[elves.len()-3..].iter().sum::<usize>());
}
