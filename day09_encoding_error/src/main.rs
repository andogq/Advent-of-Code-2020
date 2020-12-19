use std::fs;

const PREAMBLE_LENGTH: usize = 25;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Problem reading input file");
    let mut numbers: Vec<usize> = Vec::new();
    for n in file.split("\n") {
        if n != "" {
            numbers.push(n.parse().unwrap());
        }
    }

    let mut goal = 0;
    let mut index = PREAMBLE_LENGTH;
    while index < numbers.len() {
        let n = numbers[index];
        let mut valid = false;

        for i in (index - PREAMBLE_LENGTH)..index {
            for j in (index - PREAMBLE_LENGTH)..index {
                if numbers[i] + numbers[j] == n {
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }

        if !valid {
            println!("Part A: {}", n);
            goal = n;
            break;
        } else {
            index += 1;
        }
    }

    let mut min_i = 0;
    let mut max_i = 0;
    let mut sum = 0;

    let mut smallest = numbers[0];
    let mut largest = numbers[0];

    while sum != goal {
        if sum < goal {
            max_i += 1;
        } else {
            min_i += 1;
        }

        sum = 0;
        smallest = numbers[min_i];
        largest = numbers[min_i];
        for i in min_i..(max_i + 1) {
            sum += numbers[i];

            if numbers[i] < smallest {
                smallest = numbers[i];
            } else if numbers[i] > largest {
                largest = numbers[i];
            }
        }
    }

    println!("Part B: {}", smallest + largest);
}
