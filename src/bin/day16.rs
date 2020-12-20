use anyhow::{Context, Result};
use aoc_2020::file_lines;
use std::collections::HashMap;
use std::iter;
use std::ops::RangeInclusive;

struct Data {
    rules: HashMap<String, RuleRange>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

struct RuleRange {
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

impl RuleRange {
    fn contains(&self, item: u32) -> bool {
        self.range1.contains(&item) || self.range2.contains(&item)
    }
}

impl Data {
    fn parse() -> Result<Self> {
        let mut lines = file_lines("./data/input16.txt")?;
        let mut rules = HashMap::new();

        for ln in &mut lines {
            if ln.is_empty() {
                break;
            }

            let mut split = ln.split(": ");
            let name = split.next().context("Invalid input")?;
            let ((range1_start, range1_end), (range2_start, range2_end)) =
                split
                    .next()
                    .and_then(|s| parse_rule(s))
                    .context("Invalid input")?;

            rules.insert(
                name.to_owned(),
                RuleRange {
                    range1: range1_start..=range1_end,
                    range2: range2_start..=range2_end,
                },
            );
        }

        lines.next();
        let my_ticket = lines
            .next()
            .and_then(|ln| parse_ticket(&ln))
            .context("Invalid input")?;

        lines.next();
        lines.next();
        let mut nearby_tickets = Vec::new();

        for ln in lines {
            let ticket = parse_ticket(&ln).context("Invalid input")?;
            nearby_tickets.push(ticket);
        }

        Ok(Data {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }

    fn field_is_valid(&self, field: u32) -> bool {
        for (_, rule) in &self.rules {
            if rule.contains(field) {
                return true;
            }
        }

        false
    }

    fn discard_invalid_tickets(&mut self) {
        let mut i = 0;

        'outer: while i < self.nearby_tickets.len() {
            for j in 0..self.nearby_tickets[i].len() {
                let field = self.nearby_tickets[i][j];

                if !self.field_is_valid(field) {
                    self.nearby_tickets.remove(i);
                    continue 'outer;
                }
            }

            i += 1;
        }
    }

    fn field_positions(&self) -> Result<HashMap<String, usize>> {
        let mut positions = HashMap::new();

        while positions.len() < self.rules.len() {
            for (name, rule) in self.rules.iter() {
                if positions.contains_key(name) {
                    continue;
                }

                let mut fields_count = vec![0; self.my_ticket.len()];

                for ticket in iter::once(&self.my_ticket)
                    .chain(self.nearby_tickets.iter())
                {
                    for (i, field) in ticket.iter().enumerate() {
                        if positions
                            .iter()
                            .find(|(_, pos)| **pos == i)
                            .is_some()
                        {
                            continue;
                        }

                        if rule.contains(*field) {
                            fields_count[i] += 1;
                        }
                    }
                }

                let (pos, max) = fields_count
                    .iter()
                    .enumerate()
                    .max_by_key(|x| x.1)
                    .unwrap();
                let max_count =
                    fields_count.iter().filter(|x| *x == max).count();

                if max_count == 1 {
                    positions.insert(name.to_owned(), pos);
                }
            }
        }

        Ok(positions)
    }
}

fn parse_rule(rule: &str) -> Option<((u32, u32), (u32, u32))> {
    let mut split = rule.split(" or ");

    let mut rule1 = split.next()?.split("-");
    let rule1_start = rule1.next()?.parse().ok()?;
    let rule1_end = rule1.next()?.parse().ok()?;

    let mut rule2 = split.next()?.split("-");
    let rule2_start = rule2.next()?.parse().ok()?;
    let rule2_end = rule2.next()?.parse().ok()?;

    Some(((rule1_start, rule1_end), (rule2_start, rule2_end)))
}

fn parse_ticket(s: &str) -> Option<Vec<u32>> {
    s.split(',').map(|s| s.parse().ok()).collect()
}

fn solve_part1() -> Result<()> {
    let data = Data::parse()?;
    let mut error_rate = 0;

    for ticket in &data.nearby_tickets {
        for field in ticket {
            if !data.field_is_valid(*field) {
                error_rate += field;
            }
        }
    }

    println!("part1: {}", error_rate);

    Ok(())
}

fn solve_part2() -> Result<()> {
    let mut data = Data::parse()?;
    let mut product: u64 = 1;

    data.discard_invalid_tickets();
    let positions = data.field_positions()?;

    for (name, pos) in positions {
        if name.starts_with("departure") {
            product *= data.my_ticket[pos] as u64;
        }
    }

    println!("part2: {}", product);

    Ok(())
}

fn main() -> Result<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
