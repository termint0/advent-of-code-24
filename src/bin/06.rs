use std::collections::HashSet;

advent_of_code::solution!(6);

type Board = Vec<Vec<char>>;

const DIM: usize = 130;

fn find_start(board: &Board) -> (usize, usize) {
    board
        .iter()
        .enumerate()
        .filter_map(|(i, ln)| {
            let idx = ln.iter().position(|&c| c == '^');
            if idx.is_some() {
                Some((i, idx.unwrap()))
            } else {
                None
            }
        })
        .nth(0)
        .unwrap()
}
fn valid(pos: (usize, usize)) -> bool {
    pos.0 < DIM && pos.1 < DIM
}

fn add(pos: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    (
        ((pos.0 as i32) + delta.0) as usize,
        ((pos.1 as i32) + delta.1) as usize,
    )
}

fn rotate(delta: (i32, i32)) -> (i32, i32) {
    if delta.0 == 0 {
        (delta.1, 0)
    } else {
        (0, -delta.0)
    }
}

fn get(board: &Board, pos: (usize, usize)) -> char {
    board[pos.0][pos.1]
}

fn get_path(board: &Board, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();

    let mut delta = (-1, 0);
    let mut pos = start;
    while valid(pos) {
        if valid(add(pos, delta)) && get(board, add(pos, delta)) == '#' {
            delta = rotate(delta);
        }
        set.insert(pos);
        pos = add(pos, delta);
    }
    set.remove(&start);
    set
}

fn loopable(board: &Board, start: (usize, usize)) -> bool {
    let mut set = HashSet::new();

    let mut delta = (-1, 0);
    let mut pos = start;
    while valid(pos) {
        if valid(add(pos, delta)) && get(board, add(pos, delta)) == '#' {
            delta = rotate(delta);
            if !set.insert((pos, delta)) {
                return true;
            }
        } else {
            pos = add(pos, delta);
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board: Board = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&board);
    let path = get_path(&board, start);
    let res = path.iter()
        .filter_map(|pos| {
            board[pos.0][pos.1] = '#';
            let res = if loopable(&board, start) {
                Some(())
            } else {
                None
            };
            board[pos.0][pos.1] = '.';
            res
        })
        .count();
    Some(res as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    None
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
//while state.valid() {
//    if state.clone().next().valid() && visit_board.get_st(state.clone().next()) != 'X' {
//        if loopable(board.clone(), state.clone()) {
//            count += 1;
//        }
//        visit_board.set_st(state.clone().next(), 'X')
//    }
//    if state.clone().next().valid() {
//        if board.get_st(state.clone().next()) == '#' {
//            board.set_st(&state, 'X');
//            state.rotate();
//        }
//    }
//    state.next();
//}
