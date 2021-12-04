use anyhow::Result;

#[derive(Debug,Default)]
struct Entry {
    value: String,
    ones: usize,
    zeros: usize,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn count_bits(input: &[String]) -> Vec<i32> {
    let max_len = input[0].len();
    let mut values = vec![0; max_len];
    for v in input {
        let v = v.as_bytes();
        for p in 0..max_len {
            values[p] += match v[p] {
                b'1' => 1,
                b'0' => -1,
                _  => panic!("Unexpected value")
            };
        }
    }
    values
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    let values = count_bits(input);
    
    for b in &values {
        // gamma = gamma << 1;
        // epsilon = epsilon << 1;
        if *b > 0 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }
    println!("{}, {}", gamma, epsilon);
    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> i32 {
    let mut working = input.to_owned();
    let mut pos = 0;
    while working.len() != 1 {
        let values = count_bits(&working);
        let target = if values[pos] >= 0 {
            b'1'
        } else {
            b'0'
        };
        working = working.iter().filter(|v| v.as_bytes()[pos] == target).map(|v| v.to_owned()).collect();
        pos += 1;
    }
    let oxygen = working[0].clone();

        let mut working = input.to_owned();
    let mut pos = 0;
    while working.len() != 1 {
        let values = count_bits(&working);
        let target = if values[pos] < 0 {
            b'1'
        } else {
            b'0'
        };
        working = working.iter().filter(|v| v.as_bytes()[pos] == target).map(|v| v.to_owned()).collect();
        pos += 1;
    }
    let co2 = working[0].clone();

    println!("{} {}", oxygen, co2);

    i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&co2, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn part1_smoke() {
        assert_eq!(198, part1(&input_generator(SMOKE)));
    }

    #[test]
    fn part2_smoke() {
        assert_eq!(230, part2(&input_generator(SMOKE)));
    }
}