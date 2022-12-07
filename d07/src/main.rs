use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::read_to_string;

// No VecDeque because no range indexing :(

fn main() {
    // Read the input
    let input = read_to_string("input").unwrap();

    // The stack of directories we are currently CD'd into
    let mut current_dir = Vec::new();

    // A map of all the directories and their total sizes
    let mut dir_sizes = HashMap::new();

    // Go through each line
    for line in input.lines() {
        // These lines are of no use to us
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        // Go back to the parent directory
        if line.starts_with("$ cd ..") {
            current_dir.pop();
            continue;
        }

        // Go to the root directory
        if line.starts_with("$ cd /") {
            current_dir.clear();
            continue;
        }

        // Go to the specified directory
        if line.starts_with("$ cd ") {
            current_dir.push(&line[5..]);
            continue;
        }

        // We have a file and its filesize. Get the filesize.
        let size = line.split(' ').next().unwrap().parse::<usize>().unwrap();

        // Add the filesize to the root directory
        *dir_sizes.entry(PathBuf::from("/")).or_insert(0) += size;

        // Add the filesize to every directory recursively
        for i in 0..current_dir.len() {
            let path = PathBuf::from_iter(&current_dir[..=i]);
            *dir_sizes.entry(path).or_insert(0) += size;
        }
    }

    // Part 1
    let p1 = dir_sizes.clone().into_values()
        .filter(|x| *x <= 100_000)
        .sum::<usize>();

    // Part 2
    let free   = 70_000_000;
    let needed = 30_000_000;
    let root   = dir_sizes.get(&PathBuf::from("/")).unwrap().clone();
    let p2 = dir_sizes.into_values().filter(|x| free - root + x >= needed).min()
        .unwrap();

    println!("{p1} {p2}");
}
