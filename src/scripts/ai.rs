use crate::scripts::bit_board::{BitBoard, GameState};
use crate::scripts::trans_table::{TranspositionTable};


const HEIGHT: usize = 6;
const WIDTH: usize = 7;
const MIN_SCORE: i64 = -(7 * 6) / 2 + 3;
const MAX_SCORE: i64 = (7 * 6 + 1) / 2 - 3;


pub struct AIGame {
    column_order: [i64; WIDTH],
}


impl AIGame {
    pub fn new() -> Self {
            let mut column_order = [0; WIDTH];


            for i in 0..WIDTH {
                column_order[i] = WIDTH as i64 / 2 + (1 - 2 * (i as i64 % 2)) * (i as i64 + 1) / 2;
            }


            AIGame {
                column_order
            }
    }


    pub fn make_move(&self, game: &mut BitBoard, trans_table: &mut TranspositionTable) -> Result<GameState, String> {
        let mut best_move = 0;
        let mut best_score = std::i64::MIN;


        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap();


            if game.is_move_valid(chosen_col) {
                // if game.is_winning_move(chosen_col) {
                //     return game.play_turn(chosen_col);
                // }


                let init:i64 = ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -init, init, 39);
                let _ = game.undo_move(chosen_col);


                if score > best_score {
                    best_move = chosen_col;
                    best_score = score;
                }
            }
        }


        return game.play_turn(best_move);
    }


    pub fn negamax(&self, game: &mut BitBoard, trans_table: &mut TranspositionTable, mut alpha: i64, mut beta: i64, depth: i64) -> i64 {
        if game.get_num_moves() >= WIDTH * HEIGHT - 2 {
            return 0;
        } else if depth == 0 {
            return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
        }


        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
            }
        }


        let mut min = -(((WIDTH * HEIGHT - 2 - game.get_num_moves()) / 2) as i64);


        if alpha < min {
            alpha = min;                    
           
            if alpha >= beta {
                return alpha;
            }
        }


        let mut max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) as i64 / 2;




        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
           
            if alpha >= beta {
                return beta;
            }
        }


        let key = game.get_unique_key();
        let val = trans_table.get(key);
        if val != 0 {
            if val as i64 > (MAX_SCORE - MIN_SCORE + 1) {
                min = val as i64 + 2*MIN_SCORE - MAX_SCORE - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta {return alpha;}
                }
            }else{
                max = val as i64 + MIN_SCORE - 1;
                if beta > max {
                    beta = max;
                    if alpha >= beta {return beta;}
                }
            }
        }




        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap();


            if game.is_move_valid(chosen_col) {
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -beta, -alpha, depth - 1);
                let _ = game.undo_move(chosen_col);


                if score >= beta {
                    trans_table.insert(key, (score + MAX_SCORE - 2*MIN_SCORE + 2) as u64);
                    return score;
                }  


                if score > alpha {
                    alpha = score;
                }
            }
        }


        trans_table.insert(key, (alpha - MIN_SCORE + 1) as u64);
        return alpha;
    }
}



