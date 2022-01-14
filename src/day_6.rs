use std::collections::HashSet;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    match part {
        1 => {
            let mut total = 0;

            if let Ok(lines) = read_lines("./test_inputs/day_6.txt") {
                let mut answers: HashSet<char> = HashSet::new();

                for line in lines {
                    if let Ok(line) = line {
                        if line == "" {
                            total += answers.len();
                            answers.drain();
                        }
                        for c in line.chars() {
                            answers.insert(c);
                        }
                    }
                }
            }
            println!("Answer: {}", total);
        }
        2 => {
            let mut total = 0;

            if let Ok(lines) = read_lines("./test_inputs/day_6.txt") {
                let mut responses: Vec<HashSet<char>> = Vec::new();

                for line in lines {
                    if let Ok(line) = line {
                        if line == "" {
                            if let Some(unanimous_response) =
                                responses.iter().cloned().reduce(|acc, hs| {
                                    acc.intersection(&hs).cloned().collect::<HashSet<char>>()
                                })
                            {
                                total += unanimous_response.len();
                            }

                            responses.clear();

                            continue;
                        }
                        let mut answers: HashSet<char> = HashSet::new();

                        for c in line.chars() {
                            answers.insert(c);
                        }
                        responses.push(answers);
                    }
                }
            }
            println!("Answer: {}", total);
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}
