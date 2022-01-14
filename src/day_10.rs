use crate::lib::read_lines;

pub fn perform(part: u8) {
    let mut adapters = vec![(0, 1)];

    if let Ok(lines) = read_lines("./test_inputs/day_10.txt") {
        for line in lines {
            if let Ok(line) = line {
                if let Ok(n) = line.parse::<usize>() {
                    adapters.push((n, 0_usize));
                }
            }
        }
    }

    adapters.sort_unstable();

    match part {
        1 => {
            let mut ones = 0;
            let mut threes = 1;

            for i in 1..adapters.len() {
                match adapters[i].0 - adapters[i - 1].0 {
                    1 => ones += 1,
                    3 => threes += 1,
                    _ => (),
                }
            }

            println!("Answer: {}", ones * threes);
        }
        2 => {
            'outer: for index in 0..adapters.len() {
                let (volts_a, multiplier_a) = adapters[index];

                for i in 1..=3 {
                    match adapters.get_mut(index + i) {
                        Some((volts_b, multiplier_b)) => {
                            if *volts_b - volts_a > 3 {
                                continue 'outer;
                            }
                            *multiplier_b += multiplier_a;
                        }
                        None => continue 'outer,
                    }
                }
            }

            println!("Answer: {}", adapters.last().unwrap().1);
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}
