use anyhow::{bail, Context, Result};
use aoc_2020::file_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    units: u32,
}

impl Direction {
    fn turn_right(&mut self, degrees: u32) -> Result<()> {
        for _ in 0..degrees / 90 {
            match self {
                Direction::East => *self = Direction::South,
                Direction::South => *self = Direction::West,
                Direction::West => *self = Direction::North,
                Direction::North => *self = Direction::East,
                _ => bail!("Direction can not turn right"),
            }
        }

        Ok(())
    }

    fn turn_left(&mut self, degrees: u32) -> Result<()> {
        for _ in 0..degrees / 90 {
            match self {
                Direction::East => *self = Direction::North,
                Direction::North => *self = Direction::West,
                Direction::West => *self = Direction::South,
                Direction::South => *self = Direction::East,
                _ => bail!("Direction can not turn left"),
            }
        }

        Ok(())
    }
}

fn parse_actions() -> Result<Vec<Action>> {
    file_lines("./data/input12.txt")?
        .map(|ln| {
            if !ln.is_char_boundary(1) {
                bail!("Invalid input");
            }

            let (dir, units) = ln.split_at(1);

            let direction = match dir {
                "N" => Direction::North,
                "S" => Direction::South,
                "E" => Direction::East,
                "W" => Direction::West,
                "L" => Direction::Left,
                "R" => Direction::Right,
                "F" => Direction::Forward,
                _ => bail!("Invalid direction"),
            };

            let units = units.parse().context("Invalid units")?;

            Ok(Action {
                direction,
                units,
            })
        })
        .collect()
}

fn solve_part1() -> Result<()> {
    let mut east_west: i32 = 0;
    let mut north_south: i32 = 0;
    let mut facing_dir = Direction::East;

    let actions = parse_actions()?;

    for action in actions {
        let direction = if action.direction == Direction::Forward {
            facing_dir
        } else {
            action.direction
        };

        match direction {
            Direction::Right => facing_dir.turn_right(action.units)?,
            Direction::Left => facing_dir.turn_left(action.units)?,
            Direction::East => east_west += action.units as i32,
            Direction::West => east_west -= action.units as i32,
            Direction::North => north_south += action.units as i32,
            Direction::South => north_south -= action.units as i32,
            _ => unreachable!(),
        }
    }

    println!("part1: {}", east_west.abs() + north_south.abs());

    Ok(())
}

fn solve_part2() -> Result<()> {
    let mut east_west_waypoint: i32 = 10;
    let mut north_south_waypoint: i32 = 1;
    let mut east_west: i32 = 0;
    let mut north_south: i32 = 0;

    let actions = parse_actions()?;

    for action in actions {
        match action.direction {
            Direction::Forward => {
                east_west += east_west_waypoint * action.units as i32;
                north_south += north_south_waypoint * action.units as i32;
            }
            Direction::Right => {
                for _ in 0..action.units / 90 {
                    let new_ew = north_south_waypoint;
                    north_south_waypoint = -east_west_waypoint;
                    east_west_waypoint = new_ew;
                }
            }
            Direction::Left => {
                for _ in 0..action.units / 90 {
                    let new_ns = east_west_waypoint;
                    east_west_waypoint = -north_south_waypoint;
                    north_south_waypoint = new_ns;
                }
            }
            Direction::East => east_west_waypoint += action.units as i32,
            Direction::West => east_west_waypoint -= action.units as i32,
            Direction::North => north_south_waypoint += action.units as i32,
            Direction::South => north_south_waypoint -= action.units as i32,
        }
    }

    println!("part2: {}", east_west.abs() + north_south.abs());

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
