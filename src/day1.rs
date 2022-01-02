use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn d1g(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| { l.parse::<i64>().unwrap() }).collect()
}

#[aoc(day1, part1)]
pub fn d1p1(input: &[i64]) -> i64 {
    let mut tmp = 0;
    let increases: i64 = input
        .iter()
        .map(|&d| {
            let prev = tmp;
            tmp = d;
            if d > prev {
                1
            } else {
                0
            }
        })
        .sum();
    increases - 1
}

#[aoc(day1, part2)]
pub fn d1p2(input: &[i64]) -> i64 {
    let mut tmp = 0;
    let mut windows = vec![];
    for (i, &d) in input.iter().enumerate() {
        if i > 0 {
                windows[i-1] += d;
        }
        if i > 1 {
            windows[i-2] += d;
        }
        windows.push(d);
    }
    // print!("{:?}", windows);
    let increases: i64 = windows
        .iter()
        .map(|&d| {
            let prev = tmp;
            tmp = d;
            if d > prev {
                1
            } else {
                0
            }
    })
    .sum();
    increases - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
