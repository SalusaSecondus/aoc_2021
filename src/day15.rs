use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::Result;
use itertools::iproduct;

use crate::MatrixTranspose;

type Coord = (usize, usize);

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Result<Vec<Vec<i64>>> {
    let map: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| (*b - b'0') as i64).collect())
        .collect();
    let map = map.transpose();
    Ok(map)
}

fn calc_dist(map: &[Vec<i64>]) -> i64 {
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;
    let mut distences: HashMap<Coord, i64> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    distences.insert((0, 0), 0);
    while distences.len() != visited.len() {
        let (position, base_dist) = distences
            .iter()
            .filter(|p| !visited.contains(p.0))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        let position = position.to_owned();
        let base_dist = base_dist.to_owned();
        if position == (max_x, max_y) {
            return base_dist;
        }
        let x = position.0;
        let y = position.1;

        if x > 0 {
            let step = map[x - 1][y];
            let current = distences.entry((x - 1, y)).or_insert(i64::MAX);
            *current = (base_dist + step).min(*current);
        }
        if y > 0 {
            let step = map[x][y - 1];
            let current = distences.entry((x, y - 1)).or_insert(i64::MAX);
            *current = (base_dist + step).min(*current);
        }
        if x < max_x {
            let step = map[x + 1][y];
            let current = distences.entry((x + 1, y)).or_insert(i64::MAX);
            *current = (base_dist + step).min(*current);
        }
        if y < max_y {
            let step = map[x][y + 1];
            let current = distences.entry((x, y + 1)).or_insert(i64::MAX);
            *current = (base_dist + step).min(*current);
        }
        visited.insert((x, y));
    }

    panic!("Should never reach here");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapElement {
    position: Coord,
    distance: i64,
}

impl HeapElement {
    fn new(x: usize, y: usize, distance: i64) -> Self {
        let position = (x, y);
        Self { position, distance }
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then(other.position.0.cmp(&self.position.0))
            .then(other.position.1.cmp(&self.position.1))
    }
}

fn calc_dist2(map: &[Vec<i64>]) -> i64 {
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;
    let mut distances = vec![vec![i64::MAX; max_y + 1]; max_x + 1];
    let mut queue: BinaryHeap<HeapElement> = BinaryHeap::new();
    // let mut distences: HashMap<Coord, i64> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    distances[0][0] = 0;
    queue.push(HeapElement::new(0, 0, 0));
    while !queue.is_empty() {
        // if visited.len() % 1000 == 0 {
        //     println!("Visited {} nodes", visited.len());
        // }
        let current = queue.pop().unwrap();
        let position = current.position;
        let base_dist = current.distance;
        if position == (max_x, max_y) {
            return base_dist;
        }
        // If this is greater than the best known value, just skip it.
        if base_dist > distances[position.0][position.1] {
            continue;
        }
        let x = position.0;
        let y = position.1;

        if x > 0 {
            let step = map[x - 1][y];
            let current = distances[x - 1][y];
            if base_dist + step < current {
                distances[x - 1][y] = base_dist + step;
                queue.push(HeapElement::new(x - 1, y, base_dist + step))
            }
        }
        if y > 0 {
            let step = map[x][y - 1];
            let current = distances[x][y - 1];
            if base_dist + step < current {
                distances[x][y - 1] = base_dist + step;
                queue.push(HeapElement::new(x, y - 1, base_dist + step))
            }
        }
        if x < max_x {
            let step = map[x + 1][y];
            let current = distances[x + 1][y];
            if base_dist + step < current {
                distances[x + 1][y] = base_dist + step;
                queue.push(HeapElement::new(x + 1, y, base_dist + step))
            }
        }
        if y < max_y {
            let step = map[x][y + 1];
            let current = distances[x][y + 1];
            if base_dist + step < current {
                distances[x][y + 1] = base_dist + step;
                queue.push(HeapElement::new(x, y + 1, base_dist + step))
            }
        }
        visited.insert((x, y));
    }

    panic!("Should never reach here");
}

fn multiply_map(map: &[Vec<i64>], multiple: usize) -> Vec<Vec<i64>> {
    let x_len = map.len();
    let y_len = map[0].len();
    let mut result = vec![vec![0; multiple * y_len]; multiple * x_len];

    for (x, col) in map.iter().enumerate() {
        for (y, value) in col.iter().enumerate() {
            for (x_mul, y_mul) in iproduct!(0..multiple, 0..multiple) {
                let mut new_value = value + (x_mul as i64) + (y_mul as i64);
                if new_value > 9 {
                    new_value -= 9;
                }
                // println!("({}, {})={} -> ({}, {})={}", x, y, value, x + x_mul * x_len, y + y_mul * y_len, new_value);
                result[x + x_mul * x_len][y + y_mul * y_len] = new_value;
            }
        }
    }
    result
}

#[aoc(day15, part1)]
fn part1(map: &[Vec<i64>]) -> Result<i64> {
    Ok(calc_dist(map))
}

#[aoc(day15, part1, fast)]
fn part1_fast(map: &[Vec<i64>]) -> Result<i64> {
    Ok(calc_dist2(map))
}

#[aoc(day15, part2, fast)]
fn part2(input: &[Vec<i64>]) -> Result<i64> {
    let map = multiply_map(input, 5);
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;
    println!("Multiplied to {}x{}", max_x, max_y);
    Ok(calc_dist2(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(40, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke1_fast() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(40, part1_fast(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(315, part2(&input)?);
        Ok(())
    }
}
