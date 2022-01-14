use crate::lib::read_lines;

use std::cmp::Ordering;

pub fn perform(part: u8) {
    let mut invalid = 0;
    let mut numbers = Vec::with_capacity(1000);

    if let Ok(lines) = read_lines("./test_inputs/day_9.txt") {
        for line in lines {
            if let Ok(line) = line {
                numbers.push(line.parse::<usize>().unwrap());
            }
        }
    }

    'outer: for i1 in 25..1000 {
        let start = i1 - 25;
        let n = numbers[i1];

        for i2 in start..i1 - 1 {
            for i3 in i2..i1 {
                if numbers[i2] + numbers[i3] == n {
                    continue 'outer;
                }
            }
        }
        invalid = n;
        break;
    }

    match part {
        1 => {
            println!("Answer: {}", invalid);
        }
        2 => {
            let mut start = 0;
            let mut end = 1;

            loop {
                if let Some(sum) = numbers[start..=end]
                    .iter()
                    .copied()
                    .reduce(|acc, n| acc + n)
                {
                    match sum.cmp(&invalid) {
                        Ordering::Equal => break,
                        Ordering::Less => end += 1,
                        Ordering::Greater => {
                            start += 1;
                            end = start + 1;
                        }
                    }
                }
            }

            let mut slice = numbers[start..=end].iter().copied().collect::<Vec<usize>>();

            slice.sort_unstable();

            println!("Answer: {}", slice.first().unwrap() + slice.last().unwrap());
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}
