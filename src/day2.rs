use std::str::FromStr;

use anyhow::{bail, Context, Result};

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Default)]
struct Position(i32, i32, i32);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, value) = s.split_once(' ').context("Bad split")?;
        let value = value.parse()?;
        Ok(match dir {
            "forward" => Instruction::Forward(value),
            "up" => Instruction::Up(value),
            "down" => Instruction::Down(value),
            _ => bail!("Invalid instruction"),
        })
    }
}

impl Instruction {
    fn update_position(&self, pos: &Position) -> Position {
        match self {
            Instruction::Forward(value) => Position(pos.0 + value, pos.1, pos.2),
            Instruction::Up(value) => Position(pos.0, pos.1 - value, pos.2),
            Instruction::Down(value) => Position(pos.0, pos.1 + value, pos.2),
        }
    }

    fn update_position2(&self, pos: &Position) -> Position {
        match self {
            Instruction::Forward(value) => Position(pos.0 + value, pos.1 + pos.2 * value, pos.2),
            Instruction::Up(value) => Position(pos.0, pos.1, pos.2 - value),
            Instruction::Down(value) => Position(pos.0, pos.1, pos.2 + value),
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let mut position = Position::default();
    instructions
        .iter()
        .for_each(|i| position = i.update_position(&position));
    position.0 * position.1
}

#[aoc(day2, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    let mut position = Position::default();
    instructions
        .iter()
        .for_each(|i| position = i.update_position2(&position));
    position.0 * position.1
}

// #[cfg(test)]
// mod tests {
//     use anyhow::Context;

//     use crate::read_file;

//     use super::*;

//     #[test]
//     fn day2() -> Result<()> {
//         let mut pos = Position::default();
//         let mut pos2 = Position::default();
//         for l in read_file("day2.txt")? {
//             let l = l?;
//             let parts = l.split_once(' ').context("No space?")?;
//             let inst: Instruction = parts.0.parse()?;
//             pos = inst.update_position(parts.1.parse()?, &pos);
//             pos2 = inst.update_position2(parts.1.parse()?, &pos2);
//         }
//         println!("Day2.1: {}", pos.0 * pos.1);
//         println!("Day2.2: {}", pos2.0 * pos2.1);

//         Ok(())
//     }
// }
