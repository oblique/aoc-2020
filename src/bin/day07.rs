use anyhow::Result;
use aoc_2020::file_lines;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Bags {
    bags: HashMap<String, Vec<(u32, String)>>,
}

impl Bags {
    fn new() -> Result<Self> {
        static RE_BAG: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^([a-z]+ [a-z]+) bags contain (.+)").unwrap()
        });

        static RE_INNER_BAGS: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"([0-9]+) (\w+ \w+) bags?").unwrap());

        let bags = file_lines("./data/input07.txt")?
            .filter_map(|ln| {
                let cap = RE_BAG.captures(&ln)?;
                let bag = &cap[1];

                let inner_bags = RE_INNER_BAGS
                    .captures_iter(&cap[2])
                    .map(|cap| {
                        let num: u32 = cap[1].parse().unwrap();
                        let bag = &cap[2];
                        (num, bag.to_owned())
                    })
                    .collect();

                Some((bag.to_owned(), inner_bags))
            })
            .collect();

        Ok(Bags {
            bags,
        })
    }

    fn can_hold(&self, bag: &str) -> HashSet<&str> {
        let mut set = HashSet::new();
        self.recursive_can_hold(bag, &mut set);
        set
    }

    fn recursive_can_hold<'a>(&'a self, bag: &str, set: &mut HashSet<&'a str>) {
        for (b, inner_b) in self.bags.iter() {
            if inner_b.iter().any(|x| x.1 == bag) && set.insert(b) {
                self.recursive_can_hold(b, set);
            }
        }
    }

    fn count_inner_bags_of(&self, bag: &str) -> u32 {
        self.bags
            .get(bag)
            .map(|inner_bags| {
                inner_bags.iter().fold(0, |acc, (num, bag)| {
                    acc + num + num * self.count_inner_bags_of(bag)
                })
            })
            .unwrap_or(0)
    }
}

fn main() -> Result<()> {
    let bags = Bags::new()?;

    println!("part1: {}", bags.can_hold("shiny gold").len());
    println!("part2: {}", bags.count_inner_bags_of("shiny gold"));

    Ok(())
}
