use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn aoc_generator(input: &str) -> Vec<u32> {
    input.split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve(input: &Vec<u32>) -> u64 {
    let mut state = input.clone();
    for _ in 0..80 {
        let num_new: u32 = state.iter().map(|x| {
            if *x == 0 {
                1
            } else {
                0
            }
        }).sum::<u32>();
        for elem in state.iter_mut() {
            if *elem==0 {
                *elem = 6;
            } else {
                *elem = *elem-1
            }
        }
        state.append(&mut vec![8; num_new as usize]);
    }
    state.len() as u64
}

#[aoc(day6, part2)]
pub fn solve2(input: &Vec<u32>) -> u64 {
    let mut hm: HashMap<u32, u64> = HashMap::new();
    for i in input.iter() {
        let v = hm.entry(*i).or_insert(0 as u64);
        *v += 1;
    }

    for _ in 0..256 {
        let mut hm_new: HashMap<u32, u64> = HashMap::new();
        for (k, v) in hm {
            let new_pos = match k {
                0 => vec![6, 8],
                x => vec![x-1]
            };
            for p in new_pos.iter() {
                let new_v = hm_new.entry(*p).or_insert(0 as u64);
                *new_v += v;
            }
        }
        hm = hm_new;
    }

    hm.values().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
