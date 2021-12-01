use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::{Context, Result};

const FILE_BASE: &str = r"res\";

pub fn read_file(file_name: &str) -> Result<Lines<BufReader<File>>> {
    let input = File::open(FILE_BASE.to_owned() + file_name).context("Could not open file")?;
    let reader = BufReader::new(input);

    Ok(reader.lines())
}

#[allow(dead_code)]
fn load_numbers(file_name: &str) -> Result<Vec<i64>> {
    let mut result = vec![];

    for line in read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push(line.parse()?);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() -> Result<()> {
        let depths = load_numbers("day1.txt")?;
        let mut increases = 0;
        let mut previous = None;
        for d in &depths {
            if let Some(p) = previous {
                if *d > p {
                    increases += 1;
                }
            }
            previous = Some(*d)
        }
        assert_eq!(1681, increases);

        increases = 0;
        previous = None;
        for w in depths.windows(3) {
            let sum = w.iter().sum();
            if let Some(p) = previous {
                if sum > p {
                    increases += 1;
                }
            }
            previous = Some(sum);
        }
        assert_eq!(1704, increases);

        Ok(())
    }
}
