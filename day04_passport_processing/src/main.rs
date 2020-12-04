use std::fs;
use regex::Regex;

const FILE_NAME: &str = "input.txt";
const VALID_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let contents = fs::read_to_string(FILE_NAME).expect("Problem reading file");

    let mut valid_count_a = 0;
    let mut valid_count_b = 0;

    let hex_re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    let passports: Vec<&str> = contents.split("\n\n").collect();
    for passport in passports {
        let passport = passport.replace("\n", " ");
        
        // Part one
        let mut count = 0;
        for field in VALID_FIELDS.iter() {
            if passport.find(field).unwrap_or(999) < 999 {
                count += 1;
            }
        }
        if count == 7 {
            valid_count_a += 1;

            // Part two
            let mut valid = true;
            let passport: Vec<&str> = passport.split_ascii_whitespace().collect();
            for field in passport {
                let field: Vec<&str> = field.split(":").collect();
                let value = field[1];
                let field = field[0];
    
                if field == "byr" {
                    let value: u32 = value.parse().unwrap();
                    if value < 1920 || value > 2002 {
                        valid = false;
                    }
                } else if field == "iyr" {
                    let value: u32 = value.parse().unwrap();
                    if value < 2010 || value > 2020 {
                        valid = false;
                    }
                } else if field == "eyr" {
                    let value: u32 = value.parse().unwrap();
                    if value < 2020 || value > 2030 {
                        valid = false;
                    }
                } else if field == "hgt" {
                    if value.contains("cm") {
                        let value: u32 = value.replace("cm", "").parse().unwrap_or(0);
                        if value < 150 || value > 193 {
                            valid = false;
                        }
                    } else if value.contains("in") {
                        let value: u32 = value.replace("in", "").parse().unwrap_or(0);
                        if value < 59 || value > 76 {
                            valid = false;
                        }
                    } else {
                        valid = false;
                    }
                } else if field == "hcl" {
                    if !hex_re.is_match(value) {
                        valid = false;
                    }
                } else if field == "ecl" {
                    if !VALID_EYE_COLOR.contains(&value) {
                        valid = false;
                    }
                } else if field == "pid" {
                    if !pid_re.is_match(value) {
                        valid = false;
                    }
                }
            }
            if valid {
                valid_count_b += 1;
            }
        }

    }

    println!("There were {} valid passports in part one and {} in part two", valid_count_a, valid_count_b);
}
