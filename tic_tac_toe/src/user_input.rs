use std::io;
use std::io::*;
pub fn prompt_user_for_input(prompt: &str) -> &str{ 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input
}

pub fn is_user_playing_first() -> bool {
    let prompt = "Would you like to play first? (y/n)";
    let valid_responses = ["y", "Y", "n", "N"];
    let mut ui: str = "";
    loop {
        ui = prompt_user_for_input(prompt);
        if ui.chars().count() == 1 && valid_responses
            .into_iter().any(|v| v == ui) {
            return ui == "y" || ui == "Y";
        }
        println!("Input {} not valid", ui);
    }
    false
}
