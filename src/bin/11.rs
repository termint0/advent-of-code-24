use std::collections::HashMap;

advent_of_code::solution!(11);

fn n_digits(rock: u64) -> u32 {
    (rock as f64).log10() as u32 + 1
}

fn transform_rock(rock: u64) -> (u64, Option<u64>) {
    if rock == 0 {
        return (1, None);
    }
    let digits = n_digits(rock);
    if digits % 2 == 0 {
        return (
            rock / 10_u64.pow(digits / 2),
            Some(rock % 10_u64.pow(digits / 2)),
        );
    }
    return (rock * 2024, None);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    for _ in 0..25 {
        for i in 0..stones.len() {
            let (a, b) = transform_rock(stones[i]);
            stones[i] = a;
            if b.is_some() {
                stones.push(b.unwrap());
            }
        }
    }
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut occurences: HashMap<u64, usize> = HashMap::new();
    stones.iter().for_each(|rock| {
        occurences.insert(*rock, 1);
    });
    for _i in 0..75 {
        let mut new_occurences: HashMap<u64, usize> = HashMap::new();
        for (key, val) in occurences.iter() {
            let (a, b) = transform_rock(*key);
            *new_occurences.entry(a).or_insert(0) += val;
            if b.is_some() {
                *new_occurences.entry(b.unwrap()).or_insert(0) += val;
            }
        }
        occurences = new_occurences;
    }
    Some(occurences.values().sum::<usize>() as u64
        )
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
