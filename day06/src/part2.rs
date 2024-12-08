use crate::{load_file, out_of_bounds, Direction};

use crate::part1::Guard;
use std::collections::HashSet;

pub fn p2_solution() -> u32 {
    let (board, mut guard) = load_file();

    let default_guard = (guard.dir, guard.x, guard.y);
    let fresh_guard = || Guard::new(default_guard.0, default_guard.1, default_guard.2);

    let mut cycle_count: u32 = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    while let Some((nx, ny)) = verify_next_position(&guard, &board) {
        match board[ny as usize][nx as usize] {
            '.' => {
                if !seen.contains(&(nx, ny)) {
                    cycle_count += simulate_barrier(nx, ny, &mut fresh_guard(), board.clone());
                }
                seen.insert((nx, ny));
                guard.p2_update_pos();
            },
            '#' => guard.turn_ninety_degress(),
            _ => ()
        }
        
    }
    cycle_count
}

fn simulate_barrier(x: i32, y: i32, guard: &mut Guard, mut board: Vec<Vec<char>>) -> u32 {
    let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();
    board[y as usize][x as usize] = '#';

    while !seen.contains(&(guard.x, guard.y, guard.dir)) {
        seen.insert((guard.x, guard.y, guard.dir));

        if let Some((nx, ny)) = verify_next_position(&guard, &board) {
            match board[ny as usize][nx as usize] {
                '.' => guard.p2_update_pos(),
                '#' => guard.turn_ninety_degress(),
                _ => panic!("Not a valid token!")
            }
        } else {
            return 0
        }
    }
    1    // found a cycle
}

fn verify_next_position(guard: &Guard, board: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    let (x, y,) = guard.next_pos();
    if !out_of_bounds(x, y, board) {
        Some((x, y))
    } else {
        None
    }
}
