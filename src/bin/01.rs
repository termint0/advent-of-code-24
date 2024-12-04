advent_of_code::solution!(1);

use std::{
    collections::HashMap, io::{self, BufRead, Write}, num::ParseIntError
};



fn push_sorted(lst: &mut Vec<u32>, item: &str) -> Result<(), ParseIntError> {
    let item_int: u32 = match item.parse() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };
    let mut i = 0;
    while i < lst.len() {
        if item_int >= lst[i] {
            lst.insert(i, item_int);
            return Ok(());
        }
        i += 1;
    }
    lst.push(item_int);
    return Ok(());
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut nums1: Vec<u32> = Vec::new();
    let mut nums2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let item1 = tokens.next().unwrap();
        let item2 = tokens.next().unwrap();
        push_sorted(&mut nums1, item1).ok()?;
        push_sorted(&mut nums2, item2).ok()?;
    }
    let mut result: u32 = 0;
    for i in 0..nums1.len() {
        result += nums1[i].abs_diff(nums2[i]) as u32;
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums1: Vec<u32> = Vec::new();
    let mut nums2: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let item1: u32 = tokens.next().unwrap().parse().unwrap();
        let item2: u32 = tokens.next().unwrap().parse().unwrap();
        nums1.push(item1);
        let curr_value = match nums2.get(&item2) {
            Some(num) => *num,
            None => 0
        };
        nums2.insert(item2, curr_value + 1);
    }
    let mut result: u32 = 0;
    for i in 0..nums1.len() {
        let curr_value = match nums2.get(&nums1[i]) {
            Some(num) => *num,
            None => 0
        };
        result += nums1[i] * curr_value;
    }
    return Some(result);
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
