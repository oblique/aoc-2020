use anyhow::{bail, Result};
use aoc_2020::file_lines;
use std::path::Path;

struct GeoMap {
    // true: has tree
    // false: no tree
    trees_grid: Vec<Vec<bool>>,
}

impl GeoMap {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let trees_grid = file_lines(path)?
            .map(|ln| {
                ln.chars()
                    .map(|c| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        _ => bail!("Invalid geomap"),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        if trees_grid.is_empty() {
            bail!("Empty geomap");
        }

        Ok(GeoMap {
            trees_grid,
        })
    }

    fn has_tree(&self, x: usize, y: usize) -> bool {
        let grid_x = x % self.trees_grid.len();
        let grid_y = y % self.trees_grid[0].len();

        self.trees_grid[grid_x][grid_y]
    }

    fn num_of_cols(&self) -> usize {
        self.trees_grid.len()
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for row in &self.trees_grid {
            for tree in row {
                if *tree {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn find_trees_with_slope(geomap: &GeoMap, right: usize, down: usize) -> usize {
    let mut trees_found = 0;
    let mut x = 0;
    let mut y = 0;

    while x < geomap.num_of_cols() {
        y += right;
        x += down;

        if geomap.has_tree(x, y) {
            trees_found += 1;
        }
    }

    trees_found
}

fn main() -> Result<()> {
    let geomap = GeoMap::load("./data/input03.txt")?;

    println!("part 1 answer: {}", find_trees_with_slope(&geomap, 3, 1));

    let part2_res = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().fold(
        1,
        |acc, (right, down)| {
            acc * find_trees_with_slope(&geomap, *right, *down)
        },
    );

    println!("part 2 anwser: {}", part2_res);

    Ok(())
}
