use crate::lib::read_lines;

fn count_occupied(plan: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i1 in 0..plan.len() {
        for i2 in 0..plan[i1].len() {
            if plan[i1][i2] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn occupied_seats_adjacent(plan: &Vec<Vec<char>>, position: (usize, usize)) -> usize {
    let mut occupied_seats = 0;

    for direction in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if is_seat_occupied(plan, position, direction) {
            occupied_seats += 1;
        }
    }

    occupied_seats
}

fn is_seat_occupied(
    plan: &Vec<Vec<char>>,
    mut position: (usize, usize),
    direction: (i32, i32),
) -> bool {
    loop {
        let new_pos_0 = position.0 as i32 + direction.0;
        if new_pos_0 < 0 {
            return false;
        }
        let new_pos_1 = position.1 as i32 + direction.1;
        if new_pos_1 < 0 {
            return false;
        }
        position.0 = new_pos_0 as usize;
        position.1 = new_pos_1 as usize;

        match plan.get(position.0) {
            Some(row) => match row.get(position.1) {
                Some('.') => continue,
                Some('L') => return false,
                Some('#') => return true,
                Some(_) => unreachable!(),
                None => break,
            },
            None => break,
        }
    }
    false
}

pub fn perform(part: u8) {
    let mut floor_plan: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./test_inputs/day_11.txt") {
        for line in lines {
            if let Ok(line) = line {
                floor_plan.push(line.chars().collect());
            }
        }
    }

    match part {
        1 => {
            loop {
                let mut changes = Vec::new();

                for i1 in 0..floor_plan.len() {
                    for i2 in 0..floor_plan[i1].len() {
                        if floor_plan[i1][i2] == '.' {
                            continue;
                        }

                        let mut occupied_seats = 0;

                        if i1 > 0 {
                            if i2 > 0 {
                                if floor_plan[i1 - 1][i2 - 1] == '#' {
                                    occupied_seats += 1;
                                }
                            }

                            if floor_plan[i1 - 1][i2] == '#' {
                                occupied_seats += 1;
                            }

                            if i2 + 1 < floor_plan[i1].len() {
                                if floor_plan[i1 - 1][i2 + 1] == '#' {
                                    occupied_seats += 1;
                                }
                            }
                        }

                        if i2 > 0 {
                            if floor_plan[i1][i2 - 1] == '#' {
                                occupied_seats += 1;
                            }
                        }

                        if i2 + 1 < floor_plan[i1].len() {
                            if floor_plan[i1][i2 + 1] == '#' {
                                occupied_seats += 1;
                            }
                        }

                        if i1 + 1 < floor_plan.len() {
                            if i2 > 0 {
                                if floor_plan[i1 + 1][i2 - 1] == '#' {
                                    occupied_seats += 1;
                                }
                            }

                            if floor_plan[i1 + 1][i2] == '#' {
                                occupied_seats += 1;
                            }

                            if i2 + 1 < floor_plan[i1].len() {
                                if floor_plan[i1 + 1][i2 + 1] == '#' {
                                    occupied_seats += 1;
                                }
                            }
                        }

                        match floor_plan[i1][i2] {
                            'L' => {
                                if occupied_seats == 0 {
                                    changes.push((i1, i2, '#'));
                                }
                            }
                            '#' => {
                                if occupied_seats > 3 {
                                    changes.push((i1, i2, 'L'));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                if changes.len() == 0 {
                    break;
                }

                for (i1, i2, c) in changes {
                    floor_plan[i1][i2] = c;
                }
            }

            println!("Answer: {}", count_occupied(&floor_plan));
        }
        2 => {
            loop {
                let mut changes = Vec::new();

                for i1 in 0..floor_plan.len() {
                    for i2 in 0..floor_plan[i1].len() {
                        if floor_plan[i1][i2] == '.' {
                            continue;
                        }

                        let occupied_seats = occupied_seats_adjacent(&floor_plan, (i1, i2));

                        match floor_plan[i1][i2] {
                            'L' => {
                                if occupied_seats == 0 {
                                    changes.push((i1, i2, '#'));
                                }
                            }
                            '#' => {
                                if occupied_seats > 4 {
                                    changes.push((i1, i2, 'L'));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                if changes.len() == 0 {
                    break;
                }

                for (i1, i2, c) in changes {
                    floor_plan[i1][i2] = c;
                }
            }

            println!("Answer: {}", count_occupied(&floor_plan));
        }
        _ => panic!("Part must be either '1' or '2'."),
    }
}
