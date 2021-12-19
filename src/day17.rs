use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug)]
struct Input {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Input {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Self {
        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            x_range,
            y_range,
        }
    }

    fn contains(&self, position: &(i32, i32)) -> bool {
        self.x_range.contains(&position.0) && self.y_range.contains(&position.1)
    }

    fn beyond(&self, position: &(i32, i32)) -> bool {
        position.0 > self.max_x || position.1 < self.min_y
    }
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Result<Input> {
    let re = Regex::new("target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)")?;
    let groups = re.captures(input).context("No match")?;
    let min_x = groups.get(1).unwrap().as_str().parse::<i32>()?;
    let max_x = groups.get(2).unwrap().as_str().parse::<i32>()?;
    let min_y = groups.get(3).unwrap().as_str().parse::<i32>()?;
    let max_y = groups.get(4).unwrap().as_str().parse::<i32>()?;

    Ok(Input::new(min_x, max_x, min_y, max_y))
}

fn step(velocity: &mut (i32, i32), position: &mut (i32, i32)) {
    position.0 += velocity.0;
    position.1 += velocity.1;

    if velocity.0 > 0 {
        velocity.0 -= 1;
    }
    velocity.1 -= 1;
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> Result<i32> {
    println!("Target: {:?}", input);

    let mut max_height = 0;
    for x in 0..input.max_x {
        for y in 0..=(2 * input.min_y.abs()) {
            // println!("Trying x={}, y={}", x, y);
            let mut position = (0, 0);
            let mut velocity = (x, y);
            let mut arc_max = 0;
            let mut count = 0;
            while !input.contains(&position) && !input.beyond(&position) {
                step(&mut velocity, &mut position);
                count += 1;
                // println!("\t({:?}", position);
                arc_max = arc_max.max(position.1);
            }
            if input.contains(&position) {
                max_height = max_height.max(arc_max);
                println!("Found solution: x={}, y={} @ {} ({})", x, y, count, arc_max);
            }
        }
    }
    // min starting velocity is min_x (+1?)
    // max starting velocity is max_x
    // min starting y is 0
    // max starting y is ???
    Ok(max_height)
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> Result<u64> {
    println!("Target: {:?}", input);
    let mut solutions = 0;
    for x in 0..=input.max_x {
        for y in 2 * input.min_y..=(2 * input.min_y.abs()) {
            // println!("Trying x={}, y={}", x, y);
            let mut position = (0, 0);
            let mut velocity = (x, y);
            let mut arc_max = 0;
            // let mut _count = 0;
            while !input.contains(&position) && !input.beyond(&position) {
                step(&mut velocity, &mut position);
                // _count += 1;
                // println!("\t({:?}", position);
                arc_max = arc_max.max(position.1);
            }
            if input.contains(&position) {
                solutions += 1;
                // println!("Found solution: x={}, y={} @ {} ({})", x, y, _count, arc_max);
            }
        }
    }
    Ok(solutions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(45, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(112, part2(&input)?);
        Ok(())
    }
}
