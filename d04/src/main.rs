use std::fs::read_to_string;

fn main() {
    // Read the input
    let input = read_to_string("input").unwrap();

    // Total number of contained and overlapping sections
    let mut total_contained   = 0;
    let mut total_overlapping = 0;

    // Checks whther `a` is fully contained by `b`
    let contained = |a: &Vec<usize>, b: &Vec<usize>| -> bool {
        a[0] >= b[0] && a[1] <= b[1]
    };

    // Checks whether there is any overlap between `a` and `b`
    let overlaps = |a: &Vec<usize>, b: &Vec<usize>| -> bool {
        a[0] <= b[1] && b[0] <= a[1]
    };

    // Go through each line
    for line in input.lines() {
        // Split the line and separate our sections
        let elves  = line.split(',').collect::<Vec<&str>>();

        // Parse the sections as ranges but not really
        let first  = elves[0].split('-')
            .map(|x| usize::from_str_radix(x, 10).unwrap())
            .collect::<Vec<usize>>();
        let second = elves[1].split('-')
            .map(|x| usize::from_str_radix(x, 10).unwrap())
            .collect::<Vec<usize>>();

        // Check if either of them is fully contained by the other
        if contained(&first, &second) || contained(&second, &first) {
            total_contained += 1;
        }

        if overlaps(&first, &second) {
            total_overlapping += 1;
        }
    }
    println!("{total_contained} {total_overlapping}");
}
