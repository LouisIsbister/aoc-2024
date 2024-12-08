
use crate::{load_file, out_of_bounds, Direction};

pub fn p1_solution() -> u64 {
    let (mut board, mut guard) = load_file();
    let mut ret: u64 = 1;
    loop {
        let (x, y) = guard.next_pos();
        if out_of_bounds(x, y, &board) {
            return ret
        }

        let cur_pos = board[y as usize][x as usize]; 
        match cur_pos {
            '_' => guard.p1_update_pos(&mut board),
            '.' => {
                guard.p1_update_pos(&mut board);
                ret += 1
            },
            '#' => guard.turn_ninety_degress(),
            _ => ()
        }        
    }
}

#[derive(Clone)]
pub struct Guard {
    pub dir: Direction,
    pub x: i32,
    pub y: i32,
}

impl Guard {
    pub fn new(dir: Direction, x: i32, y: i32) -> Self {
        Self { dir, x, y }
    }

    pub fn turn_ninety_degress(&mut self) {
        match self.dir {
            Direction::North => self.dir = Direction::East,
            Direction::East => self.dir = Direction::South,
            Direction::South => self.dir = Direction::West,
            Direction::West => self.dir = Direction::North,
        }
    }

    pub fn p1_update_pos(&mut self, board: &mut Vec<Vec<char>>) {
        let (x, y) = self.next_pos();
        self.x = x;
        self.y = y;
        board[self.y as usize][self.x as usize] = '_'   // visit cell
    }

    pub fn p2_update_pos(&mut self) {
        let (x, y) = self.next_pos();
        self.x = x;
        self.y = y;
    }

    pub fn next_pos(&self) -> (i32, i32) {
        match self.dir {
            Direction::North => (self.x, self.y - 1),
            Direction::East =>  (self.x + 1, self.y),
            Direction::South => (self.x, self.y + 1),
            Direction::West =>  (self.x - 1, self.y),
        }
    }
}