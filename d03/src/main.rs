use std::fs::read_to_string;

fn main() {
    // Read the input
    let input = read_to_string("input").unwrap();

    // Turn each line into two compartments
    let bags = input.lines()
        // Turn the characters into bytes
        .map(str::as_bytes)
        .filter_map(|line| {
            // Split the line into two compartments
            let (a, b) = line.split_at(line.len()/2);

            a.iter()
                // Find the common letter
                .find(|&x| b.contains(x))
                // Turn it into its numeral form
                .map(|&x| (x as usize - 38) - 58 * ((x >= b'a') as usize))
        })
        .sum::<usize>();

    println!("{bags}");
}
