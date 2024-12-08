mod part1;
use part1::{p1_solution, Guard};

mod part2;
use part2::p2_solution;

fn main() {
    let ret1 = p1_solution();
    assert_eq!(5312, ret1);
    println!("Solution 1 complete..");

    let ret2 = p2_solution();
    assert_eq!(1748, ret2);
    println!("Solution 2 complete..");
}

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
enum Direction {
    North, South, East, West
}

fn out_of_bounds(x: i32, y: i32, board: &Vec<Vec<char>>) -> bool {
    x < 0 || x as usize >= board[0].len() || y < 0 || y as usize >= board.len()
}

pub fn load_file() -> (Vec<Vec<char>>, Guard) {
    let mut guard: Option<Guard> = None;
    let board = std::fs::read_to_string("input.txt").expect("Cannot read input!")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut chars: Vec<char> = line.chars().collect();
            if let Some(j) = chars.iter().position(|ch| *ch == '^' || *ch == 'v' || *ch == '<' || *ch == '>') {
                match chars[j] {
                    '^' => guard = Some(Guard::new(Direction::North, j as i32, i as i32)),
                    '>' => guard = Some(Guard::new(Direction::East, j as i32, i as i32)),
                    '<' => guard = Some(Guard::new(Direction::West, j as i32, i as i32)),
                    'v' => guard = Some(Guard::new(Direction::South, j as i32, i as i32)),
                    _ => (),
                }
                chars[j] = '.';
            }
            chars
        })
        .collect::<Vec<Vec<char>>>();
    
    match guard {
        Some(g) => (board, g),
        None => panic!("No guard found!")
    }
    
}


