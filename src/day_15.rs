use std::collections::HashMap;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    if let Ok(lines) = read_lines("./test_inputs/day_15.txt") {
        for line in lines {
            let mut preamble = Vec::new();

            if let Ok(line) = line {
                for n in line.split(",") {
                    preamble.push(n.parse::<usize>().unwrap());
                }
            }
            let nth = match part {
                1 => 2020,
                2 => 30000000,
                _ => panic!("Part must be either '1' or '2'."),
            };
            println!("Answer: {}", calculate_nth(preamble, nth));
        }
    }
}

fn calculate_nth(mut preamble: Vec<usize>, nth: usize) -> usize {
    let mut i = 1;
    let mut spoken = preamble.pop().unwrap();
    let mut history = HashMap::new();

    // Assuming there isn't any repeating numbers in the preamble
    for n in preamble {
        history.insert(n, i);
        i += 1;
    }

    for i in i..nth {
        if let Some(n) = history.get_mut(&spoken) {
            spoken = i - *n;
            *n = i;
        } else {
            history.insert(spoken, i);
            spoken = 0;
        }
    }
    spoken
}