use anyhow::Result;
use aoc_2020::file_lines;
use std::str::FromStr;

struct PassPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PassPolicy {
    fn parse(s: &str) -> Option<Self> {
        let mut pol = s.splitn(2, ' ');
        let mut min_max =
            pol.next()?.splitn(2, '-').filter_map(|x| usize::from_str(x).ok());

        Some(PassPolicy {
            min: min_max.next()?,
            max: min_max.next()?,
            letter: pol.next()?.chars().next()?,
        })
    }

    fn valid_password(&self, passwd: &str) -> bool {
        let count = passwd.matches(self.letter).count();

        count >= self.min && count <= self.max
    }
}

fn to_policy_and_pass(s: String) -> Option<(PassPolicy, String)> {
    let mut x = s.splitn(2, ':');
    let pol = x.next()?;
    let pass = x.next()?.trim();
    Some((PassPolicy::parse(pol)?, pass.to_owned()))
}

fn main() -> Result<()> {
    let lines = file_lines("./data/input02.txt")?;
    let mut valid_pass = 0;

    for (policy, pass) in lines.filter_map(to_policy_and_pass) {
        if policy.valid_password(&pass) {
            valid_pass += 1;
        }
    }

    println!("{}", valid_pass);

    Ok(())
}
