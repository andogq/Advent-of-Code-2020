use std::fs;
use regex::Regex;

const FILE_NAME: &str = "input.txt";
const BAG: &str = "shiny gold";

fn main() {
    let input = fs::read_to_string(FILE_NAME).expect("Problem reading input file");

    let re_rule = Regex::new(r"^([\w ]+) bags contain (.+)$").unwrap();
    let re_child_bag = Regex::new(r"(\d+) ([\w ]+) bags?").unwrap();

    let mut part_a_target_bags: Vec<String> = Vec::new();
    part_a_target_bags.push(BAG.to_string());
    
    let mut part_a_bags: Vec<String> = Vec::new();

    while part_a_target_bags.len() > 0 {
        let target_bag = part_a_target_bags.remove(0);

        for line in input.split("\n") {
            if let Some(index) = line.find(&target_bag) {
                if index != 0 {
                    if let Some(captures) = re_rule.captures(line) {
                        let found_bag = captures[1].to_string();
                        if !part_a_bags.contains(&found_bag) {
                            part_a_bags.push(found_bag.clone());

                            if !part_a_target_bags.contains(&found_bag) {
                                part_a_target_bags.push(found_bag.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut part_b_target_bags: Vec<(String, usize)> = Vec::new();
    part_b_target_bags.push((BAG.to_string(), 1));

    let mut part_b_count = 0;

    while part_b_target_bags.len() > 0 {
        let (target_bag, multiplier) = part_b_target_bags.remove(0);

        for line in input.split("\n") {
            if let Some(index) = line.find(&target_bag) {
                if index == 0 {
                    if let Some(captures) = re_rule.captures(line) {
                        for child_bag in re_child_bag.captures_iter(&captures[2]) {
                            let child_bag_count: usize = child_bag[1].parse().unwrap();
                            part_b_count += multiplier * child_bag_count;
                            part_b_target_bags.push((child_bag[2].to_string(), multiplier * child_bag_count));
                        }
                    }
                }
            }
        }
    }

    println!("Part A: {}", part_a_bags.len());
    println!("Part B: {}", part_b_count);
}