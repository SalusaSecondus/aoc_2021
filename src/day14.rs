use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use itertools::{Itertools, MinMaxResult};
use regex::Regex;

type Input = (String, HashMap<String, String>);

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let template = lines.next().context("Missing template")?.to_string();
    lines.next(); // Blank

    let re = Regex::new("(..) -> (.)")?;
    let mut rules = HashMap::new();
    for l in lines {
        let mat = re.captures(l).context("Missing rule")?;
        let source = mat.get(1).context("Missing source")?.as_str().to_string();
        let dest = mat.get(2).context("Missing dest")?.as_str().to_string();
        rules.insert(source, dest);
    }

    Ok((template, rules))
}

fn apply_rules(molecule: &str, rules: &HashMap<String, String>) -> Result<String> {
    let mut result = String::new();
    for idx in 0..molecule.len() - 1 {
        let key = &molecule[idx..idx + 2];
        let insert = rules.get(key).context("No rule")?;
        // println!("Replacing {} with {}{}{}", key, &molecule[idx..idx+1], insert, &molecule[idx + 1..idx+2]);
        result += &molecule[idx..idx + 1];
        result += insert;
        // result += &molecule[idx + 1..idx+2];
    }
    result += &molecule[molecule.len() - 1..molecule.len()];
    Ok(result)
}

fn str_to_pairs(molecule: &str) -> HashMap<(char, char), usize> {
    let chars: Vec<char> = molecule.chars().collect();
    let mut result = HashMap::new();
    chars
        .windows(2)
        .map(|w| (w[0], w[1]))
        .for_each(|w| *result.entry(w).or_default() += 1);
    result
}

fn rules_to_pairs(rules: &HashMap<String, String>) -> HashMap<(char, char), char> {
    let mut result = HashMap::new();
    for (src, dest) in rules {
        let mut src = src.chars();
        let mut dest = dest.chars();
        let key = (src.next().unwrap(), src.next().unwrap());
        let value = dest.next().unwrap();
        result.insert(key, value);
    }
    result
}
fn apply_rules_to_pairs(
    molecule: &HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();
    for (key, count) in molecule {
        let insert = rules.get(key).unwrap();
        let pair_1 = (key.0, *insert);
        let pair_2 = (*insert, key.1);
        *result.entry(pair_1).or_default() += count;
        *result.entry(pair_2).or_default() += count;
    }

    result
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> Result<usize> {
    let mut template = input.0.clone();
    // println!("After step 0: {}", template);

    for _step in 1..=10 {
        template = apply_rules(&template, &input.1)?;
        // println!("After step {}: {}", _step, template);
    }
    let counts = template.chars().counts();
    let extremes = counts.iter().minmax_by(|a, b| a.1.cmp(b.1));
    if let MinMaxResult::MinMax(min, max) = extremes {
        Ok(max.1 - min.1)
    } else {
        bail!("No min/max");
    }
}

fn pair_iteration(input: &Input, steps: usize) -> Result<usize> {
    let mut pairs = str_to_pairs(&input.0);
    let rules = rules_to_pairs(&input.1);

    for _step in 1..=steps {
        pairs = apply_rules_to_pairs(&pairs, &rules);
    }

    let mut counts = HashMap::new();
    for (p, c) in pairs {
        // The first elements are always the second element of something except the very first which we special case
        *counts.entry(p.1).or_default() += c;
    }

    let mut template = input.0.chars();
    let first = template.next().unwrap();
    // let last = template.last().unwrap();
    *counts.entry(first).or_default() += 1;

    // Avoid double counting first and last element. All others show up twice

    let extremes = counts
        .iter()
        .minmax_by(|a: &(&char, &usize), b: &(&char, &usize)| a.1.cmp(b.1));
    if let MinMaxResult::MinMax(min, max) = extremes {
        Ok(max.1 - min.1)
    } else {
        bail!("No min/max");
    }
}

#[aoc(day14, part1, pairs)]
fn part1_pairs(input: &Input) -> Result<usize> {
    pair_iteration(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> Result<usize> {
    pair_iteration(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1588, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke1_pairs() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1588, part1_pairs(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(2188189693529, part2(&input)?);
        Ok(())
    }
}
