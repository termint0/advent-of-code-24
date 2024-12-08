use std::collections::HashMap;
use std::collections::HashSet;
use std::i64;

advent_of_code::solution!(8);

type Board = Vec<Vec<char>>;
type CoordSet = HashSet<(i64, i64)>;
type CoordMap = HashMap<char, CoordSet>;

const DIM: i64 = 50;

fn extend(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (( 2 * b.0 ).overflowing_sub(a.0).0, (2 * b.1).overflowing_sub( a.1).0)
}

fn valid(a: (i64, i64)) -> bool {
    a.0 >= 0 && a.1 >= 0 && a.0 < DIM && a.1 < DIM
}

fn find_antinodes(coords: &CoordSet) -> CoordSet {
    let mut result: CoordSet = HashSet::new();
    for pair1 in coords.iter() {
        for pair2 in coords.iter() {
            if pair1 == pair2 {
                continue;
            }
            let antinode = extend(*pair1, *pair2);
            if valid(antinode) {
                result.insert(antinode);
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let board: Board = input.lines().map(|ln| ln.chars().collect()).collect();
    let mut coord_map: CoordMap = HashMap::new();
    board.iter().enumerate().for_each(|(row, ln)| {
        ln.iter().enumerate().for_each(|(col, c)| {
            if *c == '.' {
                return;
            }
            coord_map
                .entry(*c)
                .or_insert_with(HashSet::new)
                .insert((row as i64, col as i64));
        });
    });
    let mut result: CoordSet = HashSet::new();
    coord_map
        .iter()
        .map(|(_c, coords)| find_antinodes(&coords))
        .for_each(|res| result.extend(&res));
    Some( result.len() as u32)
}

fn normalize(mut a: (i64, i64)) -> (i64, i64) {
    for i in 2..a.0 {
        if a.0 % i == 0 && a.1 % i == 0 {
            a = (a.0 / i , a.1 / i);
        }
    }
    a
}


fn subtract(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

fn add(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn multiply(a: &(i64, i64), b: i64) -> (i64, i64) {
    (a.0 * b, a.1 * b)
}


fn find_antinodes_two(coords: &CoordSet) -> CoordSet {
    let mut result: CoordSet = HashSet::new();
    for pair1 in coords.iter() {
        for pair2 in coords.iter() {
            if pair1 == pair2 {
                continue;
            }
            let diff = normalize(subtract(pair1, pair2));
            for i in 0..DIM {
                let point = add(pair1, &multiply(&diff, i));
                if !valid(point) {
                    break;
                }
                result.insert(point);
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let board: Board = input.lines().map(|ln| ln.chars().collect()).collect();
    let mut coord_map: CoordMap = HashMap::new();
    board.iter().enumerate().for_each(|(row, ln)| {
        ln.iter().enumerate().for_each(|(col, c)| {
            if *c == '.' {
                return;
            }
            coord_map
                .entry(*c)
                .or_insert_with(HashSet::new)
                .insert((row as i64, col as i64));
        });
    });
    let mut result: CoordSet = HashSet::new();
    coord_map
        .iter()
        .map(|(_c, coords)| find_antinodes_two(&coords))
        .for_each(|res| result.extend(&res));
    Some( result.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
