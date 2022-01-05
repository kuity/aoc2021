use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Hash, Eq)]
pub struct Point(u32, u32);
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[aoc_generator(day5)]
pub fn aoc_generator(input: &str) -> Vec<(Point, Point)> {
    input.lines().map(|x| {
        let recs: Vec<&str> = x.split("->").collect();
        let r1: Vec<&str> = recs[0].trim().split(",").collect();
        let r11 = r1[0].parse::<u32>().unwrap();
        let r12 = r1[1].parse::<u32>().unwrap();
        let r2: Vec<&str> = recs[1].trim().split(",").collect();
        let r21 = r2[0].parse::<u32>().unwrap();
        let r22 = r2[1].parse::<u32>().unwrap();
        (Point(r11, r12), Point(r21, r22))
    }).collect()
}

fn num_exceeds(hm: &HashMap<Point, u32>, threshold: u32) -> u32 {
    let mut counter = 0;
    for (_, v) in hm {
        if *v >= threshold {
            counter += 1;
        }
    }
    counter
}

#[aoc(day5, part1)]
pub fn solve(input: &Vec<(Point, Point)>) -> u32 {
    // println!("{:?}", input);
    let mut hm = HashMap::new();
    for (p1, p2) in input.iter() {
        if p1.0 == p2.0 && p1.1 != p2.1 {
            let range = match p1.1 >= p2.1 {
                true => p2.1..=p1.1,
                false => p1.1..=p2.1
            };
            for j in range {
                let v = hm.entry(Point(p1.0, j)).or_insert(0 as u32);
                *v+=1;
            }
        } else if p1.1 == p2.1 && p1.0 != p2.0 {
            let range = match p1.0 >= p2.0 {
                true => p2.0..=p1.0,
                false => p1.0..=p2.0
            };
            for j in range {
                let v = hm.entry(Point(j, p1.1)).or_insert(0 as u32);
                *v+=1;
            }
        } else {
            continue
        }
    }
    // println!("hashmap looks like: {:?}", hm);
    num_exceeds(&hm, 2 as u32)
}

#[aoc(day5, part2)]
pub fn solve2(input: &Vec<(Point, Point)>) -> u32 {
    // println!("{:?}", input);
    let mut hm = HashMap::new();
    for (p1, p2) in input.iter() {
        let x_dist: i32 = (p1.0 as i32) - (p2.0 as i32);
        let y_dist: i32 = (p1.1 as i32) - (p2.1 as i32);
        if x_dist == 0 {
            let range = match p1.1 >= p2.1 {
                true => p2.1..=p1.1,
                false => p1.1..=p2.1
            };
            for j in range {
                let v = hm.entry(Point(p1.0, j)).or_insert(0 as u32);
                *v+=1;
            }
        } else if y_dist == 0 {
            let range = match p1.0 >= p2.0 {
                true => p2.0..=p1.0,
                false => p1.0..=p2.0
            };
            for j in range {
                let v = hm.entry(Point(j, p1.1)).or_insert(0 as u32);
                *v+=1;
            }
        } else if x_dist.abs() == y_dist.abs() {
            match (x_dist > 0, y_dist > 0) {
                (true, true) => {
                    for j in (0 as u32)..=(x_dist.abs() as u32) {
                        let v = hm.entry(Point(p2.0+j, p2.1+j)).or_insert(0 as u32);
                        *v+=1;
                    }
                },
                (true, false) => {
                    for j in (0 as u32)..=(x_dist.abs() as u32) {
                        let v = hm.entry(Point(p2.0+j, p2.1-j)).or_insert(0 as u32);
                        *v+=1;
                    }
                },
                (false, true) => {
                    for j in (0 as u32)..=(x_dist.abs() as u32) {
                        let v = hm.entry(Point(p2.0-j, p2.1+j)).or_insert(0 as u32);
                        *v+=1;
                    }
                },
                (false, false) => {
                    for j in (0 as u32)..=(x_dist.abs() as u32) {
                        let v = hm.entry(Point(p2.0-j, p2.1-j)).or_insert(0 as u32);
                        *v+=1;
                    }

                },
            }
        } else {
            continue
        }
    }
    // println!("hashmap looks like: {:?}", hm);
    num_exceeds(&hm, 2 as u32)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
