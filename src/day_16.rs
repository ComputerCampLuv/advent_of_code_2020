use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::lib::read_lines;

enum TicketData {
    Rules,
    YourTicket,
    NearbyTickets,
    Heading,
}

pub fn perform(part: u8) {
    let re_rules = Regex::new(r"([\sa-z]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut next_data = TicketData::Rules;
    let mut rules: HashMap<String, (_, HashSet<usize>, Option<usize>)> = HashMap::new();
    let mut scanning_error_rate = 0;
    let mut my_ticket = Vec::new();

    if let Ok(lines) = read_lines("./test_inputs/day_16.txt") {
        'next_line: for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    next_data = TicketData::Heading;
                    continue;
                }

                match next_data {
                    TicketData::Rules => {
                        let captures = re_rules.captures(&line).unwrap();

                        let field = captures.get(1).unwrap().as_str().to_owned();
                        let lower_a = captures
                            .get(2)
                            .unwrap()
                            .as_str()
                            .to_owned()
                            .parse::<usize>()
                            .unwrap();
                        let upper_a = captures
                            .get(3)
                            .unwrap()
                            .as_str()
                            .to_owned()
                            .parse::<usize>()
                            .unwrap();
                        let lower_b = captures
                            .get(4)
                            .unwrap()
                            .as_str()
                            .to_owned()
                            .parse::<usize>()
                            .unwrap();
                        let upper_b = captures
                            .get(5)
                            .unwrap()
                            .as_str()
                            .to_owned()
                            .parse::<usize>()
                            .unwrap();

                        let predicate = move |n: usize| {
                            !(n < lower_a || n > upper_b || (n > upper_a && n < lower_b))
                        };
                        rules.insert(field, (predicate, HashSet::new(), None));
                    }
                    TicketData::YourTicket => {
                        my_ticket = line
                            .split(",")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
                    }
                    TicketData::NearbyTickets => {
                        let values = line
                            .split(",")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        'next_value: for value in values.iter() {
                            for (predicate, _, _) in rules.values() {
                                if predicate(*value) {
                                    continue 'next_value;
                                }
                            }
                            scanning_error_rate += value;

                            continue 'next_line;
                        }

                        for (col, value) in values.into_iter().enumerate() {
                            for (_field, (predicate, set, _)) in rules.iter_mut() {
                                if predicate(value) {
                                    continue;
                                }
                                set.insert(col);
                            }
                        }
                    }
                    TicketData::Heading => {
                        next_data = match line.as_ref() {
                            "your ticket:" => TicketData::YourTicket,
                            "nearby tickets:" => TicketData::NearbyTickets,
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    match part {
        1 => {
            println!("Answer: {}", scanning_error_rate);
            return;
        }
        2 => (),
        _ => panic!("Part must be either '1' or '2'."),
    }

    let mut full_set: HashSet<usize> = HashSet::new();

    for i in 0..20 {
        full_set.insert(i);
    }

    loop {
        let mut found: Option<usize> = None;

        for (_, (_, set, known)) in rules.iter_mut() {
            if known.is_some() {
                continue;
            }
            let possibilites: Vec<_> = full_set.symmetric_difference(set).collect();

            if possibilites.len() == 1 {
                found = Some(*possibilites[0]);
                *known = Some(*possibilites[0]);
                break;
            }
        }

        if let Some(col) = found {
            for (_, (_, set, known)) in rules.iter_mut() {
                if known.is_some() {
                    continue;
                }
                set.insert(col);
            }
        } else {
            break;
        }
    }

    let re_departure = Regex::new(r"departure [a-z]+").unwrap();
    let mut result = 1;

    for (field, (_, _, known)) in rules.iter() {
        if re_departure.is_match(field) {
            result *= my_ticket[known.unwrap()];
        }
    }

    println!("Answer: {}", result);
}
