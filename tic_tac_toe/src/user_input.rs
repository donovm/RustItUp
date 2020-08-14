use std::io;
use std::io::*;
pub fn prompt_user_for_input(prompt: &str) -> String { 
    println!("{}", &prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    String::from(input)
}

pub fn is_user_playing_first() -> bool {
    let prompt = "Would you like to play first? (y/n)";
    let valid_responses = ["y", "Y", "n", "N"];
    loop {
        let ui = prompt_user_for_input(prompt);
        let t = ui.trim();
        if t.chars().count() == 1 && valid_responses
            .into_iter().any(|v| v == &t) {
            return t== "y" || t== "Y";
        }
        println!("Input {} not valid", t);
    }
    false
}
