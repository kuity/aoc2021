use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub struct Position(u32, u32);
#[derive(Debug)]
pub struct Board {
    numbers: HashMap<u32, Position>,
    ticked: Vec<Position>,
    unticked: Vec<u32>,
    won: bool,
}

fn init_board(input: Vec<u32>)-> Board {
    let mut y = 0;
    let mut x = 0;
    let mut numbers = HashMap::new();
    let mut unticked: Vec<u32> = vec![];
    for i in input.iter() {
        numbers.insert(*i, Position(x, y));
        unticked.push(*i);
        x += 1;
        if x % 5 == 0 {
            x = 0;
            y += 1;
        }
    }
    return Board {
        numbers,
        ticked: vec![],
        unticked,
        won: false,
    };
}

impl Board {
    fn check_win(&self) -> Option<u32> {
        let mut vers: HashMap<u32, usize> = HashMap::new();
        let mut hors: HashMap<u32, usize> = HashMap::new();
        for Position(x, y) in self.ticked.iter() {
            *vers.entry(*x).or_default() += 1;
            *hors.entry(*y).or_default() += 1;
        }
        // println!("vers: {:?}", vers);
        // println!("hors: {:?}", hors);
        let max_x = vers.into_iter().max_by_key(|(_, v)| *v).map(|(_, k)| k);
        let max_y = hors.into_iter().max_by_key(|(_, v)| *v).map(|(_, k)| k);
        if let Some(k) = max_x {
            // println!("max_x = {:?}", k);
            if k >= 5 {
                return Some(self.unticked.iter().sum());
            }
        }
        if let Some(k) = max_y {
            // println!("max_y = {:?}", k);
            if k >= 5 {
                return Some(self.unticked.iter().sum());
            }
        }
        None
    }
}

#[aoc_generator(day4)]
// Need Some representation for bingo board
// pub fn aoc_generator(input: &str) -> (Vec<u32>, Vec<Board>) {
pub fn aoc_generator(input: &str) -> (Vec<u32>, Vec<Vec<u32>>) {
    let mut input_lines = input.lines();
    let entries = input_lines.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    input_lines.next();
    let mut board_input = vec![];
    let mut board_inputs: Vec<Vec<u32>> = vec![];
    for i in input_lines {
        if i.len() == 0 {
            board_inputs.push(board_input);
            board_input = vec![];
            continue;
        }
        let mut numbers:Vec<u32> = i.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_input.append(&mut numbers);
    }

    (entries, board_inputs)
}

#[aoc(day4, part1)]
// pub fn solve(input: &mut (Vec<u32>, Vec<Board>)) -> u64 {
pub fn solve(input: &(Vec<u32>, Vec<Vec<u32>>)) -> u64 {
    let temp = input.0.clone();
    let mut entries = temp.iter();
    let board_inputs = input.1.iter();
    let mut boards: Vec<Board> = vec![];
    for bi in board_inputs {
        // println!("{:?}", board_input);
        let new_board = init_board(bi.to_vec());
        // println!("{:?}", new_board);
        boards.push(new_board);
    }

    loop {
        let i = entries.next().unwrap();
        println!("Current bingo entry: {}", i);
        for b in boards.iter_mut() {
            if let Some(pos) = b.numbers.get(&i) {
                (*b).ticked.push(pos.clone());
                // println!("{:?}", b.ticked);
                b.unticked.retain(|x| *x != *i );
                // println!("{:?}", b.unticked);
                if let Some(has_won) = b.check_win() {
                    return (has_won * i) as u64
                }
            }
        }
    }
}

#[aoc(day4, part2)]
// pub fn solve(input: &mut (Vec<u32>, Vec<Board>)) -> u64 {
pub fn solve2(input: &(Vec<u32>, Vec<Vec<u32>>)) -> u64 {
    let temp = input.0.clone();
    let mut entries = temp.iter();
    let board_inputs = input.1.iter();
    let mut boards: Vec<Board> = vec![];
    for bi in board_inputs {
        // println!("{:?}", board_input);
        let new_board = init_board(bi.to_vec());
        // println!("{:?}", new_board);
        boards.push(new_board);
    }
    let num_boards = boards.len();
    let mut num_won = 0;

    loop {
        let i = entries.next().unwrap();
        println!("Current bingo entry: {}", i);
        let mut final_score = 0;
        for b in boards.iter_mut() {
            if b.won == true {
                continue
            }
            if let Some(pos) = b.numbers.get(&i) {
                b.ticked.push(pos.clone());
                b.unticked.retain(|x| *x != *i );
                if let Some(has_won) = b.check_win() {
                    final_score = (has_won * i) as u64;
                    b.won = true;
                    num_won += 1;
                }
            }
        }
        println!("Current number of won boards = {}", num_won);
        if num_won == num_boards {
            return final_score;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
