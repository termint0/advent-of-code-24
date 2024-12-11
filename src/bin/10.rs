use std::{collections::HashSet, usize};

advent_of_code::solution!(10);

type Grid = Vec<Vec<u32>>;
type CoordSet = HashSet<(usize, usize)>;

const DIM: usize = 55;
//const DIM: usize = 8;

fn get_zeros(grid: &Grid) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .map(|(row, ln)| {
            ln.iter()
                .enumerate()
                .filter_map(move |(col, val)| if *val == 0 { Some((row, col)) } else { None })
        })
        .flatten()
        .collect()
}

fn get(grid: &Grid, coords: (usize, usize)) -> u32 {
    grid[coords.0][coords.1]
}

fn get_neighbors(coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = coords;
    vec![
        (row + 1, col),
        (row.overflowing_sub(1).0, col),
        (row, col + 1),
        (row, col.overflowing_sub(1).0),
    ]
    .iter()
    .filter_map(|(row, col)| {
        if *row < DIM && *col < DIM {
            Some((*row, *col))
        } else {
            None
        }
    })
    .collect()
}

fn get_score(grid: &Grid, coords: (usize, usize)) -> u32 {
    let mut coord_lst = vec![coords];
    let mut visited: CoordSet = HashSet::new();
    let mut nines: CoordSet = HashSet::new();
    while !coord_lst.is_empty() {
        let current = coord_lst.pop().unwrap();
        if !visited.insert(current) {
            continue;
        }
        if get(&grid, current) == 9 {
            //println!("{:?}", current);
            nines.insert(current);
        }
        let good_neighbors: Vec<(usize, usize)> = get_neighbors(current)
            .iter()
            .filter(|&pair| !visited.contains(pair) && get(&grid, *pair) == get(&grid, current) + 1)
            .map(|(row, col)| (*row, *col))
            .collect();
        coord_lst.extend(good_neighbors);
    }
    nines.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    //    let input = "89010123
    //78121874
    //87430965
    //96549874
    //45678903
    //32019012
    //01329801
    //10456732";
    let grid: Grid = input
        .lines()
        .map(|ln| ln.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    //println!("{:?}", get_zeros(&grid));
    let result: u32 = get_zeros(&grid)
        .iter()
        .map(|coords| get_score(&grid, *coords))
        .sum();
    Some(result)
}

fn get_rating(grid: &Grid, coords: (usize, usize)) -> u32 {
    if get(grid, coords) == 9 {
        return 1;
    }
    get_neighbors(coords)
        .iter()
        .filter_map(|&pair| {
            if get(&grid, pair) == get(&grid, coords) + 1 {
                Some(get_rating(grid, pair))
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
//    let input = "89010123
//78121874
//87430965
//96549874
//45678903
//32019012
//01329801
//10456732";
    let grid: Grid = input
        .lines()
        .map(|ln| ln.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    //println!("{:?}", get_zeros(&grid));
    let result: u32 = get_zeros(&grid)
        .iter()
        .map(|coords| get_rating(&grid, *coords))
        .sum();
    Some(result)
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
