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
        if board.game_over() {
            return (board.score(),1,1)
        }
        let mut best_move: (usize,usize) =(0,0);
        let mut best_score = match player {
            Player::O => 100,
            Player::X => -100,
        };
        for i in board.moves(){
            let mut board2 = board.clone();
            board2.apply_move(i,player);
            let mut score = SolutionAgent::solve(&mut board2,
                                                                        match player{
                                                                            Player::O => Player::X,
                                                                            Player::X => Player::O
                                                                        },_time_limit);
            match player{
                Player::X =>if score.0 > best_score{
                best_score = score.0;
                best_move = i;
            }
                Player::O =>if score.0 < best_score{
                best_score = score.0;
                best_move = i;
            }
        }
        }
    


        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        return (best_score,best_move.0,best_move.1)
    }
}
