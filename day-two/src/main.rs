const INPUT: &str = include_str!("../input.txt");

fn is_safe(levels: &[i32]) -> bool {
    (levels.windows(2).all(|window| window[0] < window[1])
        || levels.windows(2).all(|window| window[0] > window[1]))
        && levels.windows(2).all(|window| {
            let diff = window[0].abs_diff(window[1]);
            diff >= 1 && diff <= 3
        })
}

fn is_safe_pd(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.to_vec();
        new_levels.remove(i);

        if is_safe(&new_levels) {
            return true;
        }
    }

    false
}

fn main() {
    let safe = INPUT
        .lines()
        .map(|line| {
            line.split(' ')
                .into_iter()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .filter(|result| {
            result
                .as_ref()
                .map(|levels| is_safe_pd(levels))
                .unwrap_or(true)
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()
        .unwrap();

    println!("safe: {:?}", safe.len());
}
