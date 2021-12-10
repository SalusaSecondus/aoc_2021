use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use anyhow::{Context, Result};
#[macro_use]
extern crate aoc_runner_derive;

const FILE_BASE: &str = if cfg!(windows) { r"res\" } else { "res/" };

pub fn read_file(file_name: &str) -> Result<Lines<BufReader<File>>> {
    let input = File::open(FILE_BASE.to_owned() + file_name).context("Could not open file")?;
    let reader = BufReader::new(input);

    Ok(reader.lines())
}

#[allow(dead_code)]
fn load_numbers(file_name: &str) -> Result<Vec<i64>> {
    let mut result = vec![];

    for line in read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push(line.parse()?);
    }
    Ok(result)
}

#[aoc_generator(day1)]
fn day1_generator(input: &str) -> Vec<i32> {
    input.lines().map(|d| d.parse().unwrap()).collect()
    // vec![]
}

#[aoc(day1, part1)]
fn day1_1(input: &[i32]) -> usize {
    input.windows(2).map(|w| w[1] > w[0]).filter(|p| *p).count()
}

#[aoc(day1, part2)]
fn day1_2(input: &[i32]) -> usize {
    input.windows(4).map(|w| w[3] > w[0]).filter(|p| *p).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() -> Result<()> {
        let depths = load_numbers("day1.txt")?;
        let mut increases = 0;
        let mut previous = None;
        for d in &depths {
            if let Some(p) = previous {
                if *d > p {
                    increases += 1;
                }
            }
            previous = Some(*d)
        }
        assert_eq!(1681, increases);

        increases = 0;
        previous = None;
        for w in depths.windows(3) {
            let sum = w.iter().sum();
            if let Some(p) = previous {
                if sum > p {
                    increases += 1;
                }
            }
            previous = Some(sum);
        }
        assert_eq!(1704, increases);

        Ok(())
    }
}

aoc_lib! { year = 2021 }
