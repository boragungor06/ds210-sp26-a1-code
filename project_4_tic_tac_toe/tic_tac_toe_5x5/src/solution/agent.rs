use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {

        // this helper will keep track of recursion depth
        fn solve_helper(board: &mut Board, player: Player, _time_limit: u64, depth: u32, max_depth: u32) -> (i32, usize, usize) {
        
            if board.game_over() || depth == max_depth {
                return (SolutionAgent::heuristic(board), 0, 0)
            }
            let mut best_move = None;
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

                let (score, _, _) = solve_helper(board, opponent, _time_limit, depth + 1, max_depth);
                board.undo_move(possible_move, player); // by using undo_move, we can avoid cloning by directly reversing applied moves


                if player == Player::X {
                    if score > best_score {
                        best_score = score;
                        best_move = Some(possible_move);
                    }
                } else {
                    if score < best_score {
                        best_score = score;
                        best_move = Some(possible_move);
                    }
                }
            }
            
            let (row, col) = best_move.unwrap_or((0,0));
            return (best_score, row, col)
        }
        
        // max_depth determines the current max depth. with more code optimization, max_depth could be increased.
        let max_depth = 4; 
        return solve_helper(board, player, _time_limit, 0, max_depth)
    }

}

impl SolutionAgent {
     fn heuristic(board: &Board) -> i32 {
        // points beat potential points; any 3 in a row is amplified 1000-fold
        let mut score = board.score() * 1000;
        
        // centrality
        let cells = board.get_cells();
        let len = cells.len();
        for row in 0..len {
            for col in 0..len {
                let cell = &cells[row][col];

                let centrality = 
                    // dead center; preferrable (3)
                    if row == 2 && col == 2 {
                        3
                    }
                    // inner ring; less potential (1)
                    else if row >= 1 && row <= 3 && col >= 1 && col <= 3 {
                        1
                    }
                    // outer ring; least potential (0)
                    else {
                        0
                    };
                
                if let tic_tac_toe_stencil::board::Cell::X = cell {
                    score += centrality;
                } else if let tic_tac_toe_stencil::board::Cell::O = cell {
                    score -= centrality;
                }                          
            }
        } 
        return score
    }
}