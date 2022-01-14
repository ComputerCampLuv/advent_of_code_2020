use regex::Regex;

use crate::lib::read_lines;

pub fn perform(part: u8) {
    if let Ok(mut lines) = read_lines("./test_inputs/day_13.txt") {
        let start = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        match part {
            1 => {
                let re = Regex::new(r"\d+").unwrap();

                let mut waits = Vec::new();

                for cap in re.captures_iter(&lines.next().unwrap().unwrap()) {
                    let service = cap.get(0).unwrap().as_str().parse::<usize>().unwrap();
                    let wait = service - start % service;

                    waits.push((service, wait));
                }
                waits.sort_by_key(|k| k.1);

                if let Some((service, wait)) = waits.first() {
                    println!("Answer: {}", service * wait);
                }
            }
            2 => {
                let re = Regex::new(r"[^,]+").unwrap();

                let mut services = Vec::new();
                let mut big_n = 1;

                for (i, cap) in re
                    .captures_iter(&lines.next().unwrap().unwrap())
                    .enumerate()
                {
                    if let Ok(service) = cap.get(0).unwrap().as_str().parse::<usize>() {
                        big_n *= service;

                        services.push((service, i));
                    }
                }

                let total = &services[1..]
                    .iter()
                    .map(|(service, sequence)| {
                        let foo = (big_n / service) % service;
                        let mut x = 1;
                        while (foo * x) % service != 1 {
                            x += 1;
                        }

                        let mut service_normalized = *service;
                        while service_normalized < *sequence {
                            service_normalized += service;
                        }
                        let b = (service_normalized - sequence) % service;
                        let n = big_n / service;

                        b * n * x
                    })
                    .sum::<usize>();

                println!("Answer: {}", total % big_n);
            }
            _ => panic!("Part must be either '1' or '2'."),
        }
    }
}
