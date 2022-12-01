use std::fs::read_to_string;

/// Solution to the first part of the advent
fn first_part(elves: &Vec<usize>) -> usize {
    // Return the most calories
    return *elves.iter().reduce(|max, cal| {
        if cal > max { cal } else { max }
    }).unwrap();
}

/// Solution to the second part of the advent
fn second_part(elves: &mut Vec<usize>) -> usize {
    // Sort the elves and return the sum of the three highest
    elves.sort_unstable();
    return elves[elves.len()-3..].into_iter().sum();
}

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

    // First part
    println!("{}", first_part(&elves));

    // Second part
    println!("{}", second_part(&mut elves));
}
