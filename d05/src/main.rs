use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    // Read the file
    let input = read_to_string("input").unwrap();
    let input = input.lines().collect::<Vec<&str>>();

    // A vector of all the cargo crate stacks.
    // There can only really be 9 stacks before the parsing gets complicated...
    let mut cargo_stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    // Go through each line, make space for all stacks and push the cargo there
    for &line in input.iter() {
        // Once we reach the end of the cargo section, stop parsing
        if !line.contains('[') { break; }

        // Go through each space where a crate should be
        for (i, crg) in line[1..].chars().into_iter().step_by(4).enumerate() {
            // If the space is empty, go to the next one
            if crg == ' ' { continue; }

            // Push the crate to the bottom of its stack
            cargo_stacks[i].push_front(crg);
        }
    }

    // Parse the instructions. This is inefficient as we "try" to parse the
    // initial cargo section as well, but it doesn't matter.
    // Actually, I think using a deque is inefficient to begin with (but it
    // works).
    for &line in input.iter() {
        // Each instruction has to start with "move"
        if !line.starts_with("move") { continue; }

        // Parse the coordinates from the instruction. The instruction is always
        // `move <coordinate> from <coordinate> to <coordinate>`.
        let mut coords = line.split(' ').skip(1).step_by(2)
            .map(|x| x.parse::<usize>().unwrap());
        let count = coords.next().unwrap();
        let from  = coords.next().unwrap() - 1;
        let to    = coords.next().unwrap() - 1;


        // Do the moving around! :D

        // // Part 1
        // {
        //     for _ in 0..count {
        //         let cargo_crate = cargo_stacks[from].pop_back().unwrap();
        //         cargo_stacks[to].push_back(cargo_crate);
        //     }
        // }

        // Part 2
        // This one doesn't feel very elegant...
        {
            let len = cargo_stacks[from].len();
            let cargo_crates = cargo_stacks[from].drain(len - count..)
                .collect::<VecDeque<_>>();
            cargo_stacks[to].extend(cargo_crates);
        }
    }

    cargo_stacks.iter()
        .for_each(|i| print!("{}", i.iter().rev().next().unwrap_or(&'\0')));
    println!();
}
