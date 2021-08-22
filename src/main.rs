use std::io::{stdin, stdout, Write};

use othello::{
    board::Stone,
    game::Game,
    players::{Player, RandomPlayer},
    utils::input_parse,
};

fn main() {
    let mut game = Game::default();
    let mut input = String::new();
    let mut random_player = RandomPlayer::new(Stone::Whilte);
    while !game.finished {
        println!("turn: {:?}", game.turn);
        println!("{}", game.board);
        if game.turn == Stone::Whilte {
            let (x, y) = random_player.find_move(&game.board).unwrap();
            if let Err(msg) = game.put(x, y) {
                eprintln!("{}", msg);
            }
        } else {
            print!("Where do you put stone? [a-h][0-7]:  ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            println!();
            input = input.strip_suffix('\n').unwrap().to_string();
            match input_parse(&input) {
                Ok((x, y)) => {
                    if let Err(msg) = game.put(x, y) {
                        eprintln!("{}", msg);
                    }
                }
                Err(msg) => eprintln!("{}", msg),
            }
            input.clear();
        }
    }
    println!("Game finished!!")
}
