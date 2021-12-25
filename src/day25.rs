use anyhow::Result;

use crate::MatrixTranspose;

type Input = Vec<Vec<char>>;

#[aoc_generator(day25)]
fn input_generator(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
    let grid = grid.transpose();
    Ok(grid)
}

#[allow(dead_code)]
fn print_floor(floor: &Input) {
    let max_x = floor.len();
    let max_y = floor[0].len();
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", floor[x][y]);
        }
        println!();
    }
    println!();

}

fn step(floor: &Input, herd: char) -> (Input, usize) {
    let (x_off, y_off) = match herd {
        '>' => (1, 0),
        'v' => (0, 1),
        _ => panic!("Invalid herd")
    };
    let max_x = floor.len();
    let max_y = floor[0].len();
    let mut next_floor = vec![vec!['.'; max_y]; max_x];
    let mut move_count = 0;
    for (x, row) in floor.iter().enumerate() {
        for (y, spot) in row.iter().enumerate() {
            if *spot == herd {
                let (dest_x, dest_y) = (x + x_off, y + y_off);
                let dest_x = dest_x % max_x;
                let dest_y = dest_y % max_y;
                if floor[dest_x][dest_y] == '.' {
                    move_count += 1;
                    next_floor[dest_x][dest_y] = *spot;
                } else {
                    next_floor[x][y] = *spot;
                }
            } else if *spot != '.' {
                next_floor[x][y] = *spot;
            }
        }
    }
    (next_floor, move_count)
}

#[aoc(day25, part1)]
fn part1(input: &Input) -> Result<i32> {
    let mut floor = input.clone();
    // print_floor(&floor);

    let mut step_count = 0;
    loop {
        let step_1 = step(&floor, '>');
        // print_floor(&step_1.0);
        let step_2 = step(&step_1.0, 'v');
        step_count += 1;
        if step_1.1 + step_2.1 == 0 {
            break;
        }
        floor = step_2.0;
        // print_floor(&floor);
    }
    Ok(step_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(58, part1(&input)?);
        Ok(())
    }
}
