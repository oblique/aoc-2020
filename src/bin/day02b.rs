use anyhow::Result;
use aoc_2020::file_lines;
use std::str::FromStr;

struct PassPolicy {
    allowed_pos: Vec<usize>,
    letter: char,
}

impl PassPolicy {
    fn parse(s: &str) -> Option<Self> {
        let mut x = s.splitn(2, ' ');

        let allowed_pos: Vec<_> = x
            .next()?
            .splitn(2, '-')
            .filter_map(|x| usize::from_str(x).ok())
            .filter_map(|x| x.checked_sub(1))
            .collect();
        let letter = x.next()?.chars().next()?;

        Some(PassPolicy {
            allowed_pos,
            letter,
        })
    }

    fn valid_password(&self, passwd: &str) -> bool {
        let mut found = false;

        for pos in &self.allowed_pos {
            if passwd.chars().nth(*pos) == Some(self.letter) {
                if found {
                    return false;
                }

                found = true;
            }
        }

        found
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
