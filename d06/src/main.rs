use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    // Read the input as a vector of chars (omitting the newline at the end)
    let input = read_to_string("input").unwrap();
    let input = input[..input.len()-1].chars().collect::<Vec<char>>();

    // Checks if all characters in a `window` of `window_size` are different
    let is_unique = |window: &[char]| {
        HashSet::<&char>::from_iter::<&[char]>(window).len() == window.len()
    };

    // Window size for part 1
    let part_1_window_size = 4;

    // Window size for part 2
    let part_2_window_size = 14;

    // Go through all the windows and find the one where all characters differ
    let part1 = input.windows(part_1_window_size).enumerate()
        .find(|(_, c)| { is_unique(*c) })
        .unwrap().0 + part_1_window_size;

    let part2 = input.windows(part_2_window_size).enumerate()
        .find(|(_, c)| { is_unique(*c) })
        .unwrap().0 + part_2_window_size;

    println!("{part1} {part2}");
}
