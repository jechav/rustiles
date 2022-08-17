use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::{thread, time};

pub enum GameStatus {
    INIT,
    ONPROGRESS,
    OVER,
}

#[derive(Debug)]
struct Tile (u8, u8);

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
            username: "".to_string(),
            all_tiles: vec![],
            board: vec![],
            player_deck: vec![],
            machine1_deck: vec![], 
            machine2_deck: vec![], 
            machine3_deck: vec![],
            turns: vec![1, 2, 3, 4],
            current_turn: 0,
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
        println!("AFTER SHUFFLE {:?}", self.turns);
    }

    fn get_random_tiles(&mut self) -> Vec<Tile> {
        self.all_tiles.shuffle(&mut thread_rng());
        self.all_tiles.drain(..7).collect::<Vec<Tile>>()
    }

    pub fn step(&mut self) -> u8 {
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

        let mut rng = rand::thread_rng();
        let random_secs = time::Duration::from_secs(rng.gen_range(2..5));
        thread::sleep(random_secs);
        turn
    }

    fn play_user(&self) {
    }
    fn play_machine(&self, _turn: u8) {
    }

    /* PRINT */
    pub fn print(&self) {
        self.print_players_deck();
        self.print_board_deck();
    }

    fn print_players_deck(&self) {
        println!("Player Deck");
        self.print_tiles(Some(&self.player_deck));
        println!("Machine 1 Deck");
        self.print_tiles(Some(&self.machine1_deck));
        println!("Machine 2 Deck");
        self.print_tiles(Some(&self.machine2_deck));
        println!("Machine 3 Deck");
        self.print_tiles(Some(&self.machine3_deck));
    }

    fn print_board_deck(&self) {
        println!("Board");
        self.print_tiles(Some(&self.board));
    }

    fn print_tiles(&self, tiles: Option<&Vec<Tile>>) {
        match tiles {
            Some(t) => print(&t),
            None => print(&self.all_tiles),
        }
        fn print(t_tiles: &Vec<Tile>) {
            println!("\n--------------------------");
            println!("Length {}", t_tiles.len());
            for t in t_tiles {
                print!("{:?} ", t);
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

