use std::{ops::Add, str::FromStr, iter::Sum};

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Entry {
    val: u8,
    depth: u32
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SnailfishNumber (Vec<Entry>);

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for e in result.0.iter_mut() {
            e.depth += 1;
        }
        rhs.0.iter().for_each(|v| {let mut v = v.to_owned(); v.depth += 1; result.0.push(v);});
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
        self.0.insert(idx, Entry{val: 0, depth: left.depth - 1});
    }

    fn split(&mut self, idx: usize) {
        let val = self.0[idx].val;
        let depth = self.0[idx].depth + 1;
        let left = val / 2;
        let right = (val + 1) / 2;
        self.0[idx].depth = depth;
        self.0[idx].val = left;
        self.0.insert(idx + 1, Entry { val: right, depth});
    }

    fn reduce(&mut self) -> bool {
        for (idx, e) in self.0.iter().enumerate() {
            if e.depth > 4 {
                self.explode(idx);
                return true;
            }
            if e.val >= 10 {
                self.split(idx);
                return true;
            }
        }
        false
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
                _ => result.0.push(Entry{val: b - b'0', depth}),
            };
        }
        Ok(result)
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Result<Vec<SnailfishNumber>> {
    input.lines().map(|l| l.parse::<SnailfishNumber>()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[SnailfishNumber]) -> Result<i32> {
    Ok(1)
}

#[aoc(day18, part2)]
fn part2(input: &[SnailfishNumber]) -> Result<u64> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "";

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

        let mut val: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse()?;
        assert!(val.reduce());
        println!("{:?}", val);
        let mut val: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse()?;
        assert!(val.reduce());
        println!("{:?}", val);
        let mut val: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse()?;
        assert!(val.reduce());
        println!("{:?}", val);
        Ok(())
    }

    #[test]
    fn smoke1() -> Result<()> {
        let mut sum: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<SnailfishNumber>()? + "[1,1]".parse()?;
        println!("{:?}", sum);
        while sum.reduce() {
            println!("{:?}", sum);
        }
        let expected: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse()?;
        assert_eq!(expected, sum);

        let small: Vec<SnailfishNumber> = input_generator("[1,1]
[2,2]
[3,3]
[4,4]")?;

        let mut iter = small.iter();
        let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
        for val in iter {
            result = result + val.to_owned();
        }
        let expected: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse()?;
        assert_eq!(expected, result);

        let small: Vec<SnailfishNumber> = input_generator("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]")?;

        let mut iter = small.iter();
        let mut result: SnailfishNumber = iter.next().unwrap().to_owned();
        for val in iter {
            result = result + val.to_owned();
        }
        let expected: SnailfishNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse()?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(2, part2(&input)?);
        Ok(())
    }
}
