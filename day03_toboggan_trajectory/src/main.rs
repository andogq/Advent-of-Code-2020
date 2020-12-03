use std::fs;

const INPUT_FILE: &str = "input.txt";
const TREE: char = '#';

fn main() {
    let map = fs::read_to_string(INPUT_FILE).expect("Problem reading file");

    let map: Vec<&str> = map.split("\n").collect();

    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut positions = [0; 5];
    let mut tree_counts = [0; 5];
    let mut distance_travelled = 0;

    let mut tree_count = 0;
    let mut x = 0;
    
    for line in &map {
        let mut spaces: Vec<char> = Vec::new();
        for c in line.chars() {
            spaces.push(c);
        }

        // Part 1
        if spaces[x] == TREE {
            tree_count += 1;
        }
        x += 3;
        x %= map[0].len();

        // Part 2
        let mut current_slope = 0;
        for slope in slopes.iter() {
            if distance_travelled % slope[1] == 0 {
                if spaces[positions[current_slope]] == TREE {
                    tree_counts[current_slope] += 1;
                }

                positions[current_slope] += slope[0];
                positions[current_slope] %= map[0].len();
                current_slope += 1;
            }
        }

        distance_travelled += 1;
    }

    let mut part_two_result: u64 = 1;
    println!("{:?}", tree_counts);
    for count in tree_counts.iter() {
        part_two_result *= count;
    }

    println!("{} trees were encountered in part one", tree_count);
    println!("{} trees were encountered in part two", part_two_result);
}
