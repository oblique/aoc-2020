use anyhow::{bail, Context, Result};
use aoc_2020::file_lines;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
enum Insn {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum Exit {
    Success(i32),
    InfiniteLoop(i32),
}

fn assemble() -> Result<Vec<Insn>> {
    file_lines("./data/input08.txt")?
        .map(|ln| {
            let mut split = ln.splitn(2, ' ');

            let insn = split.next().context("Instruction not found")?;
            let num = split
                .next()
                .context("Operand not found")?
                .parse::<i32>()
                .context("Invalid operand")?;

            match insn {
                "acc" => Ok(Insn::Acc(num)),
                "jmp" => Ok(Insn::Jmp(num)),
                "nop" => Ok(Insn::Nop(num)),
                _ => bail!("Invalid instruction"),
            }
        })
        .collect::<Result<Vec<_>>>()
}

fn run_code(code: &[Insn]) -> Result<Exit> {
    let mut acc = 0;
    let mut pc = 0;
    let mut executed_insns = vec![false; code.len()];

    while !executed_insns[pc] {
        executed_insns[pc] = true;

        match code[pc] {
            Insn::Acc(x) => {
                acc += x;
                pc = pc.checked_add(1).context("Integer overflow")?;
            }
            Insn::Jmp(x) if x >= 0 => {
                let x = usize::try_from(x).context("Integer overflow")?;
                pc = pc.checked_add(x).context("Integer overflow")?;
            }
            Insn::Jmp(x) => {
                let x = usize::try_from(-x).context("Integer overflow")?;
                pc = pc.checked_sub(x).context("Integer underflow")?;
            }

            Insn::Nop(_) => {
                pc = pc.checked_add(1).context("Integer overflow")?;
            }
        }

        if pc == code.len() {
            return Ok(Exit::Success(acc));
        } else if pc > code.len() {
            bail!("SEGFAULT");
        }
    }

    Ok(Exit::InfiniteLoop(acc))
}

fn solve_part1(code: &[Insn]) -> Result<()> {
    match run_code(code)? {
        Exit::InfiniteLoop(acc) => println!("part1: {}", acc),
        Exit::Success(_) => bail!("part1 not solved"),
    }

    Ok(())
}

fn solve_part2(code: &mut [Insn]) -> Result<()> {
    let mut acc = None;

    for i in 0..code.len() {
        let prev_insn = code[i].clone();

        match code[i] {
            Insn::Acc(_) => continue,
            Insn::Jmp(x) => code[i] = Insn::Nop(x),
            Insn::Nop(x) if x == 0 => continue,
            Insn::Nop(x) => code[i] = Insn::Jmp(x),
        }

        if let Ok(Exit::Success(res)) = run_code(code) {
            acc = Some(res);
            break;
        }

        code[i] = prev_insn;
    }

    match acc {
        Some(acc) => println!("part2: {}", acc),
        None => bail!("part2 not solved"),
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut code = assemble()?;

    solve_part1(&code[..])?;
    solve_part2(&mut code[..])?;

    Ok(())
}
