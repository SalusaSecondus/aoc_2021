use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn apply(&self, contents: &[Packet]) -> Result<u64> {
        let values: Vec<u64> = contents
            .into_iter()
            .map(|p| p.value())
            .collect::<Result<Vec<u64>>>()?;
        match self {
            Operator::Sum => Ok(values.into_iter().sum()),
            Operator::Product => Ok(values.into_iter().product()),
            Operator::Minimum => values.into_iter().min().context("Insufficient values"),
            Operator::Maximum => values.into_iter().max().context("Insufficient values"),
            Operator::GreaterThan => {
                if values[0] > values[1] {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            Operator::LessThan => {
                if values[0] < values[1] {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            Operator::EqualTo => {
                if values[0] == values[1] {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => bail!("Invalid value"),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u8, u64),
    Operator(u8, Operator, Vec<Packet>),
}

impl Packet {
    fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Packet> {
        let version = bits_to_int(iter, 3)? as u8;
        let id = bits_to_int(iter, 3)? as u8;

        // println!("{} {}", version, id);
        if id == 4 {
            let mut value = 0u64;
            let mut last = false;
            while !last {
                last = read_bit(iter)? == 0;
                value <<= 4;
                value += bits_to_int(iter, 4)? as u64;
            }
            Ok(Packet::Literal(version, value))
        } else {
            let length_type = read_bit(iter)?;
            let mut contents = vec![];
            if length_type == 0 {
                let bit_length = bits_to_int(iter, 15)? as usize;
                #[allow(clippy::needless_collect)] // Prevents infinite recursion of types
                let nested_bits: Vec<u8> = iter.take(bit_length).collect();
                let mut nested_iter = nested_bits.into_iter();
                // iter.skip(bit_length);
                while let Ok(nested) = Packet::from_iter(&mut nested_iter) {
                    contents.push(nested);
                }
            } else if length_type == 1 {
                let packet_count = bits_to_int(iter, 11)? as usize;
                while contents.len() < packet_count {
                    contents.push(Packet::from_iter(iter)?);
                }
            } else {
                panic!("Impossible");
            }
            Ok(Packet::Operator(version, id.try_into()?, contents))
        }
    }

    fn value(&self) -> Result<u64> {
        match self {
            Packet::Literal(_, value) => Ok(*value),
            Packet::Operator(_, operator, contents) => operator.apply(contents),
        }
    }
}

fn read_bit(iter: &mut impl Iterator<Item = u8>) -> Result<u8> {
    iter.next().context("Insufficient bits")
}

fn bits_to_int(iter: &mut impl Iterator<Item = u8>, len: usize) -> Result<u64> {
    let mut result = 0u64;
    for _ in 0..len {
        result <<= 1;
        result += read_bit(iter)? as u64;
    }
    Ok(result)
}

fn parse_nibble(nibble: char) -> u8 {
    // println!("N={}", nibble);
    if ('0'..='9').contains(&nibble) {
        nibble as u8 - b'0'
    } else if ('A'..='F').contains(&nibble) {
        nibble as u8 + 10 - b'A'
    } else {
        panic!("Invalid nibble")
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Result<Packet> {
    let mut bytes = vec![];
    for mut chunk in &input.chars().map(parse_nibble).into_iter().chunks(2) {
        let mut byte = chunk.next().unwrap() << 4;
        if let Some(next) = chunk.next() {
            byte += next;
        }
        bytes.push(byte);
    }
    let mut iter = bit_iterator(&bytes);
    Packet::from_iter(&mut iter)
}

fn byte_to_bits(byte: u8) -> [u8; 8] {
    let mut result = [0; 8];
    let mut byte = byte;
    for idx in (0..8).rev() {
        result[idx] = byte & 1;
        byte >>= 1;
    }
    result
}

fn bit_iterator(bytes: &[u8]) -> impl Iterator<Item = u8> + '_ {
    bytes.iter().flat_map(|b| byte_to_bits(*b))
}

#[aoc(day16, part1)]
fn part1(packet: &Packet) -> Result<i64> {
    match packet {
        Packet::Literal(version, _) => Ok(*version as i64),
        Packet::Operator(version, _, contents) => {
            let inner_sum: i64 = contents.iter().map(|p| part1(p)).flatten().sum();
            Ok(inner_sum + *version as i64)
        }
    }
}

#[aoc(day16, part2)]
fn part2(input: &Packet) -> Result<u64> {
    input.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values() -> Result<()> {
        assert_eq!(Packet::Literal(6, 2021), input_generator("D2FE28")?);
        assert_eq!(
            Packet::Operator(
                1,
                Operator::LessThan,
                vec![Packet::Literal(6, 10), Packet::Literal(2, 20)]
            ),
            input_generator("38006F45291200")?
        );
        assert_eq!(
            Packet::Operator(
                7,
                Operator::Maximum,
                vec![
                    Packet::Literal(2, 1),
                    Packet::Literal(4, 2),
                    Packet::Literal(1, 3)
                ]
            ),
            input_generator("EE00D40C823060")?
        );

        Ok(())
    }

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator("8A004A801A8002F478")?;
        assert_eq!(16, part1(&input)?);
        let input = input_generator("620080001611562C8802118E34")?;
        assert_eq!(12, part1(&input)?);
        let input = input_generator("C0015000016115A2E0802F182340")?;
        assert_eq!(23, part1(&input)?);
        let input = input_generator("A0016C880162017C3686B18A3D4780")?;
        assert_eq!(31, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator("C200B40A82")?;
        assert_eq!(3, part2(&input)?);
        Ok(())
    }
}
