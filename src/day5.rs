use std::str::FromStr;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

type Position = (i32, i32);

#[derive(Debug, Default, PartialEq, Eq)]
struct Line {
    start: Position,
    end: Position,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$").unwrap();
        };
        let mat = RE.captures(s).context("No match")?;
        let start = (
            mat.get(1).context("No match")?.as_str().parse()?,
            mat.get(2).context("No match")?.as_str().parse()?,
        );
        let end = (
            mat.get(3).context("No match")?.as_str().parse()?,
            mat.get(4).context("No match")?.as_str().parse()?,
        );
        Ok(Self { start, end })
    }
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_diagonal(&self) -> bool {
        !(self.is_horizontal() || self.is_vertical())
    }
    fn contains(&self, point: &Position) -> bool {
        let min_x = self.start.0.min(self.end.0);
        let max_x = self.start.0.max(self.end.0);
        let min_y = self.start.1.min(self.end.1);
        let max_y = self.start.1.max(self.end.1);
        if self.is_horizontal() && point.1 != self.start.1 {
            return false;
        }
        if self.is_vertical() && point.0 != self.start.0 {
            return false;
        }
        if self.start == (8, 0) && self.end == (0, 8) {
            println!(
                "Point: {:?} xdiff {} ydiff {}",
                point,
                point.0 - self.start.0,
                point.1 - self.start.1
            );
        }
        if self.is_diagonal() && (point.0 - self.start.0).abs() != (point.1 - self.start.1).abs() {
            return false;
        }
        point.0 >= min_x && point.0 <= max_x && point.1 >= min_y && point.1 <= max_y
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Vec<Line>> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> Result<i32> {
    let min_x = 0;
    let min_y = 0;
    let max_x = input.iter().map(|l| l.start.0).max().context("No values")?;
    let max_x = max_x.max(input.iter().map(|l| l.end.0).max().context("No values")?);
    let max_y = input.iter().map(|l| l.start.1).max().context("No values")?;
    let max_y = max_y.max(input.iter().map(|l| l.end.1).max().context("No values")?);

    let mut result = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let hits = input
                .iter()
                .filter(|l| !l.is_diagonal())
                .filter(|l| l.contains(&(x, y)))
                .count();
            // if hits > 0 {
            //     print!("{}", hits);
            // } else {
            //     print!(".");
            // }
            if hits > 1 {
                // println!("Point ({}, {})", x, y);
                result += 1;
            }
        }
        // println!();
    }
    Ok(result)
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> Result<i32> {
    // println!("Lines: {:?}", input);
    let min_x = 0;
    let min_y = 0;
    let max_x = input.iter().map(|l| l.start.0).max().context("No values")?;
    let max_x = max_x.max(input.iter().map(|l| l.end.0).max().context("No values")?);
    let max_y = input.iter().map(|l| l.start.1).max().context("No values")?;
    let max_y = max_y.max(input.iter().map(|l| l.end.1).max().context("No values")?);

    let mut result = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let hits = input.iter().filter(|l| l.contains(&(x, y))).count();
            // if hits > 0 {
            //     print!("{}", hits);
            // } else {
            //     print!(".");
            // }
            if hits > 1 {
                // println!("Point ({}, {})", x, y);
                result += 1;
            }
        }
        // println!();
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(5, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(12, part2(&input)?);
        Ok(())
    }
}
