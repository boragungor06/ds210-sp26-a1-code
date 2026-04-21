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


                // now implements alpha-beta pruning: we can ignore searching over branches
                // that would not be considered by a rational player since better moves have
                // already been found
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
        
        
        let cells = board.get_cells();
        let len = cells.len();
        
        // centrality
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

        // consecutive X/O counting
         for row in 0..len {
            for col in 0..len {
                if col + 2 < len {
                   score += Self::count_consecutive_cells(&cells[row][col], &cells[row][col+1], &cells[row][col+2]); 
                }

                if row + 2 < len {
                    score += Self::count_consecutive_cells(&cells[row][col], &cells[row+1][col], &cells[row+2][col]);
                }

                if row + 2 < len && col + 2 < len {
                    score += Self::count_consecutive_cells(&cells[row][col], &cells[row+1][col+1], &cells[row+2][col+2]);
                }

                if row + 2 < len && col >= 2 {
                    score += Self::count_consecutive_cells(&cells[row][col], &cells[row+1][col-1], &cells[row+2][col-2]);
                }
            }
        } 

        return score
    }

    // takes a trio of consecutive cells, and counts the number of X's, O's, empty cells, and accounts for walls
    // returns a score based on strategic importance of the cell group. 
    // invalid trios (if there are walls) will are successfully ignored by the heuristic
     fn count_consecutive_cells(c1: &tic_tac_toe_stencil::board::Cell, c2: &tic_tac_toe_stencil::board::Cell, c3: &tic_tac_toe_stencil::board::Cell) -> i32 {
        use tic_tac_toe_stencil::board::Cell;

        let mut x_count = 0;
        let mut o_count = 0;
        let mut free_count = 0;

        for cell in [c1, c2, c3] {
            match cell {
                Cell::X => x_count += 1,
                Cell::O => o_count += 1,
                Cell::Empty => free_count += 1,
                _ => {} // Walls or anything else are ignored, remember that _ is the catch-all for match statements
            }
        }
        
        if x_count == 2 && free_count == 1 {
            50  // X has a major threat (Open 2-in-a-row)
        } else if o_count == 2 && free_count == 1 {
            -50 // O has a major threat 
        } else if x_count == 1 && free_count == 2 {
            5   // X is building a line (Open 1-in-a-row)
        } else if o_count == 1 && free_count == 2 {
            -5  // O is building a line
        } else {
            0   
        }
    } 
}