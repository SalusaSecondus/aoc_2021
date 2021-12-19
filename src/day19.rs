use std::{collections::HashSet, fmt::Display, vec};

use anyhow::{Context, Result};
use itertools::Itertools;
use regex::Regex;

const MAX_ORIENTATION: u8 = 24;
type Coordinate = [i32; 3];

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    location: Coordinate,
    orientation: u8, // 0-23
    beacons: Vec<Coordinate>,
}

impl Scanner {
    fn new(id: i32, beacons: Vec<Coordinate>) -> Self {
        Self {
            id,
            location: [0; 3],
            orientation: 0,
            beacons,
        }
    }

    fn abs_beacon_location(&self, beacon: &Coordinate) -> Coordinate {
        let mut result = beacon.to_owned();
        // First, manage orientation
        let axis = self.orientation / 4;
        let rotation = self.orientation % 4;

        result = match axis {
            0 => result,
            1 => [-result[0], -result[1], result[2]],
            2 => [-result[1], result[0], result[2]],
            3 => [result[1], -result[0], result[2]],
            4 => [-result[2], result[1], result[0]],
            5 => [result[2], result[1], -result[0]],
            _ => panic!("Invalid axis"),
        };

        result = match rotation {
            0 => result,
            1 => [result[0], result[2], -result[1]],
            2 => [result[0], -result[1], -result[2]],
            3 => [result[0], -result[2], result[1]],
            _ => panic!("Invalid rotation"),
        };

        // Calculate offset
        for (val, base) in result.iter_mut().zip(self.location) {
            *val += base;
        }

        result
    }

    fn abs_beacons(&self) -> Vec<Coordinate> {
        self.beacons
            .iter()
            .map(|b| self.abs_beacon_location(b))
            .collect()
    }

    fn set_beacon_abs(&mut self, beacon_id: usize, location: &Coordinate) {
        self.location = [0; 3];
        let beacon_location = &self.beacons[beacon_id];
        let abs_beacon_location = self.abs_beacon_location(beacon_location);
        self.location[0] = location[0] - abs_beacon_location[0];
        self.location[1] = location[1] - abs_beacon_location[1];
        self.location[2] = location[2] - abs_beacon_location[2];

        assert_eq!(location, &self.abs_beacon_location(beacon_location));
    }
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Result<Vec<Scanner>> {
    let input = scanner_parser(input)?;
    Ok(build_world(&input))
}

fn scanner_parser(input: &str) -> Result<Vec<Scanner>> {
    let mut lines = input.lines();
    let re = Regex::new("--- scanner (\\d+) ---")?;
    let mut result = vec![];
    while let Some(line) = lines.next() {
        let captures = re.captures(line).context("No match")?;
        let id: i32 = captures.get(1).unwrap().as_str().parse()?;

        let mut beacons = vec![];
        #[allow(clippy::while_let_on_iterator)]
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            let mut coords = line.split(',');
            let x: i32 = coords.next().unwrap().parse()?;
            let y: i32 = coords.next().unwrap().parse()?;
            let z: i32 = coords.next().unwrap().parse()?;
            assert_eq!(None, coords.next());
            beacons.push([x, y, z]);
        }
        result.push(Scanner::new(id, beacons));
    }
    Ok(result)
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- scanner {} ---", self.id)?;
        for b in self.abs_beacons() {
            writeln!(f, "{},{},{}", b[0], b[1], b[2])?;
        }
        Ok(())
    }
}

fn find_overlap(base: &Scanner, relative: &mut Scanner) -> Vec<Coordinate> {
    let mut base_locs = HashSet::new();
    for b in base.abs_beacons() {
        base_locs.insert(b);
    }
    let base_locs = base_locs;
    let max_rel_id = relative.beacons.len();
    for orientation in 0..MAX_ORIENTATION {
        relative.orientation = orientation;

        for base_loc in &base_locs {
            for rel_id in 0..max_rel_id {
                relative.set_beacon_abs(rel_id, base_loc);
                let overlap: Vec<Coordinate> = relative
                    .abs_beacons()
                    .iter()
                    .filter(|c| base_locs.contains(*c))
                    .copied()
                    .collect();
                if overlap.len() >= 12 {
                    return overlap;
                }
            }
        }
    }
    vec![]
}

fn build_world(input: &[Scanner]) -> Vec<Scanner> {
    let mut input = input.to_owned();

    let mut known_scanners = vec![input.pop().unwrap()];
    let mut known_bad = HashSet::new();

    while !input.is_empty() {
        // println!("Base size = {}", known_scanners.len());
        'next_scanner: for base in &known_scanners {
            for idx in 0..input.len() {
                let rel = input.get_mut(idx).unwrap();
                if known_bad.contains(&(base.id, rel.id)) {
                    continue;
                }
                let overlap = find_overlap(base, rel);
                if overlap.len() >= 12 {
                    known_scanners.push(input.remove(idx));
                    break 'next_scanner;
                } else {
                    known_bad.insert((base.id, rel.id));
                    known_bad.insert((rel.id, base.id));
                }
            }
        }
    }
    known_scanners
}

#[aoc(day19, part1)]
fn part1(input: &[Scanner]) -> Result<usize> {
    // let known_scanners = build_world(input);

    // Find all beacons
    let mut beacons = HashSet::new();
    for s in input {
        for b in s.abs_beacons() {
            beacons.insert(b);
        }
    }
    Ok(beacons.len())
}

#[aoc(day19, part2)]
fn part2(input: &[Scanner]) -> Result<i32> {
    // let known_scanners = build_world(input);
    let max_dist: i32 = input
        .iter()
        .map(|s| s.location)
        .cartesian_product(input.iter().map(|s| s.location))
        .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
        .max()
        .unwrap();
    Ok(max_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn rotations() -> Result<()> {
        let mut scanner = scanner_parser(
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7",
        )?
        .remove(0);

        for orientation in 0..MAX_ORIENTATION {
            scanner.orientation = orientation;
            // println!("{}", scanner);
            for beacon_id in 0..scanner.beacons.len() {
                // println!("\t Beacon {}", beacon_id);
                scanner.set_beacon_abs(beacon_id, &[0; 3]);
            }
            scanner.location = [0; 3];
        }

        let mut scanners = scanner_parser(SMOKE)?;
        let scanners = scanners.as_mut_slice();
        {
            let split = scanners.split_at_mut(1);
            let one_two = find_overlap(&split.0[0], &mut split.1[0]);
            assert_eq!(12, one_two.len());
            println!("{:?}", one_two);
        }
        Ok(())
    }

    #[test]
    fn smoke1() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(79, part1(&input)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        let input = input_generator(SMOKE)?;
        assert_eq!(3621, part2(&input)?);
        Ok(())
    }
}

/*
 * I can select an arbiratry scanner as 0,0,0 with correct orientation.
 * Then I can iterate through all others (and rotations) until I find an overlap.
 * This should take ~624 trials, not counting finding /which/ beacons overlap.
 * Give each scanner the ability to set its offset relative to one of its beacons? */
