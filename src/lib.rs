use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Display,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader, Lines},
};

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
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

pub trait MatrixTranspose {
    fn transpose(&self) -> Self;
}

impl<T> MatrixTranspose for Vec<Vec<T>>
where
    T: Clone,
{
    #[allow(clippy::needless_range_loop)]
    fn transpose(&self) -> Self {
        let old_y = self.len();
        let old_x = self[0].len();
        let mut result = vec![vec![]; old_y];

        for y in 0..old_y {
            for x in 0..old_x {
                result[y].push(self[x][y].clone());
            }
        }
        result
    }
}

impl<K, V> MatrixTranspose for HashMap<(K, K), V>
where
    K: Copy + Eq + std::hash::Hash,
    V: Clone,
{
    fn transpose(&self) -> Self {
        let mut result = HashMap::new();
        for (k, v) in self {
            result.insert((k.1, k.0), v.clone());
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T>
where
    T: Clone + Eq + Hash,
{
    digraph: bool,
    map: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(digraph: bool) -> Self {
        Self {
            digraph,
            map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        if self.digraph {
            self.map.entry(to.clone()).or_default().push(from.clone());
        }
        self.map.entry(from).or_default().push(to);
    }

    pub fn nodes(&self) -> std::collections::hash_map::Keys<T, Vec<T>> {
        self.map.keys()
    }

    pub fn edges<Q: ?Sized>(&self, node: &Q) -> Option<&Vec<T>>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(node)
    }
}

impl<T> MatrixTranspose for Graph<T>
where
    T: Clone + Eq + Hash,
{
    fn transpose(&self) -> Self {
        if self.digraph {
            self.clone()
        } else {
            let mut result = Graph::new(self.digraph);
            for (node, edges) in &self.map {
                for dest in edges {
                    result.add_edge(dest.clone(), node.clone());
                }
            }
            result
        }
    }
}

impl<T> Graph<T>
where
    T: Ord + Clone + Eq + Hash,
{
    pub fn sort(&mut self) {
        self.map.values_mut().for_each(|v| v.sort_unstable());
    }
}

impl<T> Display for Graph<T>
where
    T: Clone + Eq + Hash + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.map {
            write!(f, "{} ->", e.0)?;
            for v in e.1.iter().enumerate() {
                if v.0 == 0 {
                    write!(f, " {}", v.1)?;
                } else {
                    write!(f, ", {}", v.1)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct ReplacementEngine<T>
where
    T: Hash + Eq + Clone,
{
    pub elements: HashMap<T, u64>,
    pub rules: HashMap<T, Vec<(T, u64)>>,
}

impl<T> ReplacementEngine<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new(elements: HashMap<T, u64>, rules: HashMap<T, Vec<(T, u64)>>) -> Self {
        Self { elements, rules }
    }

    pub fn step(&mut self) {
        let mut result = HashMap::new();
        for (node, count) in &self.elements {
            if let Some(replacements) = self.rules.get(node) {
                for r in replacements {
                    *result.entry(r.0.to_owned()).or_default() += *count * r.1;
                }
            }
        }
        self.elements = result;
    }
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
