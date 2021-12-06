use std::{collections::VecDeque, str::FromStr};

use anyhow::Result;

#[derive(Clone, Copy)]
struct Fish {
    phase: u8,
}

impl Fish {
    fn spawn(&mut self) -> Option<Fish> {
        if self.phase == 0 {
            self.phase = 7 - 1;
            Some(Fish { phase: 8 })
        } else {
            self.phase -= 1;
            None
        }
    }
}

impl FromStr for Fish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let phase: u8 = s.parse()?;
        Ok(Self { phase })
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Result<Vec<Fish>> {
    input.split(',').map(|f| f.parse::<Fish>()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Fish]) -> Result<usize> {
    let mut input = input.to_vec();
    for _day in 0..80 {
        // print!("Day {}:\t", _day);
        // for f in &input {
        //     print!("{},", f.phase);
        // }
        // println!();
        let mut new_fish = input.iter_mut().map(|f| f.spawn()).flatten().collect();
        input.append(&mut new_fish);
    }
    Ok(input.len())
}

fn fast_count(input: &[Fish], days: usize) -> Result<u64> {
    let mut counts = [0u64; 7];
    let mut next_counts = VecDeque::new();
    next_counts.extend(std::iter::repeat(0).take(9));

    for f in input {
        counts[f.phase as usize] += 1;
    }
    let mut ptr = 0;
    for _day in 0..days {
        if let Some(spawn) = next_counts.pop_front() {
            counts[ptr] += spawn;
        }
        // println!(
        //     "Day {}: Total {} ({:?}, {:?})",
        //     _day,
        //     counts.iter().sum::<u64>() + next_counts.iter().sum::<u64>(),
        //     counts,
        //     next_counts
        // );
        next_counts.push_back(counts[ptr]);
        ptr = (ptr + 1) % 7;
    }
    Ok(counts.iter().sum::<u64>() + next_counts.iter().sum::<u64>())
}

#[aoc(day6, part1, fast)]
fn part1_array(input: &[Fish]) -> Result<u64> {
    fast_count(input, 80)
}

#[aoc(day6, part2, fast)]
fn part2(input: &[Fish]) -> Result<u64> {
    fast_count(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "3,4,3,1,2";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(5934, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke1_array() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(5934, part1_array(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(26984457539u64, part2(&input)?);
        Ok(())
    }
}
