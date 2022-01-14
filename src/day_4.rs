use regex::Regex;
use std::convert::TryFrom;

use crate::lib::read_lines;

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

enum Height {
    Inches(usize),
    Centimetres(usize),
}

#[allow(dead_code)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    exp_year: usize,
    height: Height,
    hair_color: String,
    eye_color: EyeColor,
    id: String,
    country_id: Option<String>,
}

impl TryFrom<&str> for Passport {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut birth_year = None::<usize>;
        let mut issue_year = None::<usize>;
        let mut exp_year = None::<usize>;
        let mut height = None::<Height>;
        let mut hair_color = None::<String>;
        let mut eye_color = None::<EyeColor>;
        let mut id = None::<String>;
        let mut country_id = None::<String>;

        let re = Regex::new(r"(?P<field>\w+):(?P<value>[\w\d#]+)").unwrap();

        let captures = re.captures_iter(value);

        for caps in captures {
            match &caps["field"] {
                "byr" => {
                    if let Ok(year) = &caps["value"].parse::<usize>() {
                        if *year < 1920 || *year > 2002 {
                            continue;
                        }
                        birth_year = Some(*year);
                    }
                }
                "iyr" => {
                    if let Ok(year) = &caps["value"].parse::<usize>() {
                        if *year < 2010 || *year > 2020 {
                            continue;
                        }
                        issue_year = Some(*year);
                    }
                }
                "eyr" => {
                    if let Ok(year) = &caps["value"].parse::<usize>() {
                        if *year < 2020 || *year > 2030 {
                            continue;
                        }
                        exp_year = Some(*year);
                    }
                }
                "hgt" => {
                    let re = Regex::new(r"(?P<value>\d+)(?P<type>(cm|in))").unwrap();

                    let caps = re.captures(&caps["value"]).ok_or("Couldn't parse hieght")?;

                    if let Ok(value) = &caps["value"].parse::<usize>() {
                        height = match &caps["type"] {
                            "cm" => {
                                if *value < 150 || *value > 193 {
                                    continue;
                                }
                                Some(Height::Centimetres(*value))
                            }
                            "in" => {
                                if *value < 59 || *value > 76 {
                                    continue;
                                }
                                Some(Height::Inches(*value))
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^#[0-9a-fA-f]{6}$").unwrap();

                    if re.is_match(&caps["value"]) {
                        hair_color = Some(caps["value"].to_owned());
                    }
                }
                "ecl" => {
                    eye_color = match &caps["value"] {
                        "amb" => Some(EyeColor::Amber),
                        "blu" => Some(EyeColor::Blue),
                        "brn" => Some(EyeColor::Brown),
                        "gry" => Some(EyeColor::Grey),
                        "grn" => Some(EyeColor::Green),
                        "hzl" => Some(EyeColor::Hazel),
                        "oth" => Some(EyeColor::Other),
                        _ => None,
                    }
                }
                "pid" => {
                    let re = Regex::new(r"^\d{9}$").unwrap();

                    if re.is_match(&caps["value"]) {
                        id = Some(caps["value"].to_owned());
                    }
                }
                "cid" => {
                    country_id = Some(caps["value"].to_owned());
                }
                _ => panic!("Unrecognised Field"),
            }
        }

        Ok(Passport {
            birth_year: birth_year.ok_or("Birth year is not present or is invalid")?,
            issue_year: issue_year.ok_or("Issue year is not present or is invalid")?,
            exp_year: exp_year.ok_or("Expiration year is not present or is invalid")?,
            height: height.ok_or("Height is not present or is invalid")?,
            hair_color: hair_color.ok_or("Hair color is not present or is invalid")?,
            eye_color: eye_color.ok_or("Eye color is not present or is invalid")?,
            id: id.ok_or("ID is not present or is invalid")?,
            country_id,
        })
    }
}

#[allow(dead_code)]
struct FakePassport {
    birth_year: String,
    issue_year: String,
    exp_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    id: String,
    country_id: Option<String>,
}

impl TryFrom<&str> for FakePassport {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut birth_year = None::<String>;
        let mut issue_year = None::<String>;
        let mut exp_year = None::<String>;
        let mut height = None::<String>;
        let mut hair_color = None::<String>;
        let mut eye_color = None::<String>;
        let mut id = None::<String>;
        let mut country_id = None::<String>;

        let re = Regex::new(r"(?P<field>\w+):(?P<value>[\w\d#]+)").unwrap();

        let captures = re.captures_iter(value);

        for caps in captures {
            match &caps["field"] {
                "byr" => birth_year = Some(caps["value"].to_owned()),
                "iyr" => issue_year = Some(caps["value"].to_owned()),
                "eyr" => exp_year = Some(caps["value"].to_owned()),
                "hgt" => height = Some(caps["value"].to_owned()),
                "hcl" => hair_color = Some(caps["value"].to_owned()),
                "ecl" => eye_color = Some(caps["value"].to_owned()),
                "pid" => id = Some(caps["value"].to_owned()),
                "cid" => country_id = Some(caps["value"].to_owned()),
                _ => panic!("Unrecognised Field"),
            }
        }

        Ok(FakePassport {
            birth_year: birth_year.ok_or("Birth year is not present or is invalid")?,
            issue_year: issue_year.ok_or("Issue year is not present or is invalid")?,
            exp_year: exp_year.ok_or("Expiration year is not present or is invalid")?,
            height: height.ok_or("Height is not present or is invalid")?,
            hair_color: hair_color.ok_or("Hair color is not present or is invalid")?,
            eye_color: eye_color.ok_or("Eye color is not present or is invalid")?,
            id: id.ok_or("ID is not present or is invalid")?,
            country_id,
        })
    }
}

pub fn perform(part: u8) {
    let mut valid_passports = 0;

    if let Ok(lines) = read_lines("./test_inputs/day_4.txt") {
        let mut passport_data = String::new();

        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    match part {
                        1 => {
                            if let Ok(_) = FakePassport::try_from(passport_data.as_str()) {
                                valid_passports += 1;
                            }
                        }
                        2 => {
                            if let Ok(_) = Passport::try_from(passport_data.as_str()) {
                                valid_passports += 1;
                            }
                        }
                        _ => panic!("Part must be either '1' or '2'."),
                    }
                    passport_data = String::new();
                } else {
                    passport_data = format!("{} {}", passport_data, line);
                }
            }
        }
        match part {
            1 => {
                if let Ok(_) = FakePassport::try_from(passport_data.as_str()) {
                    valid_passports += 1;
                }
            }
            2 => {
                if let Ok(_) = Passport::try_from(passport_data.as_str()) {
                    valid_passports += 1;
                }
            }
            _ => panic!("Part must be either '1' or '2'."),
        }
    }
    println!("Answer: {}", valid_passports);
}
