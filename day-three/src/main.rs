const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut enabled = true;
    let sum = INPUT
        .split("mul(")
        .into_iter()
        .filter_map(|line| {
            let skip = !enabled;

            match (line.find("do()"), line.find("don't()")) {
                (Some(do_idx), Some(dont_idx)) => {
                    enabled = do_idx > dont_idx;
                }
                (Some(_), None) => enabled = true,
                (None, Some(_)) => enabled = false,
                _ => (),
            }

            if skip {
                return None;
            }

            Some(line)
        })
        .filter(|input| matches!(input.chars().next(), Some('0'..='9')))
        .filter_map(|input| input.split_once(')'))
        .filter_map(|(input, _)| input.split_once(','))
        .filter_map(|(left, right)| {
            left.parse()
                .ok()
                .and_then(|left: u32| right.parse().and_then(|right: u32| Ok((left, right))).ok())
        })
        .map(|(left, right)| left * right)
        .sum::<u32>();

    println!("answer: {sum}");
}
