use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading input file");

    let mut part_a_count = 0;
    let mut part_b_count = 0;

    for line in input.split("\n\n") {
        let mut used: HashMap<char, usize> = HashMap::new();
        let mut group_size = 1;

        for c in line.chars() {
            if c == '\n' {
                group_size += 1;
            } else {
                if !used.contains_key(&c) {
                    used.insert(c, 1);
                } else {
                    *used.get_mut(&c).expect("Missing Key") += 1;
                }
            }
        }

        part_a_count += used.len();

        for (_, count) in used {
            if count == group_size {
                part_b_count += 1;
            }
        }
    }

    println!("Part A: {}, Part B: {}", part_a_count, part_b_count);
}
