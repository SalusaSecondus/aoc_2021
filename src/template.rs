use anyhow::{Context, Result};
use regex::Regex;

type Input = u32;

#[aoc_generator(day_template)]
fn input_generator(input: &str) -> Result<Input> {
    Ok(0)
}

#[aoc(day_template, part1)]
fn part1(input: &Input) -> Result<i32> {
    Ok(1)
}

#[aoc(day_template, part2)]
fn part2(input: &Input) -> Result<u64> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(2, part2(&input)?);
        Ok(())
    }
}
