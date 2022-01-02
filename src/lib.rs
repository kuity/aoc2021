#[macro_use]
extern crate aoc_runner_derive;

// Generator: #[aoc_generator(dayX)]
// #[aoc_generator(day2)]
// pub fn input_generator(input: &str) -> Vec<Gift> {
//     input
//         .lines()
//         .map(|l| {
//             let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
//             (
//                 gift.next().unwrap(),
//                 gift.next().unwrap(),
//                 gift.next().unwrap(),
//             )
//         }).collect()
// }
// Solver: #[aoc(day2, part1)]
//#[aoc(day2, part1)]
// pub fn solve_part1(input: &[Gift]) -> u32 {
//     input
//         .iter()
//         .map(|&(l, w, h)| {
//             let (s1, s2) = smallest_side((l, w, h));
//             2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
//         })
//         .sum()
// }


pub mod day1;
pub mod day2;
pub mod day3;

aoc_lib!{ year = 2021 }
