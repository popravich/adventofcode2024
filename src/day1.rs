use std::collections::HashMap;

pub fn main(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut left = vec![];
    let mut right = vec![];
    let stream = input.lines().filter_map(|line| line.split_once("   "));
    for (l, r) in stream {
        left.push(l.parse::<usize>()?);
        right.push(r.parse::<usize>()?);
    }
    left.sort();
    right.sort();

    let sum: usize = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r));

    // Part 2

    let mut counts = HashMap::new();
    for r in right.iter() {
        counts.entry(*r).and_modify(|v| *v += 1).or_insert(1);
    }
    let sum2 = left
        .iter()
        .map(|l| l * counts.get(l).copied().unwrap_or(0))
        .sum();
    Ok((sum, sum2))
}
