use std::{collections::HashSet, fmt::Display};

use anyhow::{Context, Result};
use itertools::iproduct;

type Coord = (i32, i32);

#[derive(Debug, Clone)]
struct Image {
    enhancer: Vec<bool>,
    pixels: HashSet<Coord>,
    base_pixel: bool,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Image {
    fn min_max(pixels: &HashSet<Coord>) -> (Coord, Coord) {
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;

        for pixel in pixels {
            x_min = x_min.min(pixel.0);
            y_min = y_min.min(pixel.1);
            y_max = y_max.max(pixel.1);
            x_max = x_max.max(pixel.0);
        }
        // println!("Limits: {:?}", ((x_min, y_min), (x_max, y_max)));
        ((x_min, y_min), (x_max, y_max))
    }

    fn neighborhood(coord: &Coord) -> impl Iterator<Item = Coord> {
        iproduct!(coord.1 - 1..=coord.1 + 1, coord.0 - 1..=coord.0 + 1).map(|c| (c.1, c.0))
    }

    fn enhance2(&mut self) {
        let mut result = HashSet::new();
        for x in self.min_x - 1..=self.max_x + 1 {
            let mut idx = self.n2int(&mut Self::neighborhood(&(x, self.min_y - 2)));
            for y in self.min_y - 1..=self.max_y + 1 {
                let center = (x, y);
                // idx >>= 3;
                // let mut tmp_idx = 0;
                for offset_x in x - 1..=x + 1 {
                    idx <<= 1;
                    if self.read_pixel(&(offset_x, y + 1)) {
                        idx += 1;
                    }
                }
                // tmp_idx <<= 6;
                idx &= 0x01ff;
                // println!("\t=>{:?}", center);
                // assert_eq!(idx, self.n2int(&mut Self::neighborhood(&center)));
                if self.enhancer[idx] {
                    result.insert(center);
                }
            }
        }
        self.pixels = result;
        let ((min_x, min_y), (max_x, max_y)) = Self::min_max(&self.pixels);
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;

        if self.enhancer[0] && !self.enhancer[511] {
            self.base_pixel = !self.base_pixel;
        }
    }

    fn enhance(&mut self) {
        let mut result = HashSet::new();
        for center in iproduct!(
            self.min_x - 1..=self.max_x + 1,
            self.min_y - 1..=self.max_y + 1
        ) {
            let idx = self.n2int(&mut Self::neighborhood(&center));
            if self.enhancer[idx] {
                result.insert(center.to_owned());
            }
        }
        self.pixels = result;
        let ((min_x, min_y), (max_x, max_y)) = Self::min_max(&self.pixels);
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;

        if self.enhancer[0] && !self.enhancer[511] {
            self.base_pixel = !self.base_pixel;
        }
    }

    fn n2int(&self, iter: &mut impl Iterator<Item = Coord>) -> usize {
        let mut result = 0;
        for n in iter {
            result <<= 1;
            if self.read_pixel(&n) {
                result += 1;
            }
        }

        result
    }

    fn read_pixel(&self, coord: &Coord) -> bool {
        #[allow(clippy::if_same_then_else)]
        if self.base_pixel
            && (coord.0 < self.min_x
                || coord.0 > self.max_x
                || coord.1 < self.min_y
                || coord.1 > self.max_y)
        {
            return true;
        } else if self.pixels.contains(coord) {
            return true;
        }
        false
    }

    fn new(enhancer: Vec<bool>, pixels: HashSet<Coord>) -> Self {
        // println!("{:?}", pixels);
        let ((min_x, min_y), (max_x, max_y)) = Self::min_max(&pixels);
        let base_pixel = false;
        Self {
            enhancer,
            pixels,
            base_pixel,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.pixels.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Result<Image> {
    let mut lines = input.lines();
    let enhancer = lines
        .next()
        .context("No enhancer")?
        .chars()
        .map(|c| c == '#')
        .collect();

    lines.next(); // Blank line

    let mut pixels = HashSet::new();
    for (y, l) in lines.enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                pixels.insert((x as i32, y as i32));
            }
        }
    }
    Ok(Image::new(enhancer, pixels))
}

#[aoc(day20, part1)]
fn part1(input: &Image) -> Result<usize> {
    let mut image = input.to_owned();
    // let enhancer = image.enhancer.clone();
    // println!("Enhancer: {}", enhancer.iter().map(|b| match b { true => "#".to_string(), false => ".".to_string()}).reduce(|a, b| a + &b).unwrap());
    // println!("{}", image);
    image.enhance();
    // println!("{}", image);
    image.enhance();
    // println!("{}", image);
    Ok(image.pixels.len())
}

#[aoc(day20, part2)]
fn part2(input: &Image) -> Result<usize> {
    let mut image = input.to_owned();
    for _ in 0..50 {
        image.enhance();
    }
    Ok(image.pixels.len())
}

#[aoc(day20, part1, window)]
fn part1_window(input: &Image) -> Result<usize> {
    let mut image = input.to_owned();
    // let enhancer = image.enhancer.clone();
    // println!("Enhancer: {}", enhancer.iter().map(|b| match b { true => "#".to_string(), false => ".".to_string()}).reduce(|a, b| a + &b).unwrap());
    // println!("{}", image);
    image.enhance2();
    // println!("{}", image);
    image.enhance2();
    // println!("{}", image);
    Ok(image.pixels.len())
}

#[aoc(day20, part2, window)]
fn part2_window(input: &Image) -> Result<usize> {
    let mut image = input.to_owned();
    for _ in 0..50 {
        image.enhance2();
    }
    Ok(image.pixels.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_neighborhood() {
        let center = (5, 10);
        let mut grid = Image::neighborhood(&center);
        assert_eq!(Some((4, 9)), grid.next());
        assert_eq!(Some((5, 9)), grid.next());
        assert_eq!(Some((6, 9)), grid.next());
        assert_eq!(Some((4, 10)), grid.next());
        assert_eq!(Some((5, 10)), grid.next());
        assert_eq!(Some((6, 10)), grid.next());
        assert_eq!(Some((4, 11)), grid.next());
        assert_eq!(Some((5, 11)), grid.next());
        assert_eq!(Some((6, 11)), grid.next());
        assert_eq!(None, grid.next());
    }

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(35, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(3351, part2(&input)?);
        Ok(())
    }

    #[test]
    fn smoke1_window() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(35, part1_window(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2_window() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(3351, part2_window(&input)?);
        Ok(())
    }
}
