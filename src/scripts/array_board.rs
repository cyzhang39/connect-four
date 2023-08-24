const HEIGHT: usize = 6; 
const WIDTH: usize = 7;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cell {
    Red,
    Yellow,
    Empty
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    Win,
    Loss,
    Tie,
    Default,
}

use GameState::{Win, Loss, Tie, Default};

pub struct ArrayBoard {
    pub board: [Cell; WIDTH * HEIGHT], //board where coins will be stored in row-major order from bottom to top
    num_moves: usize, //total number of moves played in the current game
    pub red_turn: bool, //used to signify whose turn it is
    heights: [usize; WIDTH], //height of each column (number of coins in each col)
    pub moves: String, //numeric string to signify sequence of moves
    pub state: GameState, //current game state
}

//Implementation of board
impl ArrayBoard {
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; WIDTH * HEIGHT], 
            num_moves: 0,
            red_turn: true,
            heights: [0; WIDTH],
            moves: String::new(),
            state: Default,
        }
    }

    pub fn play_turn(&mut self, column: usize) -> Result<GameState, String> {
        if !self.is_move_valid(column) {
            return Err("Column is full. Choose another move!".to_string());
        }

        if self.is_winning_move(column) {
            self.state = if self.red_turn {Win} else {Loss};
        } else if self.is_draw() {
            self.state = Tie;
        } else {
            self.state = Default;
        }
        
        self.play_move(column);
        self.moves.push_str(&column.to_string());

        return Ok(self.state);
    }

    pub fn undo_move(&mut self, column: usize) -> Result<GameState, String> {
        if self.heights[column] == 0 {
            return Err("Column in empty!".to_string())
        }

        self.red_turn = !self.red_turn;
        self.num_moves -= 1;
        self.board[WIDTH*(self.heights[column] - 1) + column] = Cell::Empty;
        self.heights[column] -= 1;
        
        Ok(self.state)
    }

    pub fn play_move(&mut self, column: usize) { 
        let coin = if self.red_turn {
            Cell::Red
        } else {
            Cell::Yellow
        };

        self.num_moves += 1;
        //Find height and multiply by width to find index of row and then add column offset
        self.board[WIDTH*self.heights[column] + column] = coin;
        self.heights[column] += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn is_move_valid(&self, column: usize) -> bool {
        return self.heights[column] < HEIGHT;
    }

    pub fn is_draw(&self) -> bool {
        return self.num_moves == (WIDTH * HEIGHT);
    }

    pub fn get_num_moves(&self) -> usize {
        return self.num_moves;
    }

    pub fn is_winning_move(&self, column: usize) -> bool {
        let coin = if self.red_turn {
            Cell::Red
        } else {
            Cell::Yellow
        };

        //Check vertical direction 
        if self.heights[column] >= 3 
        && self.board[WIDTH*(self.heights[column] - 1) + column] == coin
        && self.board[WIDTH*(self.heights[column] - 2) + column] == coin
        && self.board[WIDTH*(self.heights[column] - 3) + column] == coin {
            return true;
        }       
        
        //Check horizontal direction
        let mut horiz_count = 1;
        let mut x = 1;
        //Check rightward direction
        while column + x < WIDTH && self.board[WIDTH*self.heights[column] + column + x] == coin {
            x += 1;
            horiz_count += 1;
        }
        
        //Check leftward direction
        x = 1;
        while column >= x && self.board[WIDTH*self.heights[column] + column - x] == coin {
            x += 1;
            horiz_count += 1;
        }
        
        if horiz_count >= 4 {
            return true;
        }
        
        //Check diagonals
        let mut left_diag_count = 1;
        let mut direction = 1;
        
        //Check upper left direction
        while column >= direction && direction + self.heights[column] < HEIGHT 
                && self.board[WIDTH*(self.heights[column] + direction) + column - direction] == coin {
            direction += 1;
            left_diag_count += 1;
        }
        
        //Check bottom right direction
        direction = 1;
        while column + direction < WIDTH && self.heights[column] >= direction 
                && self.board[WIDTH*(self.heights[column] - direction) + column + direction] == coin {
            direction += 1;
            left_diag_count += 1;
        }
        
        //Check if left diagonal has 4 or more contiguous coins 
        if left_diag_count >= 4 {
            return true;
        }
        
        let mut right_diag_count = 1;
        direction = 1;
        
        //Check upper right direction
        while column + direction < WIDTH && direction + self.heights[column] < HEIGHT 
                && self.board[WIDTH*(self.heights[column] + direction) + column + direction] == coin {
            direction += 1;
            right_diag_count += 1;
        }
        //Check bottom left direction
        direction = 1;
        while column >= direction && self.heights[column] >= direction 
                && self.board[WIDTH*(self.heights[column] - direction) + column - direction] == coin {
            direction += 1;
            right_diag_count += 1;
        }
        
        //Check if right diagonal has 4 or more contiguous coins 
        if right_diag_count >= 4 {
            return true;
        }
        
        //Not winning move
        return false;
    }
} 
