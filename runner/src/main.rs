use std::env;
use tetris;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to specify a game to run.");
        std::process::exit(0);
    }

    let game_name = args[1].as_str();

    match game_name.to_lowercase().as_str() {
        "tetris" => tetris::run(),
        _ => println!("Not a valid game name."),
    }
}
