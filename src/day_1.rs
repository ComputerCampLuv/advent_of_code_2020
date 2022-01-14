use crate::lib::read_lines;

pub fn perform(part: u8) {
    let mut v = Vec::with_capacity(200);

    if let Ok(lines) = read_lines("./test_inputs/day_1.txt") {
        for line in lines {
            if let Ok(num) = line {
                v.push(num.parse::<u32>().unwrap());
            }
        }
    }

    match part {
        1 => {
            for i1 in 0..199 {
                for i2 in (i1 + 1)..200 {
                    if v[i1] + v[i2] == 2020 {
                        println!("Answer: {}", v[i1] * v[i2]);
                        return;
                    }
                }
            }
        }
        2 => {
            for i1 in 0..198 {
                for i2 in (i1 + 1)..199 {
                    for i3 in (i2 + 1)..200 {
                        if v[i1] + v[i2] + v[i3] == 2020 {
                            println!("Answer: {}", v[i1] * v[i2] * v[i3]);
                            return;
                        }
                    }
                }
            }
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}
