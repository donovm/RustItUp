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



fn clear_console() {
    print!("{}[2J", 27 as char);
}



// TODO: instead of using integers to store player values, use objects and an inherited interface 
// to check occupation of a spot / nullness

fn main() {
    let mut _board: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    if is_user_playing_first() {
        println!("You're player 1!");
    }
    else {
        println!("You're player 2!");
        
    }

    while !
    }

    draw_board(&mut _board);
    
}
