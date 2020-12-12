use anyhow::Result;
use aoc_2020::file_lines;

fn parse_joltages() -> Result<Vec<u32>> {
    file_lines("./data/input10.txt")?
        .map(|ln| ln.parse().map_err(Into::into))
        .collect()
}

fn solve_part1(joltages: &[u32]) -> u32 {
    let mut prev = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for i in 0..joltages.len() {
        let diff = joltages[i] - prev;

        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1;
        }

        prev = joltages[i];
    }

    diff_1 * diff_3
}

fn split_to_branches(joltages: &[u32]) -> Vec<&[u32]> {
    let mut v = Vec::new();
    let mut i = 0;
    let mut prev = 0;

    while i < joltages.len() {
        let base = i;

        while i < joltages.len() {
            let diff = joltages[i] - prev;

            if diff > 3 {
                break;
            }

            i += 1;
        }

        v.push(&joltages[base..i]);
        prev = joltages[i - 1];
    }

    v
}

fn solve_part2(joltages: &[u32]) -> u64 {
    let mut count = 1;
    let branches = split_to_branches(joltages);

    for i in 0..branches.len() - 1 {
        let cur_branch = branches[i];
        let mut next_num = branches[i + 1][0];

        let mut x = 1;
        let mut count_branch = 0;

        for j in (0..cur_branch.len()).rev() {
            if next_num - cur_branch[j] > 3 {
                count_branch += 1;
                next_num = cur_branch[j];
            } else {
                count_branch += x;
                x *= 2;
            }
        }

        count *= count_branch;
    }

    count
}

fn main() -> Result<()> {
    let mut joltages = parse_joltages()?;

    joltages.sort();
    joltages.push(joltages[joltages.len() - 1] + 3);

    println!("part1: {}", solve_part1(&joltages[..]));
    println!("part2: {}", solve_part2(&joltages[..]));

    Ok(())
}

#[test]
fn test_basic() {
    assert_eq!(solve_part2(&[1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]), 8);
    assert_eq!(solve_part2(&[1, 4, 5, 6, 7, 10]), 4);
    assert_eq!(solve_part2(&[1, 4, 5, 6, 7, 8, 11]), 7);
    assert_eq!(solve_part2(&[1, 2, 3, 6]), 4);
    assert_eq!(solve_part2(&[1, 4, 5, 6, 7, 10, 11, 12, 13, 14, 17]), 28);
    assert_eq!(
        solve_part2(&[
            1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28,
            31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49
        ]),
        19208
    );
}
