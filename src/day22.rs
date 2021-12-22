use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{Context, Result};
use itertools::iproduct;
use regex::Regex;
use lazy_static::lazy_static;

type Coord = [i64; 3];

const INIT_RANGE: RangeInclusive<i64> = -50..=50;

#[derive(Debug, Clone)]
struct Cube {
    on: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new("\\s*(\\S+) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)").unwrap();
        };
        let captures = RE.captures(s).context("No match")?;
        let on = captures.get(1).context("Missing action")?.as_str() == "on";
        let x_min = captures.get(2).unwrap().as_str().parse()?;
        let x_max = captures.get(3).unwrap().as_str().parse()?;
        let y_min = captures.get(4).unwrap().as_str().parse()?;
        let y_max = captures.get(5).unwrap().as_str().parse()?;
        let z_min = captures.get(6).unwrap().as_str().parse()?;
        let z_max = captures.get(7).unwrap().as_str().parse()?;
        let x = x_min..=x_max;
        let y = y_min..=y_max;
        let z = z_min..=z_max;
        Ok(Self {on, x, y ,z})
    }
}

impl Cube {
    fn new(on: bool, x_min: i64, x_max: i64, y_min: i64, y_max: i64, z_min: i64, z_max: i64) -> Self {
        let x = x_min..=x_max;
        let y = y_min..=y_max;
        let z = z_min..=z_max;
        Self {on, x, y ,z}
    }

    fn contains(&self, point: &Coord) -> bool {
        self.x.contains(&point[0]) && self.y.contains(&point[1]) && self.z.contains(&point[2])
    }

    fn is_init(&self) -> bool {
        INIT_RANGE.contains(self.x.start()) &&
        INIT_RANGE.contains(self.x.end()) &&
        INIT_RANGE.contains(self.y.start()) &&
        INIT_RANGE.contains(self.y.end()) &&
        INIT_RANGE.contains(self.z.start()) &&
        INIT_RANGE.contains(self.z.end())
    }        

    fn disjoint(&self, other: &Cube) -> bool {
        self.x.start() > other.x.end() ||
        self.x.end() < other.x.start() ||
        self.y.start() > other.y.end() ||
        self.y.end() < other.y.start() ||
        self.z.start() > other.z.end() ||
        self.z.end() < other.z.start()
    }

    fn overlaps(&self, other: &Cube) -> bool {
        !self.disjoint(other)
    }

    fn volume(&self) -> i64 {
        (self.x.end() - self.x.start()) *
        (self.y.end() - self.y.start()) *
        (self.z.end() - self.z.start())
    }

    fn overlap(&self, other: &Cube) -> Option<Cube> {
        if self.disjoint(other) {
            return None;
        }
        // We must overlap

        let x_min = *self.x.start().max(other.x.start());
        let x_max = *self.x.end().min(other.x.end());
        let y_min = *self.y.start().max(other.y.start());
        let y_max = *self.y.end().min(other.y.end());
        let z_min = *self.x.start().max(other.z.start());
        let z_max = *self.x.end().min(other.z.end());
        
        let x = x_min..=x_max;
        let y = y_min..=y_max;
        let z = z_min..=z_max;
        Some(Self {x, y ,z, on: self.on })
    }

