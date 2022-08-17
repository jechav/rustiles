use colored::Colorize;
use std::io;
use std::fmt;
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::{thread, time};

#[derive(Debug, Default)]
pub enum GameStatus {
    #[default]
    INIT,
    ONPROGRESS,
    OVER,
}

#[derive(Debug)]
struct Tile (u8, u8);

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]",
            format!("{}", self.0).red(),  format!("{}", self.1).green())
    }
}
#[derive(Debug, Default)]
pub struct Game {
    status: GameStatus,
    username: String,
    all_tiles: Vec<Tile>,
    board: Vec<Tile>,
    player_deck: Vec<Tile>,
    machine1_deck: Vec<Tile>, 
    machine2_deck: Vec<Tile>, 
    machine3_deck: Vec<Tile>,
    turns: Vec<u8>,
    current_turn: usize,
}


impl Game {
    pub fn new(status: GameStatus) -> Game {
        Game {
            status,
            turns: vec![1, 2, 3, 4],
            current_turn: 0,
            ..Default::default()
        }
    }


    pub fn start(&mut self) {
        self.deal_tiles();
        self.assign_turn();
    }

    fn deal_tiles(&mut self) {
        for i in 0..7 {
            for k in 0..7 {
                if i >= k {
                    self.all_tiles.push(Tile(i, k))
                }
            }
        }
        self.player_deck = self.get_random_tiles();
        self.machine1_deck = self.get_random_tiles();
        self.machine2_deck = self.get_random_tiles();
        self.machine3_deck = self.get_random_tiles();
    }

    fn assign_turn(&mut self) {
        self.turns.shuffle(&mut thread_rng());
        println!("TURNS AFTER SHUFFLE {:?}", self.turns);
    }

    fn get_random_tiles(&mut self) -> Vec<Tile> {
        self.all_tiles.shuffle(&mut thread_rng());
        self.all_tiles.drain(..7).collect::<Vec<Tile>>()
    }

    pub fn play(&mut self) {
        let turn = self.turns[self.current_turn];
        if turn == 1 {
            self.play_user();
        } else {
            self.play_machine(turn);
        }
        self.current_turn+=1;
        if self.current_turn >= self.turns.len() {
            self.current_turn = 0;
        }
    }

    fn play_user(&self) {
        println!("Inset your move");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Your move {}", input);
            }
            Err(error) => println!("error: {error}"),
        }
    }

    fn play_machine(&self, _turn: u8) {
        // Add timeout of machine Play
        let mut rng = rand::thread_rng();
        let random_secs = time::Duration::from_secs(rng.gen_range(2..5));
        thread::sleep(random_secs);
    }

    pub fn check(&self) {
    }

    /* PRINT */
    pub fn print(&self) {
        self.print_players_deck();
        self.print_board_deck();
    }

    fn print_players_deck(&self) {
        println!("{:_^32}", "Player Deck".bold().red());
        self.print_tiles(Some(&self.player_deck), Some(true));
        println!("{:_^32}", "Machine 1 Deck");
        self.print_tiles(Some(&self.machine1_deck), None);
        println!("{:_^32}", "Machine 2 Deck");
        self.print_tiles(Some(&self.machine2_deck), None);
        println!("{:_^32}", "Machine 3 Deck");
        self.print_tiles(Some(&self.machine3_deck), None);
    }

    fn print_board_deck(&self) {
        println!("Board");
        self.print_tiles(Some(&self.board), None);
    }

    fn print_tiles(&self, tiles: Option<&Vec<Tile>>, with_number: Option<bool>) {
        match tiles {
            Some(t) => print(&t, with_number),
            None => print(&self.all_tiles, with_number),
        }
        fn print(t_tiles: &Vec<Tile>, with_number: Option<bool>) {
            println!("Length {}", t_tiles.len());
            for (ind, t)in t_tiles.iter().enumerate() {
                if with_number.unwrap_or(false) == true {
                 print!("{}-{} ", format!("#{}", ind).bold().on_green().white(), t);
                } else {
                    print!("{} ", t);
                }
            }
            println!("\n--------------------------");
            println!();
        }
    }
    /* END PRINT */


    pub fn on_over(&self) {
        println!("Game over");
    }

    pub fn set_status(&mut self, status: GameStatus) {
        self.status = status
    }

    pub fn get_status(&self) -> &GameStatus {
        &self.status
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}

