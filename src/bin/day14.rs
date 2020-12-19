use anyhow::{bail, Context, Result};
use aoc_2020::file_lines;
use std::collections::HashMap;

enum Insn {
    Mask(u64, u64),
    Mem(u64, u64),
}

fn parse_mask(s: &str) -> Result<(u64, u64)> {
    let mut x_bits = 0;
    let mut set_bits = 0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            'X' => x_bits |= 1 << i,
            '1' => set_bits |= 1 << i,
            '0' => {}
            _ => bail!("Invalid input"),
        }
    }

    Ok((x_bits, set_bits))
}

fn parse() -> Result<Vec<Insn>> {
    file_lines("./data/input14.txt")?
        .map(|ln| {
            let mut split = ln.split(" = ");
            let k = split.next().context("Invalid input")?;
            let v = split.next().context("Invalid input")?;

            if k == "mask" {
                let (x, set) = parse_mask(v)?;
                Ok(Insn::Mask(x, set))
            } else if let Some(k) = k
                .strip_prefix("mem[")
                .and_then(|k| k.strip_suffix("]"))
                .and_then(|k| k.parse().ok())
            {
                let v = v.parse().context("Invalid input")?;
                Ok(Insn::Mem(k, v))
            } else {
                bail!("Invalid input");
            }
        })
        .collect()
}

fn solve_part1() -> Result<()> {
    let insns = parse()?;
    let mut keep_bits = !0;
    let mut or_bits = 0;
    let mut mem = HashMap::new();

    for insn in insns {
        match insn {
            Insn::Mask(x, set) => {
                keep_bits = x;
                or_bits = set;
            }
            Insn::Mem(addr, mut val) => {
                val &= keep_bits;
                val |= or_bits;
                mem.insert(addr, val);
            }
        }
    }

    println!("part1: {}", mem.iter().map(|(_, v)| *v).sum::<u64>());

    Ok(())
}

fn ones_pos(val: u64) -> Vec<u64> {
    let mut pos = Vec::new();

    for i in 0..64 {
        if val & (1 << i) != 0 {
            pos.push(i);
        }
    }

    pos
}

fn solve_part2() -> Result<()> {
    let insns = parse()?;
    let mut mem = HashMap::new();
    let mut x_bits_pos = Vec::new();
    let mut set_bits = 0;

    for insn in insns {
        match insn {
            Insn::Mask(x, set) => {
                x_bits_pos = ones_pos(x);
                set_bits = set;
            }
            Insn::Mem(mut addr, val) => {
                addr |= set_bits;

                for bits in 0..(1 << x_bits_pos.len()) {
                    for (i, pos) in x_bits_pos.iter().enumerate() {
                        if bits & (1 << i) != 0 {
                            addr |= 1 << pos;
                        } else {
                            addr &= !(1 << pos);
                        }
                    }

                    mem.insert(addr, val);
                }
            }
        }
    }

    println!("part2: {}", mem.iter().map(|(_, v)| *v).sum::<u64>());

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
