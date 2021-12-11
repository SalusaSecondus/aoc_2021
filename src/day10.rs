use anyhow::{bail, Result};

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input.lines().map(|l| l.chars().collect()).collect())
}

fn find_illegal_char(line: &[char]) -> Option<char> {
    let mut stack = vec![];
    for c in line {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(old) = stack.pop() {
                    if *c
                        != match *old {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("Illegal state"),
                        }
                    {
                        return Some(*c);
                    }
                } else {
                    panic!("Unexpected close")
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    None
}

fn find_close(line: &[char]) -> Option<Vec<char>> {
    let mut stack = vec![];
    for c in line {
        match c {
            '(' | '[' | '{' | '<' => stack.push(*c),
            ')' | ']' | '}' | '>' => {
                if let Some(old) = stack.pop() {
                    if *c
                        != match old {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("Illegal state"),
                        }
                    {
                        return None;
                    }
                } else {
                    panic!("Unexpected close")
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    Some(stack)
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<char>]) -> Result<i32> {
    let mut result = 0;
    for bad_char in input.iter().flat_map(|l| find_illegal_char(l)) {
        result += match bad_char {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => bail!("Invalid char"),
        };
    }
    Ok(result)
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<char>]) -> Result<i32> {
    let mut scores = vec![];
    for mut incomplete in input.iter().flat_map(|l| find_close(l)) {
        let mut curr_score = 0;

        while let Some(next) = incomplete.pop() {
            curr_score *= 5;
            curr_score += match next {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => bail!("Invalid char"),
            };
            println!("Popping {} -> {}", next, curr_score);
        }
        println!();
        scores.push(curr_score);
    }
    scores.sort_unstable();
    let middle = (scores.len() - 1) / 2;
    Ok(scores[middle])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(26397, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(288957, part2(&input)?);
        Ok(())
    }
}
