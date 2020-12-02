use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<&str> = contents.split("\n").collect();

    let mut part_one = 0;
    let mut part_two = 0;

    for num_a in &numbers {
        let num_a: u32 = num_a.parse().expect("");

        for num_b in &numbers {
            let num_b: u32 = num_b.parse().expect("");

            for num_c in &numbers {
                let num_c: u32 = num_c.parse().expect("");
                match num_a + num_b + num_c {
                    2020 => part_two = num_a * num_b * num_c,
                    _ => continue
                }
            }

            match num_a + num_b {
                2020 => part_one = num_a * num_b,
                _ => continue
            }
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
