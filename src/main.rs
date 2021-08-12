use std::io::{Write, stdin, stdout};

use othello::{board::Square, game::Game};

fn main() {
    let mut g = Game::new();
    while !g.finish() {
        println!("turn: {:?}", g.turn());
        println!("{}", g.board());
        let mut input = String::new();
        print!("Where do you put stone? [a-h][0-7]:  ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        println!();
        let s = Square::parse(input.strip_suffix('\n').unwrap().to_string());
        if let Ok(s) = s {
            println!("{:?}", g.put_stone(s.x, s.y));
        }
    }
    println!("Game finished!!")
}
