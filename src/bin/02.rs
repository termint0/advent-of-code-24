advent_of_code::solution!(2);

fn is_safe(codes: &Vec<u32>) -> bool {
    if codes.len() < 2 {
        return true;
    }
    let asc = codes[0] < codes[1];
    codes
        .iter()
        .map(|x| x.clone() as i64)
        .collect::<Vec<i64>>()
        .windows(2)
        .map(|p| p[0] - p[1])
        .map(|diff| diff != 0 && diff.abs() <= 3 && (diff < 0) == asc)
        .all(|x| x)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line_str in input.lines() {
        let line_split = line_str.split(" ");
        let line: Vec<u32> = line_split.map(|x| x.parse::<u32>().unwrap()).collect();
        count += is_safe(&line) as u32;
    }
    return Some(count);
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line_str| {
                let line: Vec<u32> = line_str
                    .split(" ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();

                let safe = (0..line.len())
                    .map(|idx| {
                        let codes: Vec<u32> = line
                            .iter()
                            .enumerate()
                            .filter(|&(i, _val)| i != idx)
                            .map(|(_i, val)| val.clone())
                            .collect();
                        if codes.len() < 2 {
                            return true;
                        }
                        let asc = codes[0] < codes[1];
                        codes
                            .iter()
                            .map(|x| x.clone() as i64)
                            .collect::<Vec<i64>>()
                            .windows(2)
                            .map(|p| p[0] - p[1])
                            .map(|diff| diff != 0 && diff.abs() <= 3 && (diff < 0) == asc)
                            .all(|x| x)
                    })
                    .any(|b| b);
                safe
            })
            .filter(|b| *b)
            .count() as u32,
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
