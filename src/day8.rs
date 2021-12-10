use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    vec,
};

use anyhow::{Context, Result};

struct Input {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("|").context("No pipe")?;
        let patterns = left
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let output = right
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        Ok(Self { patterns, output })
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Result<Vec<Input>> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Input]) -> Result<i32> {
    let mut ones = 0;
    let mut fours = 0;
    let mut sevens = 0;
    let mut eights = 0;
    for i in input {
        for o in &i.output {
            match o.len() {
                2 => ones += 1,
                4 => fours += 1,
                3 => sevens += 1,
                7 => eights += 1,
                _ => (),
            };
        }
    }
    Ok(ones + fours + sevens + eights)
}

#[aoc(day8, part2)]
fn part2(input: &[Input]) -> Result<i32> {
    let mut decoder = HashMap::new();
    decoder.insert("abcefg".to_string(), 0);
    decoder.insert("cf".to_string(), 1);
    decoder.insert("acdeg".to_string(), 2);
    decoder.insert("acdfg".to_string(), 3);
    decoder.insert("bcdf".to_string(), 4);
    decoder.insert("abdfg".to_string(), 5);
    decoder.insert("abdefg".to_string(), 6);
    decoder.insert("acf".to_string(), 7);
    decoder.insert("abcdefg".to_string(), 8);
    decoder.insert("abcdfg".to_string(), 9);
    let decoder = decoder;
    let mut result = 0;

    let mut all_segments = HashSet::new();
    for c in &['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        all_segments.insert(c);
    }
    // let all_segments = all_segments;
    for i in input {
        let mut segments = HashMap::new();
        segments.insert('a', all_segments.clone());
        segments.insert('b', all_segments.clone());
        segments.insert('c', all_segments.clone());
        segments.insert('d', all_segments.clone());
        segments.insert('e', all_segments.clone());
        segments.insert('f', all_segments.clone());
        segments.insert('g', all_segments.clone());

        for p in &i.patterns {
            for c in &all_segments {
                let c = *c;
                if let Some(remaining) = possible_segments(p) {
                    // println!("{} {} {:?} {:?}", c, p, segments.get(c), remaining);
                    segments
                        .get_mut(c)
                        .unwrap()
                        .retain(|v| p.contains(*c) == remaining.contains(v));
                    // println!("{} {} {:?}", c, p, segments.get(c));
                } else if p.len() == 6 {
                    let remaining = vec!['c', 'd', 'e'];
                    segments
                        .get_mut(c)
                        .unwrap()
                        .retain(|v| p.contains(*c) || remaining.contains(v));
                }
            }

            loop {
                let mut reduced = false;
                for c in &all_segments {
                    let remaining = segments.get(c).unwrap().clone();
                    if remaining.len() == 1 {
                        let to_remove = remaining.iter().next().unwrap();
                        // println!("Removing {} from everything but {}", to_remove, c);
                        for c_prime in &all_segments {
                            if c == c_prime {
                                continue;
                            }
                            if segments.get_mut(c_prime).unwrap().remove(to_remove) {
                                reduced = true;
                            }
                        }
                    }
                }
                if !reduced {
                    break;
                }
            }
        }
        // At this point segments is properly decoded
        let segments: HashMap<char, char> = segments
            .iter()
            .map(|(k, v)| (*k, **v.iter().next().unwrap()))
            .collect();
        let mut entry = 0;
        for o in &i.output {
            entry *= 10;
            let mut translated_entry: Vec<char> =
                o.chars().map(|c| *segments.get(&c).unwrap()).collect();
            translated_entry.sort_unstable();
            let translated_entry = translated_entry
                .iter()
                .fold(String::new(), |acc, x| acc + &x.to_string());
            entry += decoder.get(&translated_entry).unwrap();
        }
        result += entry;
    }
    Ok(result)
}

fn possible_segments(code: &str) -> Option<Vec<char>> {
    match code.len() {
        2 => Some(vec!['c', 'f']),
        3 => Some(vec!['c', 'f', 'a']),
        4 => Some(vec!['b', 'c', 'd', 'f']),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(26, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )?;
        assert_eq!(5353, part2(&input)?);
        let input = input_generator(SMOKE)?;
        assert_eq!(61229, part2(&input)?);
        Ok(())
    }
}
