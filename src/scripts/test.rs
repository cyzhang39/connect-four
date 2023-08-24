use std::convert::TryInto;
const HEIGHT: usize = 6; 
const WIDTH: usize = 7;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    Win,
    Loss,
    Tie,
    Default,
}

use GameState::{Win, Loss, Tie, Default};

pub struct BitBoard {
    pub player_mask: u64, //first 49 bits used to store current player position
    pub total_mask: u64, //first 49 bits used to store all played coins
    pub bottom_row: u64,
    num_moves: usize, //total number of moves played in the current game
    pub red_turn: bool, //used to signify whose turn it is
    pub state: GameState, //current game state
}

//Implementation of board
impl BitBoard {
    pub fn new() -> Self {
        let mut temp:u64 = 1;

        for col in 1..WIDTH {
            temp |= 1 << (HEIGHT + 1) * col
        }

        Self {
            player_mask: 0, 
            total_mask: 0,
            bottom_row: temp,
            num_moves: 0,
            red_turn: true,
            state: Default,
        }
    }

    pub fn top_col_mask(&self, col: usize) -> u64 {
        return (1 << (HEIGHT - 1)) << ((HEIGHT + 1) * col);
    }

    pub fn bottom_col_mask(&self, col: usize) -> u64 {
        return 1 << ((HEIGHT + 1) * col);
    }

    pub fn full_col_mask(&self, col: usize) -> u64 {
        return ((1 << HEIGHT) - 1) << ((HEIGHT + 1) * col);
    }

    pub fn is_move_valid(&self, col: usize) -> bool {
        return self.total_mask & self.top_col_mask(col) == 0;
    }

    pub fn get_height_mask(&self) -> u64 {
        return self.total_mask + self.bottom_row;
    }

    pub fn undo_move(&mut self, col: usize) -> Result<GameState, String> {

        if self.total_mask & self.bottom_col_mask(col) == 0 {
            return Err("Column in empty!".to_string())
        }
        
        self.red_turn = !self.red_turn;
        self.num_moves -= 1;
        let yellow_mask:u64 = self.player_mask ^ self.total_mask;
        self.total_mask ^= (((self.total_mask + self.bottom_col_mask(col))) & self.get_height_mask()) >> 1;        

        if self.red_turn {
            self.player_mask = yellow_mask ^ self.total_mask;
        }

        Ok(self.state)
    }

    pub fn play_move(&mut self, col: usize) { 

        let yellow_mask:u64 = self.player_mask ^ self.total_mask;
        self.total_mask |= self.total_mask + self.bottom_col_mask(col);

        if self.red_turn {
            self.player_mask = yellow_mask ^ self.total_mask;
        }

        self.num_moves += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn play_turn(&mut self, col: usize) -> Result<GameState, String> {
        if !self.is_move_valid(col) {
            return Err("Column is full. Choose another move!".to_string());
        }

        if self.is_winning_move(col) {
            self.state = if self.red_turn {Win} else {Loss};
        } else if self.is_draw() {
            self.state = Tie;
        } else {
            self.state = Default;
        }
        
        self.play_move(col);

        return Ok(self.state);
    }

    pub fn is_draw(&self) -> bool {
        return self.num_moves == (WIDTH * HEIGHT);
    }

    pub fn get_num_moves(&self) -> usize {
        return self.num_moves;
    }

    pub fn is_winning_move(&mut self, col: usize) -> bool {
        self.play_move(col);
        let position:u64 = if self.red_turn {self.total_mask ^ self.player_mask}  else {self.player_mask};
        println!("{:b}", position);
        self.undo_move(col);
        
        //horizontal direction
        let n:u64 = position & (position >> (HEIGHT + 1));
        println!("{:b}", n);
        if n & (n >> 2 * (HEIGHT + 1)) > 0 {
            return true;
        }

        //vertical direction
        let n:u64 = position & (position >> 1);
        println!("{:b}", n);

        if n & (n >> 2) > 0 {
            return true;
        }

        //diagonal = (/) direction
        let n:u64 = position & (position >> (HEIGHT - 1));
        println!("{:b}", n);
        if n & (n >> 2 * (HEIGHT - 2)) > 0 {
            return true;
        }
        
        //diagonal = (\) direction
        let n:u64 = position & (position >> (HEIGHT + 2));
        println!("{:b}", n);
        if n & (n >> 2 * (HEIGHT + 2)) > 0 {
            return true;
        }

        return false;
    }
} 


fn main() {
    let mut bit_board = BitBoard::new();
    
    bit_board.play_move(3);
    bit_board.play_move(4);
    bit_board.play_move(4);
    bit_board.play_move(5);
    bit_board.play_move(6);
    bit_board.play_move(5);
    bit_board.play_move(5);
    bit_board.play_move(5);
    bit_board.play_move(6);
    bit_board.play_move(1);
    bit_board.play_move(6);
    bit_board.play_turn(6);
}