use anyhow::{bail, Context, Result};
use itertools::{Itertools, MinMaxResult};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Result<Vec<i64>> {
    input
        .split(',')
        .map(|f| f.parse::<i64>().context("bad int"))
        .collect()
}

fn cost(positions: &[i64], target: i64) -> i64 {
    positions.iter().map(|p| (*p - target).abs()).sum()
}

fn cost2(positions: &[i64], target: i64) -> i64 {
    positions
        .iter()
        .map(|p| (*p - target).abs())
        .map(|diff| ((diff + 1) * diff) / 2)
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &[i64]) -> Result<i64> {
    if let MinMaxResult::MinMax(min, max) = input.iter().minmax() {
        let mut best = i64::MAX;
        for p in *min..*max {
            let c = cost(input, p);
            best = best.min(c);
        }
        return Ok(best);
    }
    bail!("Insufficient elements");
}

#[aoc(day7, part2)]
fn part2(input: &[i64]) -> Result<i64> {
    if let MinMaxResult::MinMax(min, max) = input.iter().minmax() {
        let mut best = i64::MAX;
        for p in *min..*max {
            let c = cost2(input, p);
            best = best.min(c);
        }
        return Ok(best);
    }
    bail!("Insufficient elements");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(37, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(168, part2(&input)?);
        Ok(())
    }
}
