use regex::Regex;
use std::collections::HashSet;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    let re = Regex::new(r"(acc|jmp|nop) ([-\+]\d+)").unwrap();

    let mut instructions = Vec::new();

    if let Ok(lines) = read_lines("./test_inputs/day_8.txt") {
        for line in lines {
            if let Ok(line) = line {
                let cap = re.captures(&line).unwrap();

                let cmd = cap.get(1).unwrap().as_str();
                let arg = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

                instructions.push((cmd.to_owned(), arg));
            }
        }
    }

    let mut acc = 0;

    match part {
        1 => {
            let mut i = 0;
            let mut visited: HashSet<usize> = HashSet::new();

            while i < instructions.len() {
                if visited.contains(&i) {
                    break;
                }
                visited.insert(i);

                let (cmd, arg) = instructions.get(i).unwrap();

                match cmd.as_ref() {
                    "acc" => acc += arg,
                    "jmp" => {
                        i = (i as i32 + arg) as usize;

                        continue;
                    }
                    "nop" => (),
                    _ => unreachable!(),
                }
                i += 1;
            }
        }
        2 => {
            let mut i = 0;
            let mut a = false;
            let mut altered: HashSet<usize> = HashSet::new();
            let mut visited: HashSet<usize> = HashSet::new();

            while i < instructions.len() {
                if visited.contains(&i) {
                    visited.drain();
                    acc = 0;
                    i = 0;
                    a = false;
                    continue;
                }
                visited.insert(i);

                let (cmd, arg) = instructions.get(i).unwrap();

                match cmd.as_ref() {
                    "acc" => acc += arg,
                    "jmp" => {
                        if a || altered.contains(&i) {
                            i = (i as i32 + arg) as usize;
                            continue;
                        }
                        a = true;
                        altered.insert(i);
                    }
                    "nop" => {
                        if !a && !altered.contains(&i) {
                            a = true;
                            altered.insert(i);
                            i = (i as i32 + arg) as usize;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                i += 1;
            }
        }
        _ => panic!("Part must be either '1' or '2'."),
    }

    println!("Answer: {}", acc);
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
