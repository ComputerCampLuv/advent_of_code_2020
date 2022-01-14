use regex::Regex;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut valid = 0_u16;

    if let Ok(lines) = read_lines("./test_inputs/day_2.txt") {
        for line in lines {
            if let Ok(line) = line {
                let captures = re.captures(&line).unwrap();

                let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let chr = captures.get(3).unwrap().as_str().chars().next().unwrap();
                let psw = captures.get(4).unwrap().as_str();

                let mut count = 0_usize;

                match part {
                    1 => {
                        for character in psw.chars() {
                            if character == chr {
                                count += 1;
                            }
                        }
                        if count < min || count > max {
                            continue;
                        }
                        valid += 1;
                    }
                    2 => {
                        for (index, character) in psw.chars().enumerate() {
                            if character == chr && (index == min - 1 || index == max - 1) {
                                count += 1;
                            }
                        }
                        if count != 1 {
                            continue;
                        }
                        valid += 1;
                    }
                    _ => panic!("Part must be either '1' or '2'."),
                }
            }
        }
    }
    println!("Answer: {}", valid);
}
