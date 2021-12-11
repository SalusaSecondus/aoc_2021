use std::collections::HashSet;

use anyhow::Result;
use itertools::iproduct;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Result<Vec<Vec<u8>>> {
    let result = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    Ok(result)
}

fn find_lows(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let height = input.len() as i32;
    let width = input[0].len() as i32;
    for (y, x) in iproduct!(0..height, 0..width) {
        let elevation = input[y as usize][x as usize];
        let mut low_point = true;
        // println!("({}, {})={}", x, y, elevation);
        for (x_off, y_off) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let other = input
                .get((y + y_off) as usize)
                .map(|row| row.get((x + x_off) as usize))
                .flatten();
            low_point &= other.map(|o| *o > elevation).unwrap_or(true);
            // println!("\t({},{})={:?}, {}", x + x_off, y + y_off, other, low_point);
        }

        // if x == 0 {
        //     println!();
        // }
        if low_point {
            result.push((x as usize, y as usize));
            // println!("({}, {})={} -> {}", x, y, elevation, low_point);
            // print!("{}", elevation.to_string().blue());
            // } else {
            //     print!("{}", elevation.to_string().red());
        }
    }
    // println!("{:?}", result);
    result
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<u8>]) -> Result<u64> {
    Ok(find_lows(input)
        .iter()
        .map(|(x, y)| input[*y][*x] as u64 + 1)
        .sum())
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<u8>]) -> Result<u64> {
    let low_points = find_lows(input);
    let mut sizes = vec![];

    for bottom in low_points {
        let mut visited = HashSet::new();
        let mut queue = vec![bottom];

        while let Some(current) = queue.pop() {
            visited.insert(current);
            // let elevation = input[current.1][current.0];
            for (x_off, y_off) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let new_x = (current.0 as i32 + x_off) as usize;
                let new_y = (current.1 as i32 + y_off) as usize;
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }
                let new_elevation = input
                    .get(new_y)
                    .map(|row| row.get(new_x))
                    .flatten()
                    .unwrap_or(&9);
                if *new_elevation != 9 {
                    queue.push((new_x, new_y));
                }
            }
        }

        sizes.push(visited.len() as u64);
    }

    sizes.sort_unstable();
    let mut result = 1u64;
    result *= sizes.pop().unwrap();
    result *= sizes.pop().unwrap();
    result *= sizes.pop().unwrap();
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(15, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1134, part2(&input)?);
        Ok(())
    }
}
