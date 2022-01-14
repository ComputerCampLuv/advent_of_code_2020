use crate::lib::read_lines;

pub fn perform(part: u8) {
    let mut area: [[char; 31]; 323] = [['.'; 31]; 323];

    if let Ok(lines) = read_lines("./test_inputs/day_3.txt") {
        for (i1, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (i2, c) in line.chars().enumerate() {
                    if c == '#' {
                        area[i1][i2] = '#';
                    }
                }
            }
        }
    }
    match part {
        1 => {
            println!("Answer: {}", count_trees(&area, 3, 1));
        }
        2 => {
            let answer = count_trees(&area, 1, 1)
                * count_trees(&area, 3, 1)
                * count_trees(&area, 5, 1)
                * count_trees(&area, 7, 1)
                * count_trees(&area, 1, 2);

            println!("Answer: {}", answer);
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}

fn count_trees(area: &[[char; 31]; 323], right: usize, down: usize) -> usize {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut trees = 0;

    while i1 < 323 {
        if area[i1][i2] == '#' {
            trees += 1;
        }
        i1 += down;
        i2 = (i2 + right) % 31;
    }
    return trees;
}
