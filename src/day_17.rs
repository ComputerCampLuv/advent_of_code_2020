use std::collections::{HashMap, HashSet};

use crate::lib::read_lines;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Cube {
    fn surrounding_cubes(&self, dimensions: u8) -> Vec<Cube> {
        let capacity = match dimensions {
            // 1 => 2,
            // 2 => 8,
            3 => 26,
            4 => 80,
            _ => panic!(),
        };
        let mut cubes = Vec::with_capacity(capacity);
    
        for x in -1..=1 {
            for y in -1..=1 {
                let mut z_start = 0;
                let mut z_end = 0;

                if dimensions > 2 {
                    z_start = -1;
                    z_end = 1;
                }
                for z in z_start..=z_end {
                    let mut w_start = 0;
                    let mut w_end = 0;

                    if dimensions > 3 {
                        w_start = -1;
                        w_end = 1;
                    }
                    for w in w_start..=w_end {
                        if (x, y, z, w) == (0, 0, 0, 0) {
                            continue;
                        }
                        cubes.push(Cube {
                            x: self.x + x,
                            y: self.y + y,
                            z: self.z + z,
                            w: self.w + w,
                        })
                    }
                }
            }
        }
        cubes
    }
}

pub fn perform(part: u8) {
    let dimensions = match part {
        1 => 3,
        2 => 4,
        _ => panic!("Part must be either '1' or '2'."),
    };
    let mut active: HashSet<Cube> = HashSet::new();

    if let Ok(lines) = read_lines("./test_inputs/day_17.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        active.insert(Cube {
                            x: x as i64,
                            y: y as i64,
                            z: 0,
                            w: 0,
                        });
                    }
                }
            }
        }
    }

    for _ in 0..6 {
        let mut transitioning_to_inactive: Vec<Cube> = Vec::new();
        let mut inactive: HashMap<Cube, usize> = HashMap::new();
    
        for cube in active.iter() {
            let mut count = 0;
    
            for adj_cube in cube.surrounding_cubes(dimensions) {
                if active.contains(&adj_cube) {
                    count += 1;
                } else {
                    let inactive_entry = inactive.entry(adj_cube).or_insert(0);
                    *inactive_entry += 1;
                }
            }
    
            match count {
                2 | 3 => (),
                _ => transitioning_to_inactive.push(*cube),
            }
        }
    
        for cube in transitioning_to_inactive {
            active.remove(&cube);
        }
    
        for (cube, active_neighbours) in inactive {
            if active_neighbours == 3 {
                active.insert(cube);
            }
        }
    }
    println!("Active: {:?}", active.iter().count());
}
