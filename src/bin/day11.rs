use anyhow::{bail, Result};
use aoc_2020::file_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn parse_layout() -> Result<Vec<Vec<Cell>>> {
    file_lines("./data/input11.txt")?
        .map(|ln| {
            ln.chars()
                .map(|c| match c {
                    '.' => Ok(Cell::Floor),
                    'L' => Ok(Cell::EmptySeat),
                    '#' => Ok(Cell::OccupiedSeat),
                    _ => bail!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

fn count_occupied_seats(
    layout: &Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    limit: Option<usize>,
) -> usize {
    #[rustfmt::skip]
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
         (0, -1),           (0, 1),
         (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;

    for (dir_x, dir_y) in &directions {
        let mut n = 0;

        while limit.map_or(true, |x| n < x) {
            n += 1;

            let x2 = x as isize + (n as isize * dir_x);
            let y2 = y as isize + (n as isize * dir_y);

            if x2 < 0
                || y2 < 0
                || x2 as usize >= layout.len()
                || y2 as usize >= layout[x2 as usize].len()
            {
                break;
            }

            match layout[x2 as usize][y2 as usize] {
                Cell::Floor => {}
                Cell::EmptySeat => break,
                Cell::OccupiedSeat => {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn round(
    layout: &mut Vec<Vec<Cell>>,
    tolerance: usize,
    limit: Option<usize>,
) -> bool {
    let mut seat_changed = false;
    let saved_layout = layout.clone();

    for i in 0..layout.len() {
        for j in 0..layout[i].len() {
            let cur_seat = saved_layout[i][j];
            let occupied_seats =
                count_occupied_seats(&saved_layout, i, j, limit);

            if cur_seat == Cell::EmptySeat && occupied_seats == 0 {
                layout[i][j] = Cell::OccupiedSeat;
                seat_changed = true;
            } else if cur_seat == Cell::OccupiedSeat
                && occupied_seats >= tolerance
            {
                layout[i][j] = Cell::EmptySeat;
                seat_changed = true;
            }
        }
    }

    seat_changed
}

fn solve_part1() -> Result<()> {
    let mut layout = parse_layout()?;

    loop {
        if !round(&mut layout, 4, Some(1)) {
            break;
        }
    }

    let occupied_seats: usize = layout
        .iter()
        .map(|row| {
            row.iter().filter(|&&cell| cell == Cell::OccupiedSeat).count()
        })
        .sum();

    println!("part1: {}", occupied_seats);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let mut layout = parse_layout()?;

    loop {
        if !round(&mut layout, 5, None) {
            break;
        }
    }

    let occupied_seats: usize = layout
        .iter()
        .map(|row| {
            row.iter().filter(|&&cell| cell == Cell::OccupiedSeat).count()
        })
        .sum();

    println!("part2: {}", occupied_seats);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
