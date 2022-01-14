use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::lib::read_lines;

fn deep_count(
    map: &HashMap<String, HashMap<String, usize>>,
    bag: &str,
    multiplier: usize,
) -> usize {
    let inner_bags = map.get(bag).unwrap();

    if inner_bags.is_empty() {
        return multiplier;
    }

    let mut result = multiplier;

    for (inner_bag, count) in inner_bags {
        result += deep_count(map, inner_bag, multiplier * count);
    }

    result
}

pub fn perform(part: u8) {
    let re1 = Regex::new(r"(\w+\s\w+) bags contain").unwrap(); // Outer Bag
    let re2 = Regex::new(r"(\d+) (\w+\s\w+) bags?[,\.]").unwrap(); // Inner Bag(s)

    if let Ok(lines) = read_lines("./test_inputs/day_7.txt") {
        match part {
            1 => {
                let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();

                for line in lines {
                    if let Ok(line) = line {
                        let parent = re1.captures(&line).unwrap().get(1).unwrap().as_str();

                        for cap in re2.captures_iter(&line) {
                            let child = cap.get(2).unwrap().as_str().to_string();

                            bag_map
                                .entry(child)
                                .or_insert(Vec::new())
                                .push(parent.to_string());
                        }
                    }
                }

                let mut stack = Vec::new();
                let mut set = HashSet::new();

                for bag in bag_map.get("shiny gold").unwrap() {
                    stack.push(bag.to_owned());
                    set.insert(bag.to_owned());
                }

                let mut i = 0;

                while let Some(bag) = stack.get(i) {
                    if let Some(more_bags) = bag_map.get(bag) {
                        for other_bag in more_bags {
                            if set.contains(other_bag) {
                                continue;
                            }
                            stack.push(other_bag.to_owned());
                            set.insert(other_bag.to_owned());
                        }
                    }
                    i += 1;
                }
                println!("Answer: {}", stack.len());
            }
            2 => {
                let mut bag_map: HashMap<String, HashMap<String, usize>> = HashMap::new();

                for line in lines {
                    if let Ok(line) = line {
                        let parent = re1.captures(&line).unwrap().get(1).unwrap().as_str();
                        let parent_entry =
                            bag_map.entry(parent.to_string()).or_insert(HashMap::new());

                        for cap in re2.captures_iter(&line) {
                            let count = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let child = cap.get(2).unwrap().as_str().to_string();

                            parent_entry.insert(child, count);
                        }
                    }
                }
                println!("Answer: {}", deep_count(&bag_map, "shiny gold", 1) - 1);
            }
            _ => panic!("Part must be either '1' or '2'."),
        }
    }
}
