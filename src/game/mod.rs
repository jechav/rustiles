mod game_struct;

use game_struct::GameStatus;
pub use game_struct::Game;

pub fn run() {
    println!("Game started");

    let mut game = Game::new(GameStatus::INIT);

    loop {
        match game.get_status() {
            GameStatus::INIT => controller::run_init(&mut game),
            GameStatus::ONPROGRESS => controller::run_play(&mut game),
            GameStatus::OVER => controller::run_over(&mut game),
        }
    }
}

mod controller {
    use std::io;
    use super::*;

    pub fn run_init(game: &mut Game) {
        println!("Type your username");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                game.set_username(&input);
                game.set_status(GameStatus::ONPROGRESS);
                println!("Hi -> {}", game.get_username());
                game.start();
            }
            Err(error) => println!("error: {error}"),
        }
    }

    pub fn run_play(game: &mut Game) {
        game.print();
        let turn = game.step();
        println!("=======TURN {}", turn);
        // 2.1 wait play 
        // 2.2 machine play
        // 3. check status game
        // game.set_status(GameStatus::OVER)
    }

    pub fn run_over(game: &mut Game) {
        game.on_over();
        game.set_status(GameStatus::INIT);
    }
}

