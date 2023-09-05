use crate::scripts::bit_board::{BitBoard, GameState};
// use crate::scripts::array_board::{ArrayBoard, GameState};
use crate::scripts::trans_table::{TranspositionTable};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct AIGame {
    column_order: [i64; WIDTH],
    pub debug: String,
    pub bound: usize,
}

impl AIGame {
    pub fn new() -> Self {
            let mut column_order = [0; WIDTH];

            for i in 0..WIDTH {
                column_order[i] = WIDTH as i64 / 2 + (1 - 2 * (i as i64 % 2)) * (i as i64 + 1) / 2;
            }

            AIGame {
                column_order,
                debug: String::new(),
                bound: 0,
            }
    }

    pub fn make_move(&mut self, game: &mut BitBoard, trans_table: &mut TranspositionTable) -> Result<GameState, String> {
        let mut best_move: usize = 0;
        let mut best_score: i64 = std::i64::MIN;
        let mut depth: i64 = 20;
        self.debug = String::new();
        self.bound = game.get_num_moves();
        // self.debug.push_str(&game.total_mask.to_string());
        // self.debug.push_str(&format!("{:b}", game.total_mask));

        if game.total_mask == 400556032 {        
            return game.play_turn(5);
        } else if game.total_mask == 35297165312 {
            return game.play_turn(4);
        }

        // if self.bound <= 6 {
        //     depth = 15;
        // } 

        // if self.bound == 7 {
        //     trans_table.reset();
        // }

        for col in 0..WIDTH {
            let chosen_col: usize = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                if game.is_winning_move(chosen_col) {
                    return game.play_turn(chosen_col);
                }

                let init:i64 = ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -init, init, depth);
                // self.debug.push_str(&format!("col:{} |", chosen_col.to_string()));

                // self.debug.push_str(&format!("(col:{}, score:{}) ", (6 - chosen_col).to_string(), score.to_string()));
                let _ = game.undo_move(chosen_col);

                if score > best_score {
                    best_move = chosen_col;
                    // self.debug.push_str(&format!("best:{} |", best_move.to_string()));
                    best_score = score;
                }
            }
        }
        
        // self.debug.push_str(&format!("best:{} |", best_move.to_string()));
        return game.play_turn(best_move);
    }

    pub fn negamax(&self, game: &mut BitBoard, trans_table: &mut TranspositionTable, mut alpha: i64, mut beta: i64, depth: i64) -> i64 {
        if game.get_num_moves() >= WIDTH * HEIGHT - 2 {
            return 0;
        } 

        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
            }
        }

        let min = -(((WIDTH * HEIGHT - 2 - game.get_num_moves()) / 2) as i64);

        if alpha < min {
            alpha = min;                     
            
            if alpha >= beta {
                return alpha; 
            }
        }

        let mut max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) as i64 / 2;
        let val = trans_table.get(game.get_unique_key());
        if val != 0 {
            max = val as i64 - (WIDTH * HEIGHT / 2) as i64 + 3;
        }

        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
            
            if alpha >= beta {
                return beta;
            }
        }
        
        if depth == 0 {
            // return alpha;
            return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
        }

        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -beta, -alpha, depth - 1);
                let _ = game.undo_move(chosen_col);

                if score >= beta {
                    return score;
                }   

                if score > alpha {
                    alpha = score;
                }
            }
        }

        trans_table.insert(game.get_unique_key(), (alpha + (WIDTH * HEIGHT / 2) as i64 - 3) as u64);
        return alpha;
    }
}