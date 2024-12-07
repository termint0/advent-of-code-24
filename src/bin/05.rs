advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    let mut reading_rules = true;
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            reading_rules = false;
        } else if reading_rules {
            let mut split = line.split("|");
            let page: usize = split.next().unwrap().parse().unwrap();
            let require: usize = split.next().unwrap().parse().unwrap();
            rules[page].push(require);
        } else {
            let pages: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
            let mut good = true;
            let mut used_so_far: Vec<usize> = Vec::new();
            for page in &pages {
                if used_so_far
                    .iter()
                    .map(|x| rules[*page].contains(x))
                    .any(|x| x)
                {
                    good = false;
                    break;
                }
                used_so_far.push(*page);
            }
            if good {
                count += pages[pages.len() / 2];
            }
        }
    }
    Some(count as u32)
}

fn remove_from_rules(rules: &mut [Vec<usize>; 100], val: usize) -> () {
    for i in 0..100 {
        rules[i] = rules[i]
            .iter()
            .map(|x| x.clone())
            .filter(|x| *x != val)
            .collect();
    }
}

fn fix(pages_bor: &Vec<usize>, rules_bor: &[Vec<usize>; 100]) -> usize {
    let mut result: Vec<usize> = Vec::new();
    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    for i in 0..100 {
        rules[i] = rules_bor[i]
            .iter()
            .filter(|x| pages_bor.contains(x))
            .map(|x| x.clone())
            .collect();
    }
    let mut pages = pages_bor.clone();
    while result.len() != pages_bor.len() {
        for i in 0..pages.len() {
            if rules[pages[i]].is_empty() {
                remove_from_rules(&mut rules, pages[i]);
                result.push(pages[i]);
                pages.remove(i);
                break;
            }
        }
    }
    result[result.len() / 2]
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    let mut reading_rules = true;
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            reading_rules = false;
        } else if reading_rules {
            let mut split = line.split("|");
            let page: usize = split.next().unwrap().parse().unwrap();
            let require: usize = split.next().unwrap().parse().unwrap();
            rules[page].push(require);
        } else {
            let pages: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
            let mut good = true;
            let mut used_so_far: Vec<usize> = Vec::new();
            for page in &pages {
                if used_so_far
                    .iter()
                    .map(|x| rules[*page].contains(x))
                    .any(|x| x)
                {
                    good = false;
                    break;
                }
                used_so_far.push(*page);
            }
            if !good {
                count += fix(&pages, &rules);
            }
        }
    }
    Some(count as u32)
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
