use std::i32;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should return (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0)
        }

        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score = 
            if player == Player::X {
                -2147483647 // smallest possible i32, since we want to select a higher score
            } else {
                2147483647 // largest possible i32, since we want to select a smaller score
            };
        
        let opponent = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        for possible_move in board.moves() {
            board.apply_move(possible_move, player);

            let (score, _, _) = SolutionAgent::solve(board, opponent, _time_limit);
            board.undo_move(possible_move, player); // by using undo_move, we can avoid cloning by directly reversing applied moves

            if player == Player::X {
                if score > best_score {
                    best_score = score;
                    best_move = possible_move;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = possible_move;
                }
            }
        }
        
        return (best_score, best_move.0, best_move.1)


    }
    
}