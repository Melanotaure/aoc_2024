use memoize::memoize;

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    let nb_iter = 25;
    input
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .map(|v| blink(nb_iter, v))
        .sum::<u64>()
}

#[memoize]
fn blink(nb_iter: u32, value: u64) -> u64 {
    if nb_iter == 0 {
        return 1;
    }

    let next_iter = nb_iter - 1;
    let nb_digits = if value == 0 { 1 } else { value.ilog10() + 1 };
    if value == 0 {
        blink(next_iter, 1)
    } else if nb_digits % 2 == 0 {
        let tenth_pow = 10_u64.pow(nb_digits / 2);
        let left = value / tenth_pow;
        let right = value - (left * tenth_pow);
        blink(next_iter, left) + blink(next_iter, right)
    } else {
        blink(next_iter, value * 2024)
    }
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u64 {
    let nb_iter = 75;
    input
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .map(|v| blink(nb_iter, v))
        .sum::<u64>()
}
