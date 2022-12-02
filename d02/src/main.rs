use std::fs::read_to_string;

fn main() {
    // Read the input
    let input = read_to_string("input").unwrap();

    // Total score for the first round
    let mut first = 0usize;

    // Total score for the second round
    let mut second = 0usize;

    // Go through each line
    for line in input.lines() {
        // Get the shapes in their numeral formats
        let their = line.bytes().nth(0).unwrap() - b'A' + 1;
        let mine  = line.bytes().nth(2).unwrap() - b'X' + 1;

        // Closure to calculate the result of this round
        let round_result = |mine, their| -> usize {
            (mine + (((4 + mine - their) % 3) * 3)) as usize
        };

        // Increment the total for the first round
        first += round_result(mine, their);

        // Calculate what shape we should pick and then increment the total...
        second += round_result(((their + mine) % 3) + 1, their);
    }
    println!("{first} {second}");
}
