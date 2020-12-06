use anyhow::{bail, Result};
use aoc_2020::file_lines;

fn parse_answers(s: &str, answers: &mut [u32]) -> Result<()> {
    for c in s.chars() {
        match c {
            'a'..='z' => {
                let n = (c as u8 - b'a') as usize;
                answers[n] += 1;
            }
            _ => bail!("Invalid answers string: {}", s),
        }
    }

    Ok(())
}

fn solve_part1() -> Result<()> {
    let mut answers = [0; 26];
    let mut sum = 0;

    for line in file_lines("./data/input06.txt")? {
        if line.is_empty() {
            sum += answers.iter().filter(|&&x| x > 0).count();

            // reset answers
            answers.iter_mut().for_each(|x| *x = 0);
            continue;
        }

        parse_answers(&line, &mut answers[..])?;
    }

    sum += answers.iter().filter(|&&x| x > 0).count();

    println!("part1: {}", sum);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let mut answers = [0; 26];
    let mut person_nr = 0;
    let mut sum = 0;

    for line in file_lines("./data/input06.txt")? {
        if line.is_empty() {
            sum += answers.iter().filter(|&&x| x == person_nr).count();

            // reset answers
            answers.iter_mut().for_each(|x| *x = 0);
            person_nr = 0;
            continue;
        }

        parse_answers(&line, &mut answers[..])?;
        person_nr += 1;
    }

    sum += answers.iter().filter(|&&x| x == person_nr).count();

    println!("part2: {}", sum);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
