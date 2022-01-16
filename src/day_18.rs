use regex::{Captures, Regex};

use crate::lib::read_lines;

fn remove_parenthesis(input: &str) -> String {
    String::from(&input[1..input.len() - 1])
}

fn handle_eval(caps: &Captures) -> String {
    let op = caps.get(2).unwrap().as_str().chars().next().unwrap();

    let a = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let b = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

    let n = match op {
        '+' => a + b,
        '*' => a * b,
        _ => unreachable!(),
    };

    format!("{}", n)
}

fn handle_add(caps: &Captures) -> String {
    let a = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let b = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

    format!("{}", a + b)
}

fn handle_multiply(caps: &Captures) -> String {
    let a = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let b = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

    format!("{}", a * b)
}

fn handle_parenthesis_v1(caps: &Captures) -> String {
    let mut result = remove_parenthesis(caps.get(1).unwrap().as_str());

    let re_eval = Regex::new(r"(\d+) ([\+\*]) (\d+)").unwrap();

    while re_eval.is_match(&result) {
        result = re_eval.replace(&result, handle_eval).to_string();
    }
    result
}

fn handle_parenthesis_v2(caps: &Captures) -> String {
    let mut result = remove_parenthesis(caps.get(1).unwrap().as_str());

    let re_add = Regex::new(r"(\d+) \+ (\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+) \* (\d+)").unwrap();

    while re_add.is_match(&result) {
        result = re_add.replace_all(&result, handle_add).to_string();
    }

    while re_mul.is_match(&result) {
        result = re_mul.replace_all(&result, handle_multiply).to_string();
    }
    result
}

pub fn perform(part: u8) {
    if let Ok(lines) = read_lines("./test_inputs/day_18.txt") {
        let re_par = Regex::new(r"(\([^\(\)]+\))").unwrap();

        let mut total = 0;

        match part {
            1 => {
                let re_eval = Regex::new(r"(\d+) ([\+\*]) (\d+)").unwrap();

                for line in lines {
                    if let Ok(line) = line {
                        let mut result = line.to_string();
        
                        while re_par.is_match(&result) {
                            result = re_par.replace_all(&result, handle_parenthesis_v1).to_string();
                        }

                        while re_eval.is_match(&result) {
                            result = re_eval.replace(&result, handle_eval).to_string();
                        }

                        total += result.parse::<usize>().unwrap();
                    }
                }
            },
            2 => {
                let re_add = Regex::new(r"(\d+) \+ (\d+)").unwrap();
                let re_mul = Regex::new(r"(\d+) \* (\d+)").unwrap();

                for line in lines {
                    if let Ok(line) = line {
                        let mut result = line.to_string();
        
                        while re_par.is_match(&result) {
                            result = re_par.replace_all(&result, handle_parenthesis_v2).to_string();
                        }

                        while re_add.is_match(&result) {
                            result = re_add.replace_all(&result, handle_add).to_string();
                        }

                        while re_mul.is_match(&result) {
                            result = re_mul.replace_all(&result, handle_multiply).to_string();
                        }
                        total += result.parse::<usize>().unwrap();
                    }
                }
            },
            _ => panic!("Part must be either '1' or '2'."),
        }
        println!("Answer: {}", total);
    }
}
