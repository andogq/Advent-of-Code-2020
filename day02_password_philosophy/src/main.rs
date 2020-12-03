use std::fs;
use std::cmp::Ordering;

const FILE_NAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_NAME).expect("Problem reading file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut part_a_valid = 0;
    let mut part_b_valid = 0;
    
    for line in &contents {
        let line: Vec<&str> = line.split(": ").collect();

        let policy: Vec<&str> = line[0].split(" ").collect();
        let character = policy[1];

        let range: Vec<&str> = policy[0].split("-").collect();
        let min: usize = range[0].parse().expect("Problem parsing number");
        let max: usize = range[1].parse().expect("Problem parsing number");

        let password = line[1];

        /*
         * Part A
         */
        let count = password.matches(character).count();

        match count.cmp(&min) {
            Ordering::Less => {},
            _ => match count.cmp(&max) {
                Ordering::Greater => {},
                _ => part_a_valid += 1
            }
        }

        /*
         * Part B
         */

        let min = min - 1;
        let max = max - 1;

        match (min.cmp(&password.len()), max.cmp(&password.len())) {
            (Ordering::Less, Ordering::Less) | (Ordering::Less, Ordering::Equal) => {
                let char_a = password.chars().nth(min).unwrap();
                let char_b = password.chars().nth(max).unwrap();
        
                let character = character.chars().nth(0).unwrap();
        
                match char_a.cmp(&character) {
                    Ordering::Equal => match char_b.cmp(&character) {
                        Ordering::Equal => continue,
                        _ => part_b_valid += 1
                    },
                    _ => match char_b.cmp(&character) {
                        Ordering::Equal => part_b_valid += 1,
                        _ => continue
                    }
                }
            },
            _ => continue
        }
    }

    println!("There were a total of {} part A and {} part B valid passwords", part_a_valid, part_b_valid);
}
