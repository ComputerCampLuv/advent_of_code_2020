use regex::Regex;
use std::convert::From;

use crate::lib::read_lines;

#[derive(Debug, Copy, Clone)]
enum Bearing {
    East,
    South,
    West,
    North,
}

impl From<i32> for Bearing {
    fn from(mut v: i32) -> Self {
        while v < 0 {
            v += 4;
        }

        match v % 4 {
            0 => Bearing::East,
            1 => Bearing::South,
            2 => Bearing::West,
            3 => Bearing::North,
            _ => unreachable!(),
        }
    }
}

fn rotate_clockwise(pos: (i32, i32), turns: i32) -> (i32, i32) {
    if turns > 0 {
        return rotate_clockwise((pos.1, -pos.0), turns - 1);
    }
    pos
}
fn rotate_counter_clockwise(pos: (i32, i32), turns: i32) -> (i32, i32) {
    if turns > 0 {
        return rotate_counter_clockwise((-pos.1, pos.0), turns - 1);
    }
    pos
}

pub fn perform(part: u8) {
    let re = Regex::new(r"([NSEWLRF])(\d+)").unwrap();

    let mut x = 0;
    let mut y = 0;

    if let Ok(lines) = read_lines("./test_inputs/day_12.txt") {
        match part {
            1 => {
                let mut b = Bearing::East;

                for line in lines {
                    if let Ok(line) = line {
                        let captures = re.captures(&line).unwrap();

                        let op = captures.get(1).unwrap().as_str().chars().next().unwrap();
                        let arg = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

                        match op {
                            'L' => b = Bearing::from(b as i32 - (arg / 90)),
                            'R' => b = Bearing::from(b as i32 + (arg / 90)),
                            'N' => y += arg,
                            'S' => y -= arg,
                            'E' => x += arg,
                            'W' => x -= arg,
                            'F' => match b {
                                Bearing::North => y += arg,
                                Bearing::South => y -= arg,
                                Bearing::East => x += arg,
                                Bearing::West => x -= arg,
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            }
            2 => {
                let mut w = (10, 1);

                for line in lines {
                    if let Ok(line) = line {
                        let captures = re.captures(&line).unwrap();

                        let op = captures.get(1).unwrap().as_str().chars().next().unwrap();
                        let arg = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

                        match op {
                            'L' => w = rotate_counter_clockwise(w, arg / 90),
                            'R' => w = rotate_clockwise(w, arg / 90),
                            'N' => w = (w.0, w.1 + arg),
                            'S' => w = (w.0, w.1 - arg),
                            'E' => w = (w.0 + arg, w.1),
                            'W' => w = (w.0 - arg, w.1),
                            'F' => {
                                x += w.0 * arg;
                                y += w.1 * arg;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            _ => panic!("Part must be either '1' or '2'."),
        }
        println!("Answer: {}", x.abs() + y.abs())
    }
}
