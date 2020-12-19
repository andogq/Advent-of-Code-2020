use std::fs;
use regex::Regex;

fn main() {
    let whole_file = fs::read_to_string("input.txt").expect("Problem reading input file");
    let mut file: Vec<String> = Vec::new();
    for line in whole_file.split("\n") {
        if line != "" {
            file.push(String::from(line));
        }
    }

    println!("Part A: {}", run_program(&file).0);

    let mut accumulator = 0;
    let mut finished = false;

    let mut next_test = 0;

    let re_instruction = Regex::new(r"^(\w{3}) (.+)").unwrap();

    while !finished {
        let mut test_program = file.clone();

        while &re_instruction.captures(&test_program[next_test]).unwrap()[1] == "acc" {
            next_test += 1;
        }

        let instruction = re_instruction.captures(&test_program[next_test]).unwrap();
        match &instruction[1] {
            "jmp" => {
                test_program[next_test] = test_program[next_test].replace("jmp", "nop");
            },
            "nop" => {
                test_program[next_test] = test_program[next_test].replace("nop", "jmp");
            },
            _ => continue
        }

        let result = run_program(&test_program);
        accumulator = result.0;
        finished = result.1;

        next_test += 1;
    }

    println!("Part B: {}", accumulator);
}

fn run_program(program: &Vec<String>) -> (isize, bool) {
    let re_line = Regex::new(r"^(?P<instruction>\w{3}) (?P<sign>[\+\-])(?P<number>\d+)").unwrap();

    let mut read_lines: Vec<usize> = Vec::new();
    let mut current_line = 0;

    let mut accumulator: isize = 0;

    while !read_lines.contains(&current_line) && current_line < program.len() {
        read_lines.push(current_line);

        let line = re_line.captures(&program[current_line]).unwrap();
        let instruction = line.name("instruction").unwrap().as_str();
        let sign = line.name("sign").unwrap().as_str();
        let number: usize = line.name("number").unwrap().as_str().parse().unwrap();

        match instruction {
            "acc" => {
                current_line += 1;
                match sign {
                    "+" => accumulator += number as isize,
                    "-" => accumulator -= number as isize,
                    _ => continue
                }
            },
            "jmp" => {
                match sign {
                    "+" => current_line += number,
                    "-" => current_line -= number,
                    _ => continue
                }
            },
            "nop" => current_line += 1,
            _ => continue
        }
    }

    (accumulator, current_line == program.len())
}