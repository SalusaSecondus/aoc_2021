use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use lazy_static::lazy_static;

type Input = [[char; 2]; 4];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct World([Option<char>; 19]);

impl From<&Input> for World {
    fn from(input: &Input) -> Self {
        World([
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(input[0][0]),
            Some(input[0][1]),
            Some(input[1][0]),
            Some(input[1][1]),
            Some(input[2][0]),
            Some(input[2][1]),
            Some(input[3][0]),
            Some(input[3][1]),
        ])
    }
}

fn disp_char(val: Option<char>) -> char {
    if let Some(pod) = val {
        pod
    } else {
        '.'
    }
}

fn insert_if_less(hash: &mut HashMap<World, i32>, world: World, cost: i32) {
    if let Some(old) = hash.get_mut(&world) {
        *old = cost.min(*old);
    } else {
        hash.insert(world, cost);
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for idx in 0..=10 {
            write!(f, "{}", disp_char(self.0[idx]))?;
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        write!(f, "{}", disp_char(self.0[11]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[13]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[15]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[17]))?;
        writeln!(f, "###")?;
        write!(f, "  #")?;
        write!(f, "{}", disp_char(self.0[12]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[14]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[16]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[18]))?;
        writeln!(f, "#")?;
        writeln!(f, "  #########")?;

        Ok(())
    }
}

impl World {
    fn step_pod(&self, src: usize, dst: usize) -> World {
        let mut result = self.to_owned();
        result.0[dst] = result.0[src];
        result.0[src] = None;
        result
    }

    fn moves(&self, idx: usize) -> HashMap<World, i32> {
        let mut result = HashMap::new();
        let pod = self.0[idx];
        if pod.is_none() {
            return result;
        }
        let pod = pod.unwrap();

        // If we are in the correct place, we do nothing
        let destination_room = Self::dest_room(pod);

        if Self::room_num(idx) == destination_room
            && (destination_room != Self::room_num(idx + 1) || self.0[idx + 1] == Some(pod))
        {
            return result;
        }

        let step_cost = match pod {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Bad character"),
        };
        let can_stop_in_hall = !Self::is_hallway(idx);

        let mut possibilities = vec![(idx, self.to_owned(), 0)];
        let mut visited = HashSet::new();
        while !possibilities.is_empty() {
            let (idx, current_world, cost) = possibilities.pop().unwrap();
            if !visited.insert(current_world.to_owned()) {
                continue;
            }
            // println!("Considering at cost {}", cost);
            // println!("{}", current_world);
            for neighbor in &ADJACENCY[idx] {
                if current_world.0[*neighbor].is_some() {
                    continue;
                }

                // If I'm in a hallway, I can step elsewhere in the hallway or into my room
                if Self::is_hallway(idx) {
                    if Self::is_hallway(*neighbor) {
                        let dest = current_world.step_pod(idx, *neighbor);
                        let valid_stop = can_stop_in_hall && !Self::is_doorway(*neighbor);
                        if valid_stop {
                            insert_if_less(&mut result, dest.to_owned(), cost + step_cost);
                        }
                        possibilities.push((*neighbor, dest, cost + step_cost));
                    } else if Self::room_num(*neighbor) == destination_room {
                        // I can enter, but can I go all the way? If I can, then I should
                        if let Some(resident) = current_world.0[neighbor + 1] {
                            if resident == pod {
                                // This is my fellow pod
                                let dest = current_world.step_pod(idx, *neighbor);
                                result.clear();
                                // insert_if_less(&mut result, dest, cost+step_cost);
                                result.insert(dest, cost + step_cost);
                                return result;
                            }
                        } else {
                            // Room is empty!
                            let dest = current_world.step_pod(idx, neighbor + 1);
                            // insert_if_less(&mut result, dest, cost+step_cost+step_cost);
                            result.clear();
                            result.insert(dest, cost + step_cost + step_cost);
                            return result;
                        }
                    }
                } else {
                    // I'm in a room that isn't my own. There are no valid stopping points from here
                    let dest = current_world.step_pod(idx, *neighbor);
                    possibilities.push((*neighbor, dest, cost + step_cost));
                }
            }
        }

        result
    }

    fn is_hallway(idx: usize) -> bool {
        idx <= 0xa
    }

    fn is_doorway(idx: usize) -> bool {
        ADJACENCY[idx].len() > 2
    }

    fn room_num(idx: usize) -> u8 {
        if idx < 0xb {
            100
        } else {
            (idx as u8 - 0xb) / 2
        }
        // assert!(!Self::is_hallway(idx));
    }

    fn dest_room(pod: char) -> u8 {
        pod as u8 - b'A'
    }

    fn win(&self) -> bool {
        for idx in 11..=18 {
            // println!("{:?} at {} dest is {} current is {}", self.0[idx], idx, Self::dest_room(self.0[idx].unwrap()), Self::room_num(idx));
            if let Some(pod) = self.0[idx] {
                if Self::dest_room(pod) != Self::room_num(idx) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

lazy_static! {
    static ref ADJACENCY: Vec<Vec<usize>> = vec![
        vec![1],
        vec![0, 2],
        vec![1, 11, 3],
        vec![2, 4],
        vec![3, 13, 5],
        vec![4, 6],
        vec![5, 15, 7],
        vec![6, 8],
        vec![7, 17, 9],
        vec![8, 10],
        vec![9],
        vec![2, 12],
        vec![11],
        vec![4, 14],
        vec![13],
        vec![6, 16],
        vec![15],
        vec![8, 18],
        vec![17]
    ];
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapElement {
    world: World,
    cost: i32,
}

impl HeapElement {
    fn new(world: World, cost: i32) -> Self {
        HeapElement { world, cost }
    }
}

impl From<(World, i32)> for HeapElement {
    fn from(tuple: (World, i32)) -> Self {
        HeapElement::new(tuple.0, tuple.1)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn solve(world: World) -> i32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(HeapElement::new(world, 0));
    let mut cheapest = i32::MAX;
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current.cost > cheapest {
            return cheapest;
        }
        if !visited.insert(current.world.to_owned()) {
            continue;
        }
        // println!("Visited {} worlds with {} in queue and current cost is {}", visited.len(), queue.len(), current.cost);
        for idx in 0..=18 {
            for next in current.world.moves(idx) {
                if next.0.win() {
                    // println!("Found solution with cost {}", next.1 + current.cost);
                    cheapest = cheapest.min(next.1 + current.cost);
                } else {
                    queue.push(HeapElement::new(next.0, next.1 + current.cost));
                }
            }
        }
    }
    cheapest
}

fn solve2(world: World2) -> i32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(HeapElement2::new(world, 0));
    let mut cheapest = i32::MAX;
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current.cost > cheapest {
            return cheapest;
        }
        if !visited.insert(current.world.to_owned()) {
            continue;
        }
        // println!("Visited {} worlds with {} in queue and current cost is {}", visited.len(), queue.len(), current.cost);
        // if visited.len() < 40 {
        // println!("Considering \n{}", current.world);
        // }
        for idx in 0..27 {
            for next in current.world.moves(idx) {
                let next_cost = next.1 + current.cost;
                if next.0.win() {
                    // println!("Found solution with cost {}", next.1 + current.cost);
                    cheapest = cheapest.min(next_cost);
                } else if next_cost < cheapest {
                    queue.push(HeapElement2::new(next.0, next.1 + current.cost));
                }
            }
        }
    }
    cheapest
}

type Input2 = [[char; 4]; 4];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct World2([Option<char>; 27]);

impl From<&Input2> for World2 {
    fn from(input: &Input2) -> Self {
        World2([
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(input[0][0]),
            Some(input[0][1]),
            Some(input[0][2]),
            Some(input[0][3]),
            Some(input[1][0]),
            Some(input[1][1]),
            Some(input[1][2]),
            Some(input[1][3]),
            Some(input[2][0]),
            Some(input[2][1]),
            Some(input[2][2]),
            Some(input[2][3]),
            Some(input[3][0]),
            Some(input[3][1]),
            Some(input[3][2]),
            Some(input[3][3]),
        ])
    }
}

fn insert_if_less2(hash: &mut HashMap<World2, i32>, world: World2, cost: i32) {
    if let Some(old) = hash.get_mut(&world) {
        *old = cost.min(*old);
    } else {
        hash.insert(world, cost);
    }
}

impl Display for World2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for idx in 0..=10 {
            write!(f, "{}", disp_char(self.0[idx]))?;
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        write!(f, "{}", disp_char(self.0[11]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[15]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[19]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[23]))?;
        writeln!(f, "###")?;
        write!(f, "  #")?;
        write!(f, "{}", disp_char(self.0[12]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[16]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[20]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[24]))?;
        writeln!(f, "#")?;
        write!(f, "  #")?;
        write!(f, "{}", disp_char(self.0[13]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[17]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[21]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[25]))?;
        writeln!(f, "#")?;
        write!(f, "  #")?;
        write!(f, "{}", disp_char(self.0[14]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[18]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[22]))?;
        write!(f, "#")?;
        write!(f, "{}", disp_char(self.0[26]))?;
        writeln!(f, "#")?;
        writeln!(f, "  #########")?;

        Ok(())
    }
}

impl World2 {
    fn step_pod(&self, src: usize, dst: usize) -> World2 {
        let mut result = self.to_owned();
        result.0[dst] = result.0[src];
        result.0[src] = None;
        result
    }

    fn moves(&self, idx: usize) -> HashMap<World2, i32> {
        let mut result = HashMap::new();
        let pod = self.0[idx];
        if pod.is_none() {
            return result;
        }
        let pod = pod.unwrap();

        let destination_room = Self::dest_room(pod);

        // If we are in the correct place, we do nothing
        if Self::room_num(idx) == destination_room {
            let base_idx = destination_room * 4 + 0xb;
            if (base_idx..base_idx + 4)
                .map(|i| self.0[i as usize])
                .all(|v| v == Some(pod))
            {
                return result;
            }
        }

        let step_cost = match pod {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Bad character"),
        };
        let can_stop_in_hall = !Self::is_hallway(idx);

        let mut possibilities = vec![(idx, self.to_owned(), 0)];
        let mut visited = HashSet::new();
        while !possibilities.is_empty() {
            let (idx, current_world, cost) = possibilities.pop().unwrap();
            if !visited.insert(current_world.to_owned()) {
                continue;
            }
            // println!("Considering at cost {}", cost);
            // println!("{}", current_world);
            for neighbor in &ADJACENCY2[idx] {
                if current_world.0[*neighbor].is_some() {
                    continue;
                }

                // If I'm in a hallway, I can step elsewhere in the hallway or into my room
                if Self::is_hallway(idx) {
                    if Self::is_hallway(*neighbor) {
                        let dest = current_world.step_pod(idx, *neighbor);
                        let valid_stop = can_stop_in_hall && !Self::is_doorway(*neighbor);
                        if valid_stop {
                            insert_if_less2(&mut result, dest.to_owned(), cost + step_cost);
                        }
                        possibilities.push((*neighbor, dest, cost + step_cost));
                    } else if Self::room_num(*neighbor) == destination_room {
                        if current_world.0[neighbor + 3].is_none()
                            && current_world.0[neighbor + 2].is_none()
                            && current_world.0[neighbor + 1].is_none()
                        {
                            let dest = current_world.step_pod(idx, neighbor + 3);
                            // insert_if_less2(&mut result, dest, cost+step_cost+step_cost);
                            result.clear();
                            result
                                .insert(dest, cost + step_cost + step_cost + step_cost + step_cost);
                            return result;
                        } else if current_world.0[neighbor + 3] == Some(pod)
                            && current_world.0[neighbor + 2].is_none()
                            && current_world.0[neighbor + 1].is_none()
                        {
                            let dest = current_world.step_pod(idx, neighbor + 2);
                            // insert_if_less2(&mut result, dest, cost+step_cost+step_cost);
                            result.clear();
                            result.insert(dest, cost + step_cost + step_cost + step_cost);
                            return result;
                        } else if current_world.0[neighbor + 3] == Some(pod)
                            && current_world.0[neighbor + 2] == Some(pod)
                            && current_world.0[neighbor + 1].is_none()
                        {
                            let dest = current_world.step_pod(idx, neighbor + 1);
                            // insert_if_less2(&mut result, dest, cost+step_cost+step_cost);
                            result.clear();
                            result.insert(dest, cost + step_cost + step_cost);
                            return result;
                        } else if current_world.0[neighbor + 3] == Some(pod)
                            && current_world.0[neighbor + 2] == Some(pod)
                            && current_world.0[neighbor + 1] == Some(pod)
                        {
                            let dest = current_world.step_pod(idx, *neighbor);
                            // insert_if_less2(&mut result, dest, cost+step_cost+step_cost);
                            result.clear();
                            result.insert(dest, cost + step_cost);
                            return result;
                        }
                    }
                } else {
                    // I'm in a room that isn't my own. There are no valid stopping points from here
                    let dest = current_world.step_pod(idx, *neighbor);
                    possibilities.push((*neighbor, dest, cost + step_cost));
                }
            }
        }

        result
    }

    fn is_hallway(idx: usize) -> bool {
        idx <= 0xa
    }

    fn is_doorway(idx: usize) -> bool {
        ADJACENCY2[idx].len() > 2
    }

    fn room_num(idx: usize) -> u8 {
        if idx < 0xb {
            100
        } else {
            (idx as u8 - 0xb) / 4
        }
        // assert!(!Self::is_hallway(idx));
    }

    fn dest_room(pod: char) -> u8 {
        pod as u8 - b'A'
    }

    fn win(&self) -> bool {
        for idx in 11..27 {
            // println!("{:?} at {} dest is {} current is {}", self.0[idx], idx, Self::dest_room(self.0[idx].unwrap()), Self::room_num(idx));
            if let Some(pod) = self.0[idx] {
                if Self::dest_room(pod) != Self::room_num(idx) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl From<World> for World2 {
    fn from(world1: World) -> Self {
        let mut locs = [None; 27];
        locs[11] = world1.0[11];
        locs[12] = Some('D');
        locs[13] = Some('D');
        locs[14] = world1.0[12];

        locs[15] = world1.0[13];
        locs[16] = Some('C');
        locs[17] = Some('B');
        locs[18] = world1.0[14];

        locs[19] = world1.0[15];
        locs[20] = Some('B');
        locs[21] = Some('A');
        locs[22] = world1.0[16];

        locs[23] = world1.0[17];
        locs[24] = Some('A');
        locs[25] = Some('C');
        locs[26] = world1.0[18];
        World2(locs)
    }
}
lazy_static! {
    static ref ADJACENCY2: Vec<Vec<usize>> = vec![
        // Hallway
        vec![1],
        vec![0, 2],
        vec![1, 11, 3],
        vec![2, 4],
        vec![3, 15, 5],
        vec![4, 6],
        vec![5, 19, 7],
        vec![6, 8],
        vec![7, 23, 9],
        vec![8, 10],
        vec![9],
        // Room 0
        vec![2, 12],
        vec![11, 13],
        vec![12, 14],
        vec![13],
        // Room 1
        vec![4, 16],
        vec![15, 17],
        vec![16, 18],
        vec![17],
        // Room 2
        vec![6, 20],
        vec![19, 21],
        vec![20, 22],
        vec![21],
        // Room 2
        vec![8, 24],
        vec![23, 25],
        vec![24, 26],
        vec![25],
    ];
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapElement2 {
    world: World2,
    cost: i32,
}

impl HeapElement2 {
    fn new(world: World2, cost: i32) -> Self {
        HeapElement2 { world, cost }
    }
}

impl From<(World2, i32)> for HeapElement2 {
    fn from(tuple: (World2, i32)) -> Self {
        HeapElement2::new(tuple.0, tuple.1)
    }
}

impl PartialOrd for HeapElement2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[aoc_generator(day23)]
fn input_generator(_: &str) -> Result<Input> {
    Ok([['D', 'C'], ['D', 'C'], ['A', 'B'], ['A', 'B']])
}

#[aoc(day23, part1)]
fn part1(input: &Input) -> Result<i32> {
    let world = input.into();
    Ok(solve(world))
}

#[aoc(day23, part2)]
fn part2(input: &Input) -> Result<i32> {
    let world: World = input.into();
    let world: World2 = world.into();
    // println!("{}", world);
    Ok(solve2(world))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOKE: Input = [['B', 'A'], ['C', 'D'], ['B', 'C'], ['D', 'A']];

    #[test]
    fn smoke1() -> Result<()> {
        let mut world: World = (&SMOKE).into();
        let mut cost = 0;
        println!("Smoke input\n{}", world);
        // Fully checked state
        for next in world.moves(15) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[3] == Some('B') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(0xd) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[0xf] == Some('C') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(0xe) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[5] == Some('D') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(3) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[0xe] == Some('B') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(0xb) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[0xd] == Some('B') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(17) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[7] == Some('D') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(18) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[9] == Some('A') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(7) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[18] == Some('D') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(5) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[17] == Some('D') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Next\n===\n{}\n===", world);
        for next in world.moves(9) {
            println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
            println!("{}", next.0);
            if next.0 .0[11] == Some('A') {
                world = next.0;
                cost += next.1;
            }
        }
        println!("Done at {} winning? {}", cost, world.win());
        println!("{}", world);
        let winning_world: World = (&[['A', 'A'], ['B', 'B'], ['C', 'C'], ['D', 'D']]).into();
        assert!(winning_world.win());
        assert_eq!(12521, part1(&SMOKE)?);
        Ok(())
    }

    #[test]
    fn smoke2() -> Result<()> {
        // let winning_world: World2 = (&[['A', 'A', 'A', 'A'], ['B', 'B', 'B', 'B'], ['C', 'C', 'C', 'C'], ['D', 'D', 'D', 'D']]).into();
        // assert!(winning_world.win());
        // let world: World = (&SMOKE).into();
        // let mut world: World2 = world.into();
        // let mut cost = 0;
        // println!("Smoke input\n{}", world);
        // // Fully checked state
        // for next in world.moves(23) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[10] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(24) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[0] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(19) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[9] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(20) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[7] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(21) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[1] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(15) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[21] == Some('C') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(16) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[20] == Some('C') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(17) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[5] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(18) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[3] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(5) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[18] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(7) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[17] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(25) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[19] == Some('C') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(9) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[16] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(26) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[9] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(3) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[26] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(11) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[15] == Some('B') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(12) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[25] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(13) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[24] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(1) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[13] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(0) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[12] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(9) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[11] == Some('A') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Next\n===\n{}\n===", world);
        // for next in world.moves(10) {
        //     println!("Cost = {} (Winning={})", next.1 + cost, next.0.win());
        //     println!("{}", next.0);
        //     if next.0.0[23] == Some('D') {
        //         world = next.0;
        //         cost += next.1;
        //     }
        // }
        // println!("Done at {} winning? {}", cost, world.win());
        // println!("{}", world);
        assert_eq!(44169, part2(&SMOKE)?);
        Ok(())
    }
}
