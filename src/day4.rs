use std::collections::{HashMap, HashSet};

use anyhow::Result;

type Position = (i32, i32);

struct BingoBoard {
    numbers: HashMap<Position, i32>,
}

impl BingoBoard {
    fn new(nums: &[i32]) -> Self {
        assert_eq!(nums.len(), 25);
        let mut numbers = HashMap::new();
        let mut i = nums.iter();
        for y in 0i32..5 {
            for x in 0i32..5 {
                numbers.insert((x, y), *i.next().unwrap());
            }
        }
        Self { numbers }
    }

    fn score(&self, marked: &HashSet<i32>) -> i32 {
        let mut result = 0;
        for n in self.numbers.values() {
            if !marked.contains(n) {
                result += *n;
            }
        }
        result
    }

    fn winning(&self, marked: &HashSet<i32>) -> bool {
        // Rows
        for y in 0..5 {
            let mut winning = true;
            for x in 0..5 {
                winning &= marked.contains(self.numbers.get(&(x, y)).unwrap());
                if !winning {
                    break;
                }
            }
            if winning {
                return true;
            }
        }

        // columns
        for x in 0..5 {
            let mut winning = true;
            for y in 0..5 {
                winning &= marked.contains(self.numbers.get(&(x, y)).unwrap());
                if !winning {
                    break;
                }
            }
            if winning {
                return true;
            }
        }

        false
    }
}

struct Puzzle {
    numbers: Vec<i32>,
    boards: Vec<BingoBoard>,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Result<Puzzle> {
    let mut lines = input.lines();
    // First is numbers
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut boards = vec![];

    // Now boards
    while lines.next().is_some() {
        // blank line
        let mut values = vec![];
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .for_each(|v| values.push(v));
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .for_each(|v| values.push(v));
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .for_each(|v| values.push(v));
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .for_each(|v| values.push(v));
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .for_each(|v| values.push(v));
        boards.push(BingoBoard::new(&values));
    }
    Ok(Puzzle { numbers, boards })
}

#[aoc(day4, part1)]
fn part1(input: &Puzzle) -> Result<i32> {
    let mut marked = HashSet::new();
    for turn in &input.numbers {
        marked.insert(*turn);
        for b in &input.boards {
            if b.winning(&marked) {
                return Ok(*turn * b.score(&marked));
            }
        }
    }
    panic!()
}

#[aoc(day4, part2)]
fn part2(input: &Puzzle) -> Result<i32> {
    let mut marked = HashSet::new();
    let mut already_won = HashSet::new();
    for turn in &input.numbers {
        marked.insert(*turn);
        let board_count = input.boards.len();
        for (bid, b) in input.boards.iter().enumerate() {
            if !already_won.contains(&bid) && b.winning(&marked) {
                if already_won.len() == board_count - 1 {
                    return Ok(*turn * b.score(&marked));
                } else {
                    already_won.insert(bid);
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMOKE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(4512, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1924, part2(&input)?);
        Ok(())
    }
}
