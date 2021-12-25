use std::collections::HashSet;

use anyhow::{ensure, Result};

#[aoc(day24, part1)]
fn part1(_: &str) -> Result<i64> {
    run(true)
}

#[aoc(day24, part2)]
fn part2(_: &str) -> Result<i64> {
    run(false)
}

pub fn run(reverse: bool) -> Result<i64> {
    let mut serial = [0; 14];
    let registers = [0; 4];
    let mut seen = HashSet::new();
    let digits = if reverse {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    ensure!(driver(&digits, 0, &mut serial, registers, &mut seen));

    let mut result = 0;
    for digit in serial {
        result *= 10;
        result += digit as i64;
    }
    Ok(result)
}

pub fn driver(
    digits: &[u8],
    step: u8,
    serial: &mut [u8; 14],
    registers: [i64; 4],
    seen: &mut HashSet<(u8, [i64; 4])>,
) -> bool {
    if !seen.insert((step, registers)) {
        return false;
    }

    for digit in digits.iter() {
        let mut working_copy = registers.clone();
        if step == 1 {
            println!("Candidate: {:?} -> {:?}", serial, registers);
        }
        serial[step as usize] = *digit;
        checksum(step, serial, &mut working_copy);
        if step == 13 && working_copy[3] == 0 {
            return true;
        }
        if step < 13 {
            if driver(digits, step + 1, serial, working_copy, seen) {
                return true;
            }
        }
    }
    false
}

pub fn checksum(step: u8, serial: &[u8], registers: &mut [i64; 4]) {
    match step {
        0 => {
            registers[0] = serial[0] as i64;
            registers[2] = registers[0] + 12;
            registers[3] = registers[2];
            // Shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        1 => {
            registers[0] = serial[1] as i64;

            registers[2] = registers[0] + 7;
            registers[3] = registers[3] + registers[2];

            // Shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        2 => {
            registers[0] = serial[2] as i64;

            registers[2] = registers[0] + 1;
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        3 => {
            registers[0] = serial[3] as i64;

            registers[2] = registers[0];
            registers[2] = registers[2] + 2;
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -5;
            registers[2] = 25;
        }
        4 => {
            registers[0] = serial[4] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };

            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 4;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        5 => {
            registers[0] = serial[5] as i64;
            registers[2] = registers[0];
            registers[2] = registers[2] + 15;
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        6 => {
            registers[0] = serial[6] as i64;
            registers[2] = registers[0];
            registers[2] = registers[2] + 11;
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -13;
            registers[2] = 25;
        }
        7 => {
            registers[0] = serial[7] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 5;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -16;
            registers[2] = 25;
        }
        8 => {
            registers[0] = serial[8] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 3;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -8;
            registers[2] = 25;
        }
        9 => {
            registers[0] = serial[9] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 9;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[2] = 26;
            registers[3] = registers[3] * registers[2];
        }
        10 => {
            registers[0] = serial[10] as i64;
            registers[2] = registers[0];
            registers[2] = registers[2] + 2;
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -8;
            registers[2] = 25;
        }
        11 => {
            registers[0] = serial[11] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 3;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[2] = 25;
        }
        12 => {
            registers[0] = serial[12] as i64;
            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 3;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];

            // shifted
            registers[1] = registers[3];
            registers[1] = registers[1] % 26;
            registers[3] = registers[3] / 26;
            registers[1] = registers[1] + -4;
            registers[2] = 25;
        }
        13 => {
            registers[0] = serial[13] as i64;

            registers[1] = if registers[1] == registers[0] { 0 } else { 1 };
            registers[2] = registers[2] * registers[1];
            registers[2] = registers[2] + 1;
            registers[3] = registers[3] * registers[2];
            registers[2] = registers[0];
            registers[2] = registers[2] + 11;
            registers[2] = registers[2] * registers[1];
            registers[3] = registers[3] + registers[2];
        }
        _ => {}
    }
}
