use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn aoc_generator(input: &str) -> Vec<(String, i64)> {
    fn parse_in(i: &str) -> (String, i64) {
        let s: Vec<&str> = i.split(" ").collect();
        return (String::from(s[0]), s[1].parse::<i64>().unwrap())
    }
    input.lines().map(parse_in).collect()
}

#[aoc(day2, part1)]
pub fn solve(input: &[(String, i64)]) -> i64 {
    let mut depth = 0;
    let mut dist = 0;
    for (s, x) in input {
        match s.as_str() {
            "forward" => dist += x,
            "down" => depth += x,
            "up" =>  depth -= x,
            _ => (),
        }
    }
    // println!("{:?}", input);
    println!("depth: {}", depth);
    println!("dist: {}", dist);
    depth * dist
}

#[aoc(day2, part2)]
pub fn solve2(input: &[(String, i64)]) -> i64 {
    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;
    for (s, x) in input {
        match s.as_str() {
            "forward" => {
                dist += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" =>  aim -= x,
            _ => (),
        }
    }
    // println!("{:?}", input);
    println!("depth: {}", depth);
    println!("dist: {}", dist);
    println!("aim: {}", aim);
    depth * dist
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
