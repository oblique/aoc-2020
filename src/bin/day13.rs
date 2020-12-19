use anyhow::{Context, Result};
use aoc_2020::file_lines;

fn parse() -> Result<(u32, Vec<u32>)> {
    let mut lines = file_lines("./data/input13.txt")?;

    let arrived_tm =
        lines.next().and_then(|ln| ln.parse().ok()).context("Invalid input")?;

    let buses = lines
        .next()
        .map(|ln| ln.split(',').filter_map(|n| n.parse().ok()).collect())
        .context("Invalid input")?;

    Ok((arrived_tm, buses))
}

fn solve_part1() -> Result<()> {
    let (arrived_tm, buses) = parse()?;
    let mut closed_bus = buses[0];
    let mut closed_tm = arrived_tm + (buses[0] - arrived_tm % buses[0]);

    for bus in buses.iter().skip(1) {
        let tm = arrived_tm + (bus - arrived_tm % bus);

        if tm < closed_tm {
            closed_bus = *bus;
            closed_tm = tm;
        }
    }

    println!("part1: {}", (closed_tm - arrived_tm) * closed_bus);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    Ok(())
}
