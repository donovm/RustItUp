// 012
// 345
// 678

mod board_fns;
use board_fns::get_row;
use board_fns::get_col;
use board_fns::get_diag_up;
use board_fns::get_diag_down;

mod user_input;
use user_input::prompt_user_for_input;
use user_input::is_user_playing_first;

fn check_spot(b: [u32; 3], x: usize, y: usize) -> u32 {
    let idx = (x + y * 3) as usize;
    b[idx]
}

fn is_empty(b: [u32; 3], x: usize, y: usize) -> bool {
    check_spot(b, x, y) == 0
}

fn check_player(b: [u32; 3], x: usize, y: usize, player: u32) -> bool {
    check_spot(b, x, y) == player
}

fn check_slice_for_winner(slice: [u32; 3], player: u32) -> bool {
    (player == slice[0]) && (player == slice[1]) && (player == slice[3])
}

fn check_winner(b: &mut [u32; 9], player: u32) -> u32 {
    for rc in 1..4 {
        let row = get_row(b, rc);
        let col = get_col(b, rc);
        if check_slice_for_winner(row, player) || check_slice_for_winner(col, player) {
            return player;
        }
    }
    if check_slice_for_winner(get_diag_up(b), player) ||
        check_slice_for_winner(get_diag_down(b), player) {
            return player;
    }
    0
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn draw_board(b: &mut [u32; 9]) {
    let mut r = get_row(b, 1);
    clear_console();
    println!("{} | {} | {}", r[0], r[1], r[2]);
    println!("--+---+--");
    r = get_row(b, 2);
    println!("{} | {} | {}", r[0], r[1], r[2]);
    println!("--+---+--");
    r = get_row(b, 3);
    println!("{} | {} | {}", r[0], r[1], r[2]);
}

fn play_robo_turn(b: &mut [u32; 9], player: u32) {

    
}

// TODO: instead of using integers to store player values, use objects and an inherited interface 
// to check occupation of a spot / nullness

fn main() {
    let mut _board: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    if is_user_playing_first() {
        println!("You are player 1!");
    }
    else {
        println!("You are player 2!");
    }

    draw_board(&mut _board);
    
}
