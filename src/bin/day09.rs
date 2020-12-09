use anyhow::{bail, Result};
use aoc_2020::file_lines;

fn parse() -> Result<Vec<u64>> {
    file_lines("./data/input09.txt")?
        .map(|ln| ln.parse().map_err(Into::into))
        .collect()
}

fn is_valid_num(preamble: &[u64], num: u64) -> bool {
    for x in preamble {
        for y in preamble {
            if x == y {
                continue;
            }

            if x + y == num {
                return true;
            }
        }
    }

    false
}

fn find_invalid_num(buf: &[u64]) -> Result<u64> {
    for i in 25..buf.len() {
        let preamble = &buf[i - 25..i];
        let num = buf[i];

        if !is_valid_num(preamble, num) {
            return Ok(num);
        }
    }

    bail!("Failed to find invalid num");
}

fn solve_part1() -> Result<()> {
    let buf = parse()?;
    let invalid_num = find_invalid_num(&buf[..])?;

    println!("part1: {}", invalid_num);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let buf = parse()?;
    let invalid_num = find_invalid_num(&buf[..])?;

    for i in 0..buf.len() {
        let mut sum = 0;

        for j in i..buf.len() {
            sum += buf[j];

            if sum > invalid_num {
                break;
            } else if sum == invalid_num {
                let min = buf[i..j].iter().min().unwrap();
                let max = buf[i..j].iter().max().unwrap();

                println!("part2: {}", min + max);
                return Ok(());
            }
        }
    }

    bail!("Failed to solve part2");
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
