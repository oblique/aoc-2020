use anyhow::Result;
use aoc_2020::file_lines;

fn main() -> Result<()> {
    let lines = file_lines("./data/input01.txt")?;
    let nums: Vec<u32> = lines.filter_map(|ln| ln.parse().ok()).collect();

    'outer: for n1 in &nums {
        for n2 in &nums {
            if n1 + n2 == 2020 {
                println!("{}", n1 * n2);
                break 'outer;
            }
        }
    }

    Ok(())
}
