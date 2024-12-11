advent_of_code::solution!(9);

fn parse_input_char(idx: usize, count: usize) -> Vec<i64> {
    let uid;
    if idx % 2 == 1 {
        uid = -1;
    } else {
        uid = (idx as i64) / 2;
    }
    let mut vec = Vec::new();
    for _i in 0..count {
        vec.push(uid);
    }
    vec
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut thing: Vec<i64> = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| parse_input_char(i, c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect();
    let mut i = 0;
    let mut j = thing.len() - 1;
    while i < j {
        if thing[i] == -1 && thing[j] != -1 {
            thing[i] = thing[j];
            thing[j] = -1;
        }
        if thing[i] != -1 {
            i += 1;
        }
        if thing[j] == -1 {
            j -= 1;
        }
    }
    let result: u128 = thing
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if *val == -1 {
                0
            } else {
                (i as u128) * (*val as u128)
            }
        })
        .sum();
    Some(result)
}

fn parse_input_char_two(idx: usize, count: usize) -> (i64, usize) {
    let uid;
    if idx % 2 == 1 {
        uid = -1;
    } else {
        uid = (idx as i64) / 2;
    }
    (uid, count)
}

pub fn part_two(input: &str) -> Option<u128> {
    //let input = "2333133121414131402";
    let mut thing: Vec<(i64, usize)> = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| parse_input_char_two(i, c.to_digit(10).unwrap() as usize))
        .collect();
    let mut i = thing.len() - 1;
    while i > 0 {
        let block_full = thing[i];
        if block_full.0 == -1 {
            i -= 1;
            continue;
        }

        for j in 0..i {
            let block_empty = thing[j];
            if block_empty.0 != -1 || block_empty.1 < block_full.1 {
                continue;
            }
            if block_empty.1 == block_full.1 {
                thing[j].0 = block_full.0;
                thing[i].0 = -1;
            } else {
                thing[j].1 = block_empty.1 - block_full.1;
                thing.insert(j, block_full);
                i += 1;
                thing[i].0 = -1;
            }
            break;
        }
        i -= 1;
    }
    let result: u128 = thing
        .iter()
        .map(|(val, count)| {
            let mut v = Vec::new();
            for _ in 0..*count {
                v.push(val);
            }
            v
        })
        .flatten()
        .enumerate()
        .map(|(i, val)| {
            if *val == -1 {
                0
            } else {
                (i as u128) * (*val as u128)
            }
        })
        .sum();
    Some(result)
}

//for j in 1..i {
//    if thing[j].0 == thing[j - 1].0 {
//        thing[j-1].1 += thing[j].1;
//        thing[j].1 = 0;
//    }
//}
//let mut j = 0;
//while j < i {
//    if thing[j].1 == 0 {
//        thing.remove(j);
//    } else {
//        j += 1;
//    }
//}
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
