use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn aoc_generator(input: &str) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    fn parse_in(i: &str) -> Vec<u32> {
        i.chars().map(|x| {x.to_digit(RADIX).unwrap()}).collect()
    }
    input.lines().map(parse_in).collect()
}

fn vec_2_integer(v: Vec<u32>) -> u64 {
    let s: String = v.iter().map(|x| {format!("{:?}", &x)}).collect();
    let b = isize::from_str_radix(&s, 2).unwrap();
    b as u64
}

#[aoc(day3, part1)]
pub fn solve(input: &Vec<Vec<u32>>) -> u64 {
    let mut total_lines = 0;
    let mut tracker: Vec<i64> = vec![0; input[0].len()];
    for line in input {
        total_lines += 1;
        for (i, x) in line.iter().enumerate() {
            tracker[i] += i64::from(*x);
        }
    }
    // println!("total lines: {}", total_lines);
    // println!("tracker: {:?}", tracker);
    let gamma: Vec<u32> = tracker.iter().map(|x| {(x > &(total_lines/2)) as u32}).collect();
    let epsilon: Vec<u32> = tracker.iter().map(|x| {(x <= &(total_lines/2)) as u32}).collect();

    vec_2_integer(gamma) * vec_2_integer(epsilon)
}

fn helper(input: &Vec<Vec<u32>>, search_index: u32, majority_flag: bool) -> (Option<Vec<Vec<u32>>>, u32) {
    // 1. Takes a vector of bitstrings
    // 2. and the index to search
    // Find the 'most common' bit
    // Filter out the matching patterns and perform recursion
    if search_index >= input[0].len() as u32 {
        return (None, search_index);
    }

    let (mut total, mut counter) = (0, 0);
    for line in input {
        total += 1;
        counter += i64::from(line[search_index as usize]);
    }

    let mut matching_bit = 0;
    if majority_flag && counter*2 >= total { // flip only if more or equal ones than zeros
        matching_bit = 1;
    } else if !majority_flag && counter*2 < total { // flip only if less ones than zeroes
        matching_bit = 1
    }

    let mut output = vec![];
    for line in input {
        if line[search_index as usize] == matching_bit {
            output.push(line.clone());
        }
    }

    if output.len() == 1 {
        return (Some(output), search_index);
    }

    helper(&output, search_index+1, majority_flag)
}

#[aoc(day3, part2)]
pub fn solver(input: &Vec<Vec<u32>>) -> u64 {
    let oxy_rating;
    match helper(input, 0, true) {
        (None, x) => {
            println!("Ended with no results, x={}", x);
            return 0;
        },
        (Some(ans), x) => {
            println!(", answer={:?}, x={}", ans, x);
            oxy_rating = vec_2_integer(ans[0].clone());
            println!("integer form: {}", oxy_rating);
        },
    }
    let co2_rating;
    match helper(input, 0, false) {
        (None, x) => {
            println!("Ended with no results, x={}", x);
            return 0;
        },
        (Some(ans), x) => {
            println!(", answer={:?}, x={}", ans, x);
            co2_rating = vec_2_integer(ans[0].clone());
            println!("integer form: {}", co2_rating);
        },
    }
    oxy_rating * co2_rating
}

//
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
