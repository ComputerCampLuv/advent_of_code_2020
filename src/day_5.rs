use regex::{Captures, Regex};

use crate::lib::read_lines;

pub fn perform(part: u8) {
    let re = Regex::new(r"([FBLR])").unwrap();
    let mut seat_ids = Vec::new();

    if let Ok(lines) = read_lines("./test_inputs/day_5.txt") {
        for line in lines {
            if let Ok(line) = line {
                let result = re.replace_all(&line, |caps: &Captures| match &caps[1] {
                    "F" | "L" => "0",
                    _ => "1",
                });
                seat_ids.push(
                    usize::from_str_radix(&result[0..7], 2).unwrap() * 8
                        + usize::from_str_radix(&result[7..10], 2).unwrap(),
                );
            }
        }
        seat_ids.sort_unstable();

        if part == 1 {
            println!("Answer: {}", seat_ids.last().unwrap());
            return;
        }

        let mut prev = *seat_ids.first().unwrap();

        for seat_id in seat_ids.iter() {
            if *seat_id - prev == 2 {
                println!("Answer: {}", seat_id - 1);
                return;
            }
            prev = *seat_id;
        }
    }
}