    fn merge(&self, other: &Cube) -> Option<Vec<Cube>> {
        let overlap = self.overlap(other);
        if overlap.is_none() {
            return None;
        }
        let overlap = overlap.unwrap();
        let mut result = vec![];
        if self.x.start() < overlap.x.start() {
            let x_min = *self.x.start();
            let x_max = overlap.x.start() - 1;
            result.push(Cube {x: x_min..=x_max, .. self.clone()});
        }
        if other.x.start() < overlap.x.start() {
            let x_min = *other.x.start();
            let x_max = overlap.x.start() - 1;
            result.push(Cube {x: x_min..=x_max, .. other.clone()});
        }
        if self.y.start() < overlap.y.start() {
            let y_min = *self.y.start();
            let y_max = overlap.y.start() - 1;
            result.push(Cube {y: y_min..=y_max, .. self.clone()});
        }
        if other.y.start() < overlap.y.start() {
            let y_min = *other.y.start();
            let y_max = overlap.y.start() - 1;
            result.push(Cube {y: y_min..=y_max, .. other.clone()});
        }
        if self.z.start() < overlap.z.start() {
            let z_min = *self.z.start();
            let z_max = overlap.z.start() - 1;
            result.push(Cube {z: z_min..=z_max, .. self.clone()});
        }
        if other.z.start() < overlap.z.start() {
            let z_min = *other.z.start();
            let z_max = overlap.z.start() - 1;
            result.push(Cube {z: z_min..=z_max, .. other.clone()});
        }

        if self.x.end() > overlap.x.end() {
            let x_min = overlap.x.end() + 1;
            let x_max = *self.x.start();
            result.push(Cube {x: x_min..=x_max, .. self.clone()});
        }
        if other.x.end() > overlap.x.end() {
            let x_min = overlap.x.end() + 1;
            let x_max = *other.x.start();
            result.push(Cube {x: x_min..=x_max, .. other.clone()});
        }
        if self.y.end() > overlap.y.end() {
            let y_min = overlap.y.end() + 1;
            let y_max = *self.y.start();
            result.push(Cube {y: y_min..=y_max, .. self.clone()});
        }
        if other.y.end() > overlap.y.end() {
            let y_min = overlap.y.end() + 1;
            let y_max = *other.y.start();
            result.push(Cube {y: y_min..=y_max, .. other.clone()});
        }
        if self.z.end() > overlap.z.end() {
            let z_min = overlap.z.end() + 1;
            let z_max = *self.z.start();
            result.push(Cube {z: z_min..=z_max, .. self.clone()});
        }
        if other.z.end() > overlap.z.end() {
            let z_min = overlap.z.end() + 1;
            let z_max = *other.z.start();
            result.push(Cube {z: z_min..=z_max, .. other.clone()});
        }
        result.push(overlap);
        Some(result)
    }
}

type Input = Vec<Cube>;

fn to_series(cubes: &Input) -> Vec<Vec<Cube>> {
    let mut result = vec![];

    let mut curr_status = true;
    let mut curr_series = vec![];
    for c in cubes {
        if c.on != curr_status {
            result.push(curr_series);
            curr_status = c.on;
            curr_series = vec![];
        }
        if curr_series.is_empty() {
            curr_series.push(c.to_owned());
        } else {
            let mut separated = vec![];
            while !curr_series.is_empty() {
                
            }
        }
    }
    result
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Result<Input> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> Result<i64> {
    let mut result = 0;
    for (x, y, z) in iproduct!(-50..=50, -50..=50, -50..=50) {
        let mut curr_status = false;
        let point = [x, y, z];
        for cube in input {
            if !cube.is_init() {
                break;
            }
            if cube.on == curr_status {
                continue;
            }
            if cube.contains(&point) {
                curr_status = !curr_status;
            }
        }
        if curr_status {
            result += 1;
        }
    }
    Ok(result)
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> Result<u64> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "on x=-20..26,y=-36..17,z=-47..7
    on x=-20..33,y=-21..23,z=-26..28
    on x=-22..28,y=-29..23,z=-38..16
    on x=-46..7,y=-6..46,z=-50..-1
    on x=-49..1,y=-3..46,z=-24..28
    on x=2..47,y=-22..22,z=-23..27
    on x=-27..23,y=-28..26,z=-21..29
    on x=-39..5,y=-6..47,z=-3..44
    on x=-30..21,y=-8..43,z=-13..34
    on x=-22..26,y=-27..20,z=-29..19
    off x=-48..-32,y=26..41,z=-47..-37
    on x=-12..35,y=6..50,z=-50..-2
    off x=-48..-32,y=-32..-16,z=-15..-5
    on x=-18..26,y=-33..15,z=-7..46
    off x=-40..-22,y=-38..-28,z=23..41
    on x=-16..35,y=-41..10,z=-47..6
    off x=-32..-23,y=11..30,z=-14..3
    on x=-49..-5,y=-3..45,z=-29..18
    off x=18..30,y=-20..-8,z=-3..13
    on x=-41..9,y=-7..43,z=-33..15
    on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
    on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(590784, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(2758514936282235, part2(&input)?);
        Ok(())
    }
}
