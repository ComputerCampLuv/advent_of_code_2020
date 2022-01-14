use regex::Regex;
use std::collections::HashMap;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    let re_mask = Regex::new(r"mask = ([X10]+)").unwrap();
    let re_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    if let Ok(lines) = read_lines("./test_inputs/day_14.txt") {
        match part {
            1 => {
                let mut min = 0;
                let mut max = 0;
                let mut mem = HashMap::new();

                for line in lines {
                    if let Ok(line) = line {
                        if let Some(caps) = re_mask.captures(&line) {
                            let mask = caps.get(1).unwrap().as_str();

                            min = u64::from_str_radix(&str::replace(mask, 'X', "0"), 2).unwrap();
                            max = u64::from_str_radix(&str::replace(mask, 'X', "1"), 2).unwrap();

                            continue;
                        }

                        if let Some(caps) = re_mem.captures(&line) {
                            let address = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

                            mem.insert(address, (value | min) & max);
                        }
                    }
                }
                println!(
                    "Answer: {}",
                    mem.values().cloned().reduce(|acc, n| acc + n).unwrap()
                );
            }
            2 => {
                let mut mem = HashMap::new();
                let mut min = 0;
                let mut floating = 0;

                for line in lines {
                    if let Ok(line) = line {
                        if let Some(caps) = re_mask.captures(&line) {
                            let mask = caps.get(1).unwrap().as_str();

                            min = u64::from_str_radix(&str::replace(mask, 'X', "0"), 2).unwrap();
                            floating = u64::from_str_radix(
                                &str::replace(&str::replace(mask, '1', "0"), 'X', "1"),
                                2,
                            )
                            .unwrap();

                            continue;
                        }

                        if let Some(caps) = re_mem.captures(&line) {
                            let address = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

                            write_to_memory(
                                &mut mem,
                                (u64::MAX ^ floating) & (address | min),
                                floating,
                                value,
                                1,
                            );
                        }
                    }
                }
                println!(
                    "Answer: {}",
                    mem.values().cloned().reduce(|acc, n| acc + n).unwrap()
                );
            }
            _ => panic!("Part must be either '1' or '2'."),
        }
    }
}

fn write_to_memory(mem: &mut HashMap<u64, u64>, address: u64, floating: u64, value: u64, bit: u64) {
    if floating < bit {
        mem.insert(address, value);
        return;
    }
    if floating & bit > 0 {
        write_to_memory(mem, address | (floating & bit), floating, value, bit << 1);
    }
    write_to_memory(mem, address, floating, value, bit << 1);
}
