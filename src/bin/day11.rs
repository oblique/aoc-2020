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

fn adjacent_occupied_seats(
    layout: &Vec<Vec<Cell>>,
    x: usize,
    y: usize,
) -> usize {
    let start_row = if x > 0 {
        x - 1
    } else {
        x
    };

    let start_col = if y > 0 {
        y - 1
    } else {
        y
    };

    let end_row = (x + 1).min(layout.len() - 1);
    let end_col = (y + 1).min(layout[0].len() - 1);

    let mut count = 0;

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if i == x && j == y {
                continue;
            }

            if layout[i][j] == Cell::OccupiedSeat {
                count += 1;
            }
        }
    }

    count
}

fn visible_occupied_seats(
    layout: &Vec<Vec<Cell>>,
    x: usize,
    y: usize,
) -> usize {
    let mut direction_finished = [[false; 3]; 3];

    #[rustfmt::skip]
    let directions = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [ (0, -1),  (0, 0),  (0, 1)],
        [ (1, -1),  (1, 0),  (1, 1)],
    ];

    let mut count = 0;

    for n in 1..layout.len().max(layout[0].len()) {
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                if direction_finished[i][j] {
                    continue;
                }

                let (dir_x, dir_y) = directions[i][j];

                let x2 = x as isize + (n as isize * dir_x);
                let y2 = y as isize + (n as isize * dir_y);

                if x2 < 0
                    || y2 < 0
                    || x2 as usize >= layout.len()
                    || y2 as usize >= layout[0].len()
                {
                    direction_finished[i][j] = true;
                    continue;
                }

                match layout[x2 as usize][y2 as usize] {
                    Cell::Floor => {}
                    Cell::EmptySeat => {
                        direction_finished[i][j] = true;
                    }
                    Cell::OccupiedSeat => {
                        count += 1;
                        direction_finished[i][j] = true;
                    }
                }
            }
        }

        let count_finished: usize = direction_finished
            .iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum();

        if count_finished == 8 {
            break;
        }
    }

    count
}

fn round<F>(layout: &mut Vec<Vec<Cell>>, tolerance: usize, count: F) -> bool
where
    F: Fn(&Vec<Vec<Cell>>, usize, usize) -> usize,
{
    let mut seat_changed = false;
    let saved_layout = layout.clone();

    for i in 0..layout.len() {
        for j in 0..layout[0].len() {
            let cur_seat = saved_layout[i][j];
            let occupied_seats = count(&saved_layout, i, j);

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
        let seat_changed = round(&mut layout, 4, |layout, i, j| {
            adjacent_occupied_seats(layout, i, j)
        });

        if !seat_changed {
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
        let seat_changed = round(&mut layout, 5, |layout, i, j| {
            visible_occupied_seats(layout, i, j)
        });

        if !seat_changed {
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
