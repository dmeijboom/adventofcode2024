const INPUT: &str = include_str!("../input.txt");

fn main() {
    let pairs = INPUT
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(left, right)| {
            left.parse::<i32>()
                .and_then(|l| right.parse::<i32>().and_then(|r| Ok((l, r))))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse input");
    let mut left = pairs.iter().cloned().map(|(l, _)| l).collect::<Vec<_>>();
    let mut right = pairs.iter().cloned().map(|(_, r)| r).collect::<Vec<_>>();

    left.sort();
    right.sort();

    // Part one
    let sum = left
        .iter()
        .cloned()
        .zip(right.iter().cloned())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    println!("part one: {sum}");

    // Part two
    let sum = left
        .iter()
        .cloned()
        .map(|l| l * right.iter().cloned().filter(|r| l == *r).count() as i32)
        .sum::<i32>();

    println!("part two: {sum}");
}
