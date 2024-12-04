advent_of_code::solution!(4);

const MAS: [char; 3] = ['M', 'A', 'S'];

fn get(arr: &Vec<Vec<char>>, row: i32, col: i32) -> Result<char, ()> {
    if row >= 0
        && row < arr.len().try_into().unwrap()
        && col >= 0
        && col < arr.len().try_into().unwrap()
    {
        return Ok(arr[row as usize][col as usize]);
    }
    Err(())
}

fn search_direction(
    arr: &Vec<Vec<char>>,
    mut row: i32,
    mut col: i32,
    row_inc: i32,
    col_inc: i32,
) -> bool {
    if row_inc == 0 && col_inc == 0 {
        return false;
    }
    let row_b = row;
    let col_b = col;
    for c in MAS {
        row += row_inc;
        col += col_inc;
        match get(arr, row, col) {
            Err(_e) => {
                return false;
            }
            Ok(val) if val != c => {
                return false;
            }
            Ok(_val) => {}
        }
    }
    println!("{} {} {} {}", row_b, col_b, row_inc, col_inc);
    true
}

fn get_count(arr: &Vec<Vec<char>>, row: i32, col: i32) -> u32 {
    let mut count = 0;
    for i in (-1)..(2) {
        for j in (-1)..(2) {
            count += search_direction(arr, row, col, i, j) as u32;
        }
    }
    return count;
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input.len());
    let mut big_arr: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        big_arr.push(line.chars().collect());
    }
    let mut result: u32 = 0;
    println!("{}", big_arr.len());
    for i in 0..big_arr.len() {
        for j in 0..big_arr.len() {
            if (big_arr[i][j] != 'X') {
                continue;
            }
            result += get_count(&big_arr, i as i32, j as i32)
        }
    }
    Some(result)
}

fn is_mas_axis(arr: &Vec<Vec<char>>, row: i32, col: i32, main: bool) -> bool {
    let row_inc = 1;
    let col_inc = if main { 1 } else { -1 };
    let one = match get(arr, row + row_inc, col + col_inc) {
        Err(_e) => {
            return false;
        }
        Ok(val) => val,
    };
    let two = match get(arr, row - row_inc, col - col_inc) {
        Err(_e) => {
            return false;
        }
        Ok(val) => val,
    };
    return (one == 'M' && two == 'S') || (one == 'S' && two == 'M');
}

fn is_mas(arr: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    return is_mas_axis(arr, row, col, true) && is_mas_axis(arr, row, col, false);
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("{}", input.len());
    let mut big_arr: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        big_arr.push(line.chars().collect());
    }
    let mut result: u32 = 0;
    println!("{}", big_arr.len());
    for i in 0..big_arr.len() {
        for j in 0..big_arr.len() {
            if big_arr[i][j] != 'A' {
                continue;
            }
            result += is_mas(&big_arr, i as i32, j as i32) as u32;
        }
    }
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
