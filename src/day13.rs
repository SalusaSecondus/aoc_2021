use std::collections::HashSet;

use anyhow::{bail, Context, Result};
use regex::Regex;

type Coord = (usize, usize);

struct Input {
    dots: HashSet<Coord>,
    folds: Vec<(String, usize)>,
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Result<Input> {
    let mut dots = HashSet::new();
    let mut folds = vec![];

    let re = Regex::new("fold along (.)=(\\d+)")?;
    for l in input.lines() {
        if let Some((x, y)) = l.split_once(",") {
            dots.insert((x.parse()?, y.parse()?));
        } else if !l.is_empty() {
            let mat = re.captures(l).context("no match1")?;
            let dir = mat.get(1).context("No match2")?.as_str().to_string();
            let axis = mat.get(2).context("No match3")?.as_str().parse()?;
            folds.push((dir, axis));
        }
    }
    Ok(Input { dots, folds })
}

fn fold(dots: &HashSet<Coord>, fold: &(String, usize)) -> Result<HashSet<Coord>> {
    let mut result = HashSet::new();
    println!("Folding {:?}", fold);
    for dot in dots {
        match fold.0.as_str() {
            "y" => {
                if dot.1 < fold.1 {
                    result.insert(*dot);
                } else {
                    result.insert((dot.0, fold.1 + fold.1 - dot.1));
                }
            }
            "x" => {
                if dot.0 < fold.1 {
                    result.insert(*dot);
                } else {
                    result.insert((fold.1 + fold.1 - dot.0, dot.1));
                }
            }
            _ => bail!("Bad direction"),
        }
    }
    Ok(result)
}

#[allow(dead_code)]
fn print_dots(dots: &HashSet<Coord>) {
    let (max_x, max_y) = dots
        .iter()
        .copied()
        .reduce(|old, new| (old.0.max(new.0), (old.1.max(new.1))))
        .unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> Result<usize> {
    // print_dots(&input.dots);
    // println!();
    println!("Dot count: {}", input.dots.len());
    let result = fold(&input.dots, &input.folds[0])?;
    println!("Dot count: {}", result.len());

    // print_dots(&result);
    // let result = fold(&result, &input.folds[1])?;
    // print_dots(&result);

    Ok(result.len())
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> Result<usize> {
    let mut result = input.dots.clone();
    for f in &input.folds {
        result = fold(&result, f)?;
    }
    print_dots(&result);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(17, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        part2(&input)?;
        Ok(())
    }
}
