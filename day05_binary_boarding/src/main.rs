use std::fs;

struct Range {
    min: u8,
    max: u8
}

impl Range {
    fn new(min: u8, max: u8) -> Range {
        Range {
            min,
            max
        }
    }

    fn split(&mut self, direction: char) {
        let mut min = self.min;
        let mut max = self.max;

        let difference = max - min;

        if direction == 'F' || direction == 'L' {
            max = min + (difference / 2);
        } else if direction == 'B' || direction == 'R' {
            min = min + (difference / 2) + 1;
        }

        self.min = min;
        self.max = max;

        // println!("min: {}, max: {}", self.min, self.max);
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Problem reading input file");

    let mut max_seat_id = 0;
    let mut seat_ids: Vec<u16> = Vec::new();

    for line in contents.split_ascii_whitespace() {
        let mut line = line.chars();

        let mut row = Range::new(0, 127);
        for _ in 0..7 {
            row.split(line.next().expect("Invalid specifier"));
        }

        let mut column = Range::new(0, 7);
        for _ in 0..3 {
            column.split(line.next().expect("Invalid specifier"));
        }

        let seat_id: u16 = (row.min as u16) * 8 + (column.min as u16);

        if max_seat_id < seat_id {
            max_seat_id = seat_id;
        }

        seat_ids.push(seat_id);
    }

    let mut missing: Vec<u16> = Vec::new();

    for id in &seat_ids {
        let mut found = 0;

        for id_1 in &seat_ids {
            if id_1 - 1 == *id {
                found += 1;
            } else if id_1 + 1 == *id {
                found += 1;
            }

            if found == 2 {
                break;
            }
        }

        if found != 2 {
            missing.push(*id);
        }
    }

    let mut part_b = 0;

    for num in &missing {
        for num_test in &missing {
            if num + 2 == *num_test {
                part_b = num - 1;
            } else if num - 2 == *num_test {
                part_b = num + 1;
            }
        }
    }


    println!("Part A: {}. Part B: {}", max_seat_id, part_b);
}
