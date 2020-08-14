extern crate rand;

use rand::Rng;
use board_fns::get_winning_move;

fn generate_random_move() -> (u32, u32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0, 3), rng.gen_range(0, 3))
}

fn get_random_move(board: [u32; 9]) -> (u32, u32) {
    let mut mv: (u32, u32) = generate_random_move();

    while !is_empty(board, mv[0], mv[1]) {
        mv = generate_random_move();
    }
    mv
}

fn get_double_move(board: [u32; 9]) -> i32 {
    let rows = get_rows(board);
    let cols= get_cols(board);
    l
}

pub fn get_robot_move(board: [u32], robot_player: u32, player: u32) => u32 {
    // Check for winning moves
    let winning_move = get_winning_move(board, robot_player);
    if winning_move > -1 {
        return winning_move;
    }

    // Check for blocking moves
    let blocking_move = get_winning_move(board, player);
    if blocking_move > -1 {
        return blocking_move;
    }

    // Make a random move
    return = get_random_move();

    //TODO make calculated move
}
