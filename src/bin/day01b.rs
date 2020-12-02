use anyhow::Result;
use aoc_2020::file_lines;

fn main() -> Result<()> {
    let lines = file_lines("./data/input01.txt")?;
    let nums: Vec<u32> = lines.filter_map(|ln| ln.parse().ok()).collect();

    'outer: for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}", nums[i] * nums[j] * nums[k]);
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}
