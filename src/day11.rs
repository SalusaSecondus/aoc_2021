use std::collections::HashSet;

use anyhow::Result;
use itertools::iproduct;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Result<Vec<Vec<i32>>> {
    let mut result = vec![vec![0; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            result[x][y] = (val - b'0') as i32;
        }
    }

    Ok(result)
}

#[allow(dead_code,clippy::needless_range_loop)]
fn print_octopuses(input: &[Vec<i32>]) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", input[x][y]);
        }
        println!();
    }
    println!();
}

fn one_round(input: &mut [Vec<i32>]) -> i32 {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = vec![];

    // Initial increase
    for (y, x) in iproduct!(0..10, 0..10) {
        input[x][y] += 1;
        let octopus = input[x][y];
        if octopus == 10 {
            queue.push((x, y));
            flashed.insert((x, y));
        }
    }
    // println!("Post increment");
    // print_octopuses(&input);
    while let Some(octopus_coord) = queue.pop() {
        // println!("\tFlashing: {:?}", octopus_coord);
        // This hasn't flashed before
        for (y_off, x_off) in iproduct!(-1i32..=1, -1i32..=1) {
            if x_off == 0 && y_off == 0 {
                continue;
            }
            let x = octopus_coord.0 as i32 + x_off;
            let y = octopus_coord.1 as i32 + y_off;
            if !(0..10).contains(&x) || !(0..10).contains(&y) {
                continue;
            }
            let x = x as usize;
            let y = y as usize;

            input[x][y] += 1;
            let octopus = input[x][y];
            // println!("Incrementing: {:?} -> {}", (x, y), octopus);
            if octopus == 10 && flashed.insert((x, y)) {
                // println!("\t\tAdded flash: {:?}", (x, y));
                queue.push((x, y));
                flashed.insert((x, y));
            }
        }
    }

    for octopus_coord in &flashed {
        input[octopus_coord.0][octopus_coord.1] = 0;
    }
    flashed.len() as i32
}

fn count_flashes(input: &[Vec<i32>], limit: usize) -> Result<i32> {
    let mut input = input.to_owned();
    let mut result = 0;

    for _round in 0..limit {
        // print_octopuses(&input);
        result += one_round(&mut input);
    }
    // print_octopuses(&input);
    Ok(result)
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<i32>]) -> Result<i32> {
    count_flashes(input, 100)
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<i32>]) -> Result<i32> {
    let mut input = input.to_owned();
    let mut round = 1;
    loop {
        if one_round(&mut input) == 100 {
            return Ok(round);
        }
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1656, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(195, part2(&input)?);
        Ok(())
    }
}
