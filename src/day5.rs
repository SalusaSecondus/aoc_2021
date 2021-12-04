use anyhow::Result;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Vec<i32>> {
    let result = vec![];
    Ok(result)
}

#[aoc(day5, part1)]
fn part1(input: &[i32]) -> Result<i32> {
    Ok(1)
}

#[aoc(day5, part2)]
fn part2(input: &[i32]) -> Result<i32> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "";

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(1, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(2, part2(&input)?);
        Ok(())
    }
}
