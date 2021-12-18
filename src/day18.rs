use std::{fmt::Display, iter::Peekable, ops::Add, str::FromStr};

use anyhow::Result;
use itertools::iproduct;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Entry {
    val: u8,
    depth: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SnailfishNumber(Vec<Entry>);

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for e in result.0.iter_mut() {
            e.depth += 1;
        }
        rhs.0.iter().for_each(|v| {
            let mut v = v.to_owned();
            v.depth += 1;
            result.0.push(v);
        });
        while result.reduce() {}
        result
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for e in result.0.iter_mut() {
            e.depth += 1;
        }
        rhs.0.iter().for_each(|v| {
            let mut v = v.to_owned();
            v.depth += 1;
            result.0.push(v);
        });
        while result.reduce() {}
        result
    }
}

impl SnailfishNumber {
    fn explode(&mut self, idx: usize) {
        let left = self.0.remove(idx);
        let right = self.0.remove(idx);
        if idx > 0 {
            // There is something to the left
            self.0[idx - 1].val += left.val;
        }
        if let Some(to_right) = self.0.get_mut(idx) {
            to_right.val += right.val;
        }
        self.0.insert(
            idx,
            Entry {
                val: 0,
                depth: left.depth - 1,
            },
        );
    }

    fn split(&mut self, idx: usize) {
        let val = self.0[idx].val;
        let depth = self.0[idx].depth + 1;
        let left = val / 2;
        let right = (val + 1) / 2;
        self.0[idx].depth = depth;
        self.0[idx].val = left;
        self.0.insert(idx + 1, Entry { val: right, depth });
    }

    fn reduce(&mut self) -> bool {
        for (idx, e) in self.0.iter().enumerate() {
            if e.depth > 4 {
                self.explode(idx);
                // println!("after explode:  {}", self);
                return true;
            }
        }
        for (idx, e) in self.0.iter().enumerate() {
            if e.val >= 10 {
                self.split(idx);
                // println!("after split:    {}", self);
                return true;
            }
        }
        false
    }

    fn magnitude(&self) -> u64 {
        let pair: SnailPair = self.into();
        pair.magnitude()
    }
}

impl FromStr for SnailfishNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depth = 0;
        let mut result = SnailfishNumber::default();
        for b in s.bytes() {
            match b {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' => (),
                _ => result.0.push(Entry {
                    val: b - b'0',
                    depth,
                }),
            };
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailPair {
    Value(u8),
    Pair(Box<SnailPair>, Box<SnailPair>),
}

impl From<&SnailfishNumber> for SnailPair {
    fn from(num: &SnailfishNumber) -> Self {
        let mut iter = num.0.iter().peekable();

        SnailPair::recursive_from(&mut iter, 0)
    }
}

impl SnailPair {
    fn recursive_from<'a>(
        num: &mut Peekable<impl Iterator<Item = &'a Entry>>,
        curr_depth: u32,
    ) -> SnailPair {
        let e = num.peek().unwrap();
        if e.depth > curr_depth {
            let left = Self::recursive_from(num, curr_depth + 1);
            let right = Self::recursive_from(num, curr_depth + 1);
            SnailPair::Pair(Box::new(left), Box::new(right))
        } else {
            SnailPair::Value(num.next().unwrap().val)
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            SnailPair::Value(v) => *v as u64,
            SnailPair::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl From<SnailPair> for SnailfishNumber {
    fn from(pair: SnailPair) -> Self {
        pair.to_string().parse().unwrap()
    }
}

impl Display for SnailPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailPair::Value(v) => write!(f, "{}", v),
            SnailPair::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pair: SnailPair = self.into();
        write!(f, "{}", pair)
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Result<Vec<SnailfishNumber>> {
    input
        .lines()
        .map(|l| l.parse::<SnailfishNumber>())
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[SnailfishNumber]) -> Result<u64> {
    let mut iter = input.iter();
    let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
    for val in iter {
        result = result + val.to_owned();
    }
    Ok(result.magnitude())
}

#[aoc(day18, part2)]
fn part2(input: &[SnailfishNumber]) -> Result<u64> {
    let mut max = 0;
    for (a, b) in iproduct!(input, input) {
        if a == b {
            continue;
        }
        max = max.max((a + b).magnitude());
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn puts() -> Result<()> {
        // println!("{:?}", "[1,2]".parse::<SnailfishNumber>()?);
        // println!("{:?}", "[[1,2],3]".parse::<SnailfishNumber>()?);
        // println!("{:?}", "[9,[8,7]]".parse::<SnailfishNumber>()?);
        // println!("{:?}", "[[1,9],[8,5]]".parse::<SnailfishNumber>()?);

        // let left: SnailfishNumber = "[1,2]".parse()?;
        // let right: SnailfishNumber = "[[3,4],5]".parse()?;
        // let sum = left.clone() + right.clone();
        // println!("{:?} + {:?} = {:?}", left, right, sum);

        // let mut val: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse()?;
        // assert!(val.reduce());
        // println!("{}", val);
        // let mut val: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse()?;
        // assert!(val.reduce());
        // println!("{}", val);
        // let mut val: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse()?;
        // assert!(val.reduce());
        // println!("{}", val);

        Ok(())
    }

    #[test]
    fn smoke1() -> Result<()> {
        let sum: SnailfishNumber =
            "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<SnailfishNumber>()? + "[1,1]".parse()?;
        // println!("{}", sum);
        // // while sum.reduce() {
        // //     println!("{}", sum);
        // // }
        let expected: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse()?;
        assert_eq!(expected, sum);

        let sum: SnailfishNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<SnailfishNumber>()?
            + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse()?;
        println!("{}", sum);
        let expected: SnailfishNumber =
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".parse()?;
        assert_eq!(expected, sum);

        let small: Vec<SnailfishNumber> = input_generator(
            "[1,1]
[2,2]
[3,3]
[4,4]",
        )?;

        let mut iter = small.iter();
        let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
        for val in iter {
            print!("  {}\n+ {}\n = ", result, val);
            result = result + val.to_owned();
            println!("{}", result);
        }
        let expected: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse()?;
        assert_eq!(expected, result);

        let small: Vec<SnailfishNumber> = input_generator(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
        )?;

        println!("\n\n\n");
        let mut iter = small.iter();
        let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
        for val in iter {
            print!("  {}\n+ {}\n= ", result, val);
            result = result + val.to_owned();
            println!("{}\n", result);
        }
        let expected: SnailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse()?;
        assert_eq!(expected, result);

        let small: Vec<SnailfishNumber> = input_generator(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
        )?;

        println!("\n\n\n");
        let mut iter = small.iter();
        let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
        for val in iter {
            print!("  {}\n+ {}\n= ", result, val);
            result = result + val.to_owned();
            println!("{}\n", result);
        }
        let expected: SnailfishNumber =
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse()?;
        assert_eq!(expected, result);

        let input: Vec<SnailfishNumber> = input_generator(SMOKE)?;

        let result = part1(&input)?;
        assert_eq!(4140, result);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(3993, part2(&input)?);
        Ok(())
    }
}
