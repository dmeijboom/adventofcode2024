const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sum = INPUT
        .split("mul(")
        .into_iter()
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
