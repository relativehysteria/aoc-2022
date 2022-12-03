#![feature(array_chunks)]
// `array_chunks()` is used instead of `chunks_exact()` because it works :)

use std::fs::read_to_string;

fn main() {
    // Read the input, separate it by newlines
    let input = read_to_string("input").unwrap();
    let input = input.lines().map(str::as_bytes);

    // Letters are turned into their numeral priority form using this formula
    let prio = |x: &u8| -> usize {
        (*x as usize - 38) - 58 * ((*x >= b'a') as usize)
    };

    // Part 1
    {
        // Turn each bag into two compartments
        let priorities = input
            .clone()
            .filter_map(|line| {
                // Split the line into two compartments
                let (a, b) = line.split_at(line.len()/2);

                a.iter()
                    // Find the common letter
                    .find(|&x| b.contains(x))
                    // Turn it into its numeral form
                    .map(prio)
            })
            // Sum it up
            .sum::<usize>();

        println!("{priorities}");
    }

    // Part 2
    {
        let priorities = input
            .collect::<Vec<&[u8]>>()
            // Go through 3 lines at a time
            .array_chunks::<3>()
            .filter_map(|[a, b, c]| {
                a.iter()
                    // Find the common letter in all of them
                    .find(|&x| b.contains(x) && c.contains(x))
                    // Turn it into its numeral form
                    .map(prio)
            })
            // Sum it up
            .sum::<usize>();
        println!("{priorities}");
    }
}
