advent_of_code::solution!(7);

fn does_match(nums: &Vec<u128>, idx: usize, result: u128, goal: u128) -> bool {
    if idx == nums.len() {
        return result == goal;
    }
    let last = nums[idx];
    return does_match(nums, idx + 1, result + last, goal)
        || does_match(nums, idx + 1, result * last, goal);
}

pub fn part_one(input: &str) -> Option<u128> {
    let data: Vec<(u128, u128, Vec<u128>)> = input
        .replace(":", "")
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect()
        })
        .map(|ln: Vec<u128>| {
            (
                ln[0],
                ln[1],
                ln.iter()
                    .enumerate()
                    .filter_map(|(i, val)| if i > 1 { Some(*val) } else { None })
                    .collect(),
            )
        })
        .collect();
    let result = data
        .iter()
        .map(|(goal, result, nums)| (goal, does_match(nums, 0, *result, *goal)))
        .filter_map(|(goal, good)| if good {Some(goal)} else {None})
        .sum();
    Some(result)
}

fn cat(a: u128, b: u128) -> u128 {
    a * 10_u128.pow((b as f64).log10() as u32 + 1) + b
}

fn does_match_two(nums: &Vec<u128>, idx: usize, result: u128, goal: u128) -> bool {
    if idx == nums.len() {
        return result == goal;
    }
    let last = nums[idx];
    return does_match(nums, idx + 1, result + last, goal)
        || does_match(nums, idx + 1, result * last, goal)
        || does_match(nums, idx + 1, cat(result, last), goal);
}

pub fn part_two(input: &str) -> Option<u128> {
    let data: Vec<(u128, u128, Vec<u128>)> = input
        .replace(":", "")
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect()
        })
        .map(|ln: Vec<u128>| {
            (
                ln[0],
                ln[1],
                ln.iter()
                    .enumerate()
                    .filter_map(|(i, val)| if i > 1 { Some(*val) } else { None })
                    .collect(),
            )
        })
        .collect();
    let result = data
        .iter()
        .map(|(goal, result, nums)| (goal, does_match_two(nums, 0, *result, *goal)))
        .filter_map(|(goal, good)| if good {Some(goal)} else {None})
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
