advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result: u32 = 0;
    for (x, [one, two]) in re.captures_iter(input).map(|c| c.extract()) {
        result += one.parse::<u32>().unwrap() * two.parse::<u32>().unwrap();
    }

    Some(result)

        part_two_regex(input)
}

fn part_two_regex(input: &str) -> Option<u32> {
    let re_outer = Regex::new(r"(^|do\(\))(.*?)(don't\(\)|$)").unwrap();
    let mut result: u32 = 0;
    for (_, [_do, between, _dont]) in re_outer.captures_iter(input).map(|c| c.extract()) {
        result += part_one(between).unwrap();
    }
    Some(result)
}

fn part_two_state(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();

    let mut enabled = true;
    let mut result: u32 = 0;
    for (x, [one, two]) in re.captures_iter(input).map(|c| c.extract()) {
        if (x.contains("don't")) {
            enabled = false;
        } else if (x.contains("do")) {
            enabled = true;
        } else if (enabled) {
            result += one.parse::<u32>().unwrap() * two.parse::<u32>().unwrap();
        }
    }

    Some(result)

}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_state(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let input = advent_of_code::template::read_file("examples", DAY);
        let input = "mul(5,5)lkjhkdon't()mul(6,6)lkkjgdo()mul(5,5)";
        let result = part_one(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, None);
    }
}
