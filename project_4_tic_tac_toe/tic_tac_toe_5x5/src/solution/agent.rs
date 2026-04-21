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
        fn solve_helper(board: &mut Board, player: Player, _time_limit: u64, depth: u32, max_depth: u32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
        
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

                let (score, _, _) = solve_helper(board, opponent, _time_limit, depth + 1, max_depth, alpha, beta);
                board.undo_move(possible_move, player); // reversing moves during recursive unwinding instead of cloning during winding


                if player == Player::X {
                    if score > best_score {
                        best_score = score;
                        best_move = Some(possible_move);
                    }
                    alpha = alpha.max(best_score);
                } else {
                    if score < best_score {
                        best_score = score;
                        best_move = Some(possible_move);
                    }
                    beta = beta.min(best_score);
                }
                if alpha >= beta { break; }
            }
            
            let (row, col) = best_move.unwrap_or((0,0));
            return (best_score, row, col)
        }
        
        // max_depth determines the current max depth. with more code optimization, max_depth could be increased.
        let max_depth = 5; 
        return solve_helper(board, player, _time_limit, 0, max_depth, -2147483647, 2147483647)
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

                let center: i32 = 2; // center of the board
                let rowi32: i32 = row as i32; // I change it to i32 so it can go negative
                let coli32: i32 = col as i32;

                let rowdifference = (rowi32 - center).abs(); // verticle distance
                let columndifference = (coli32 - center).abs(); // horizontal distance
                let dist = rowdifference + columndifference; // total distance

                let centrality = 5 - dist;// if it is closer to center it will get a higher value
                
                if let tic_tac_toe_stencil::board::Cell::X = cell {
                    score += centrality;
                } else if let tic_tac_toe_stencil::board::Cell::O = cell {
                    score -= centrality;
                }                          
            }
        } 
        // nearby cell check
        for r in 0..len { //row
            for c in 0..len { //column
                let cell = &cells[r][c];

                // figure out if this cell is X or O
                let x_cell;
                let o_cell;
                if let tic_tac_toe_stencil::board::Cell::X = cell {
                    x_cell = true;
                    o_cell = false;
                } else if let tic_tac_toe_stencil::board::Cell::O = cell {
                    x_cell = false;
                    o_cell = true;
                } else {
                    continue; // skip empty cells and walls
                }

                // count friendly neighbors and empty neighbors
                let mut same_count = 0;
                let mut open_count = 0;

                // check all 8 neighbors
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 { continue; } // skip self
                        let adj_r = r as i32 + dr;
                        let adj_c = c as i32 + dc;
                        if adj_r < 0 || adj_r >= len as i32 || adj_c < 0 || adj_c >= len as i32 { continue; }

                        let adj = &cells[adj_r as usize][adj_c as usize];

                        if x_cell {
                            if let tic_tac_toe_stencil::board::Cell::X = adj {
                                same_count += 1;
                            }
                        }
                        if o_cell {
                            if let tic_tac_toe_stencil::board::Cell::O = adj {
                                same_count += 1;
                            }
                        }
                        if let tic_tac_toe_stencil::board::Cell::Empty = adj {
                            open_count += 1;
                        }
                    }
                }

                let val = same_count * 15 + open_count * 5;

                if x_cell { score += val; }
                if o_cell { score -= val; }
            }
        }
        return score
    }
}