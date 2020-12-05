use anyhow::{bail, Context, Result};
use aoc_2020::file_lines;

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
    id: u32,
}

impl Seat {
    fn from_pos(row: usize, col: usize) -> Seat {
        Seat {
            row,
            col,
            id: (row * 8 + col) as u32,
        }
    }

    fn parse(s: &str) -> Result<Seat> {
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_col = 0;
        let mut max_col = 7;

        for c in s.chars() {
            match c {
                'F' => {
                    max_row -=
                        ((max_row - min_row) as f32 / 2f32).round() as usize
                }
                'B' => {
                    min_row +=
                        ((max_row - min_row) as f32 / 2f32).round() as usize
                }
                'L' => {
                    max_col -=
                        ((max_col - min_col) as f32 / 2f32).round() as usize
                }
                'R' => {
                    min_col +=
                        ((max_col - min_col) as f32 / 2f32).round() as usize
                }
                _ => bail!("Invalid input"),
            }
        }

        if min_col != max_col || min_row != max_row {
            bail!("Invalid input");
        }

        Ok(Seat::from_pos(max_row, max_col))
    }
}

fn solve_part1() -> Result<()> {
    let max_id = file_lines("./data/input05.txt")?
        .filter_map(|s| Seat::parse(&s).ok())
        .map(|seat| seat.id)
        .max()
        .unwrap_or(0);

    println!("part1: {}", max_id);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let seats =
        file_lines("./data/input05.txt")?.filter_map(|s| Seat::parse(&s).ok());

    let mut reserved_seats = [[false; 8]; 128];

    for seat in seats {
        reserved_seats[seat.row][seat.col] = true;
    }

    let mut first_reserved_found = false;
    let mut found_seat = None;

    'outer: for row in 0..reserved_seats.len() {
        for col in 0..reserved_seats[0].len() {
            if first_reserved_found && !reserved_seats[row][col] {
                found_seat = Some(Seat::from_pos(row, col));
                break 'outer;
            }

            if !first_reserved_found && reserved_seats[row][col] {
                first_reserved_found = true;
            }
        }
    }

    let found_seat = found_seat.context("Seat not found")?;
    println!("part2: {}", found_seat.id);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
