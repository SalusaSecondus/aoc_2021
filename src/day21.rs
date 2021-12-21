use std::collections::HashMap;

use anyhow::{Context, Result};

trait Die {
    fn roll(&mut self) -> u16;
    fn roll_cnt(&self) -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct DeterministicDie(u8, u64);

impl Die for DeterministicDie {
    fn roll(&mut self) -> u16 {
        if self.0 == 100 {
            self.0 = 0;
        }
        self.0 += 1;
        self.1 += 1;
        self.0 as u16
    }

    fn roll_cnt(&self) -> u64 {
        self.1
    }
}

#[derive(Debug, Clone)]
struct DiracGame<D: Die> {
    players: Vec<u8>,
    die: D,
    scores: Vec<u64>,
    turn: usize,
}

impl<D: Die> DiracGame<D> {
    fn new(player1: u8, player2: u8, die: D) -> Self {
        let players = vec![player1, player2];
        let scores = vec![0; 2];
        Self {
            players,
            die,
            scores,
            turn: 0,
        }
    }

    fn go(&mut self) -> bool {
        let roll = self.die.roll() + self.die.roll() + self.die.roll();
        self.players[self.turn] += (roll % 10) as u8;
        if self.players[self.turn] > 10 {
            self.players[self.turn] -= 10;
        }
        self.scores[self.turn] += self.players[self.turn] as u64;
        let result = self.scores[self.turn] >= 1000;
        self.turn = (self.turn + 1) % self.players.len();
        result
    }
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
struct QuantumGameState {
    players: [u8; 2],
    scores: [u8; 2],
}

struct QuantumGame {
    states: HashMap<QuantumGameState, u64>,
    wins: [u64; 2],
    turn: usize,
}

const QUANTUM_SPLITS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl QuantumGame {
    fn new(input: &Input) -> Self {
        let state = QuantumGameState {
            players: [input.0, input.1],
            scores: [0; 2],
        };
        let mut states = HashMap::new();
        states.insert(state, 1u64);
        let wins = [0; 2];
        let turn = 0;
        Self { states, wins, turn }
    }

    fn go(&mut self) -> usize {
        let mut result: HashMap<QuantumGameState, u64> = HashMap::new();
        for (src, count) in &self.states {
            for split in QUANTUM_SPLITS {
                let mut dest = src.to_owned();
                dest.players[self.turn] += split.0;
                if dest.players[self.turn] > 10 {
                    dest.players[self.turn] -= 10;
                }
                dest.scores[self.turn] += dest.players[self.turn];
                if dest.scores[self.turn] >= 21 {
                    self.wins[self.turn] += count * split.1;
                } else {
                    *result.entry(dest).or_default() += count * split.1;
                }
            }
        }
        // if result.len() < 50 {
        //     println!("\t {:?}", result);
        // }
        self.turn = 1 - self.turn;
        self.states = result;

        self.states.len()
    }
}
type Input = (u8, u8);

#[aoc_generator(day21)]
fn input_generator(_input: &str) -> Result<Input> {
    Ok((9, 4))
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> Result<u64> {
    let mut game = DiracGame::new(input.0, input.1, DeterministicDie::default());
    // println!("{:?}", game);

    while !game.go() {
        // println!("{:?}", game);
    }
    // println!("{:?}", game);

    let loser_score = game.scores[game.turn];
    Ok(loser_score * game.die.roll_cnt())
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> Result<u64> {
    let mut game = QuantumGame::new(input);
    loop {
        let states = game.go();
        println!("Distinct states: {}", states);
        if states == 0 {
            break;
        }
    }
    game.wins.iter().copied().max().context("No scores?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke1() -> Result<()> {
        assert_eq!(739785, part1(&(4, 8))?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        assert_eq!(444356092776315u64, part2(&(4, 8))?);
        Ok(())
    }
}
