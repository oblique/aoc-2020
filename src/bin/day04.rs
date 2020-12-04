#![allow(dead_code)]

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::str;

#[derive(Debug, Clone)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

#[derive(Debug, Clone)]
enum Height {
    Cm(u32),
    In(u32),
}

impl Passport {
    fn parse(s: &str) -> Option<Passport> {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;

        for field in s.split_whitespace() {
            let mut split = field.splitn(2, ':');
            let key = split.next()?;
            let val = split.next()?.trim();

            if val.is_empty() {
                continue;
            }

            match key {
                "byr" => birth_year = val.parse::<u32>().ok(),
                "iyr" => issue_year = val.parse::<u32>().ok(),
                "eyr" => expiration_year = val.parse::<u32>().ok(),
                "hgt" => height = Height::parse(val),
                "hcl" => {
                    hair_color = if is_valid_rgb(val) {
                        Some(val.to_owned())
                    } else {
                        None
                    }
                }
                "ecl" => {
                    eye_color = if is_valid_eye_color(val) {
                        Some(val.to_owned())
                    } else {
                        None
                    }
                }
                "pid" => {
                    passport_id = if is_valid_passport_id(val) {
                        Some(val.to_owned())
                    } else {
                        None
                    }
                }
                "cid" => country_id = Some(val.to_owned()),
                _ => {}
            }
        }

        Some(Passport {
            birth_year: birth_year?,
            issue_year: issue_year?,
            expiration_year: expiration_year?,
            height: height?,
            hair_color: hair_color?,
            eye_color: eye_color?,
            passport_id: passport_id?,
            country_id,
        })
    }
}

impl Height {
    fn parse(s: &str) -> Option<Height> {
        if let Some(cm) = s.strip_suffix("cm") {
            cm.parse().ok().map(Height::Cm)
        } else if let Some(inch) = s.strip_suffix("in") {
            inch.parse().ok().map(Height::In)
        } else {
            None
        }
    }
}

fn is_valid_rgb(rgb: &str) -> bool {
    if let Some(digits) = rgb.strip_prefix('#') {
        let mut count = 0;

        for digit in digits.chars() {
            match digit {
                '0'..='9' | 'a'..='f' | 'A'..='F' => count += 1,
                _ => return false,
            }
        }

        count == 6
    } else {
        false
    }
}

fn is_valid_eye_color(color: &str) -> bool {
    let valid_colors = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.contains(&color)
}

fn is_valid_passport_id(id: &str) -> bool {
    let mut count = 0;

    for digit in id.chars() {
        if digit >= '0' && digit <= '9' {
            count += 1;
        } else {
            return false;
        }
    }

    count == 9
}

fn parse_batch(path: impl AsRef<Path>) -> Result<Vec<Passport>> {
    let data = fs::read_to_string(path).context("Failed to read batch")?;
    Ok(data.split("\n\n").filter_map(Passport::parse).collect())
}

fn solve_part1() -> Result<()> {
    let data = fs::read_to_string("./data/input04.txt")
        .context("Failed to read batch")?;
    let needed_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let count = data
        .split("\n\n")
        .filter(|s| {
            let fields: Vec<_> = s
                .split_whitespace()
                .filter_map(|x| x.splitn(2, ':').next())
                .collect();

            for needed_field in needed_fields {
                if !fields.contains(needed_field) {
                    return false;
                }
            }

            true
        })
        .count();

    println!("part1: {}", count);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let passports = parse_batch("./data/input04.txt")?;

    let count_valid = passports
        .iter()
        .filter(|passport| {
            let valid_height = match passport.height {
                Height::Cm(cm) => (cm >= 150 && cm <= 193),
                Height::In(inch) => (inch >= 59 && inch <= 76),
            };

            valid_height
                && passport.birth_year >= 1920
                && passport.birth_year <= 2002
                && passport.issue_year >= 2010
                && passport.issue_year <= 2020
                && passport.expiration_year >= 2020
                && passport.expiration_year <= 2030
        })
        .count();

    println!("part2: {}", count_valid);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
