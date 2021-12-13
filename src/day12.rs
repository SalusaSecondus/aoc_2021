use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::Graph;

fn is_large(name: &str) -> bool {
    name.chars().all(|c| c.is_uppercase())
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Result<Graph<String>> {
    let mut cavern = Graph::new(true);
    for (cave1, cave2) in input.lines().flat_map(|l| l.split_once("-")) {
        cavern.add_edge(cave1.to_string(), cave2.to_string());
    }

    cavern.sort();
    Ok(cavern)
}

fn enum_paths(
    start: &str,
    cavern: &Graph<String>,
    visited_small: &mut HashSet<String>,
    depth: usize,
) -> Vec<Vec<String>> {
    // for _ in 0..depth {
    //     print!(" ");
    // }
    // println!("+Start {}", start);
    if start == "end" {
        return vec![vec!["end".to_string()]];
    }
    let mut result = vec![];
    for next in cavern.edges(start).unwrap() {
        // for _ in 0..depth {
        //     print!(" ");
        // }
        // println!("=Trying {}", next);
        if is_large(next) || visited_small.insert(next.to_string()) {
            for mut route in enum_paths(next, cavern, visited_small, depth + 1) {
                route.insert(0, start.to_string());
                // for _ in 0..depth {
                //     print!(" ");
                // }
                // println!("-Found route: {:?}", route);
                result.push(route);
            }
            visited_small.remove(next);
        }
    }
    result
}

fn enum_paths2(
    start: &str,
    cavern: &Graph<String>,
    visited_small: &mut HashMap<String, i32>,
    depth: usize,
) -> Vec<Vec<String>> {
    // for _ in 0..depth {
    //     print!(" ");
    // }
    // println!("+Start {}", start);
    if start == "end" {
        return vec![vec!["end".to_string()]];
    }
    let mut result = vec![];
    for next in cavern.edges(start).unwrap() {
        if next == "start" {
            continue;
        }
        // for _ in 0..depth {
        //     print!(" ");
        // }
        // println!("=Trying {}", next);
        if is_large(next) || can_fit(next, visited_small) {
            for mut route in enum_paths2(next, cavern, visited_small, depth + 1) {
                route.insert(0, start.to_string());
                // for _ in 0..depth {
                //     print!(" ");
                // }
                // println!("-Found route: {:?}", route);
                result.push(route);
            }
            if let Some(v) = visited_small.get_mut(next) {
                *v -= 1;
            }
        }
    }
    result
}

fn can_fit(name: &str, visited_small: &mut HashMap<String, i32>) -> bool {
    let revisit_banned = visited_small.values().any(|v| *v >= 2);
    // println!("Values: {:?}", visited_small.values());
    let entry = visited_small.entry(name.to_string()).or_default();
    let result = *entry < if revisit_banned { 1 } else { 2 };
    if result {
        *entry += 1;
    }
    // println!("\t{}({})->{}", name, revisit_banned, entry);
    result
}

#[aoc(day12, part1)]
fn part1(input: &Graph<String>) -> Result<usize> {
    let mut visited_small = HashSet::new();
    visited_small.insert("start".to_string());
    let paths = enum_paths("start", input, &mut visited_small, 0);

    // println!("Paths: {:?}", paths);
    Ok(paths.len())
}

#[aoc(day12, part2)]
fn part2(input: &Graph<String>) -> Result<usize> {
    let mut visited_small = HashMap::new();
    // visited_small.insert("start".to_string(), 2);
    let paths = enum_paths2("start", input, &mut visited_small, 0);

    // println!("Paths: {:?}", paths);
    Ok(paths.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        println!("Graph:\n{}", input);
        assert_eq!(10, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(36, part2(&input)?);
        Ok(())
    }
}
