mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod lib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args[1].parse::<u8>().unwrap();
    let part = args[2].parse::<u8>().unwrap();

    match day {
        1 => day_1::perform(part),
        2 => day_2::perform(part),
        3 => day_3::perform(part),
        4 => day_4::perform(part),
        5 => day_5::perform(part),
        6 => day_6::perform(part),
        7 => day_7::perform(part),
        8 => day_8::perform(part),
        9 => day_9::perform(part),
        10 => day_10::perform(part),
        11 => day_11::perform(part),
        12 => day_12::perform(part),
        13 => day_13::perform(part),
        14 => day_14::perform(part),
        15..=25 => unimplemented!("I haven't got there yet!"),
        _ => panic!("Day must be between '1' and '25'."),
    };
}