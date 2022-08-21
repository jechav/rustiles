use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::io;
use std::{thread, time};

#[derive(Debug, Default)]
pub enum GameStatus {
    #[default]
    INIT,
    ONPROGRESS,
    OVER,
}

#[derive(Debug, Copy, Clone)]
enum Move {
    INIT,
    HEAD,
    TAIL,
    PASS,
    BOTH,
}

#[derive(Debug)]
struct Tile(u8, u8);

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}]",
            format!("{}", self.0).red(),
            format!("{}", self.1).green()
        )
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
    pub fn new(status: GameStatus) -> Self {
        Self {
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
        self.current_turn += 1;
        if self.current_turn >= self.turns.len() {
            self.current_turn = 0;
        }
    }

    fn play_user(&mut self) -> bool {
        if !self.check_deck_for_available_move(&self.player_deck) {
            println!("You pass");
            return false;
        }

        println!("Inset your move");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // TODO: check types
                let num = input.trim().parse::<usize>();

                if !num.is_ok() {
                    println!("{}", format!("INVALID INPUT").red());
                    return self.play_user();
                }

                let num = num.unwrap();
                let tile_move = self.player_deck.get(num);

                if !tile_move.is_some() {
                    println!("{}", format!("TIlE NOT FOUND INDEX {}", num).red());
                    return self.play_user();
                }

                let tile_move = tile_move.unwrap();
                let mut move_dir = self.check_tile_move(tile_move);
                println!("Your move {} -> {:?} M {:?}", num, tile_move, move_dir);

                if matches!(move_dir, Move::PASS) {
                    println!("{}", format!("INVALID MOVE").red());
                    return self.play_user();
                }

                if matches!(move_dir, Move::BOTH) {
                    println!("BOTH");
                    move_dir = self.get_dir_input().unwrap();
                    println!("getting {:?}", move_dir);
                }

                let tile_move = self.player_deck.remove(num);
                self.move_tile(&move_dir, tile_move);

                return true;
            }
            Err(error) => {
                println!("error: {error}");
                return self.play_user();
            }
        }
    }

    fn get_dir_input(&self) -> Option<Move> {
        println!(
            "Where do you want to play press {} or {}",
            format!("0 to HEAD").green(),
            format!("1 to TAIL").blue()
        );

        let mut input_dir = String::new();
        match io::stdin().read_line(&mut input_dir) {
            Ok(_) => {
                // TODO: check types
                let dir = input_dir.trim().parse::<u32>();
                if !dir.is_ok() {
                    println!("Invalid input {} or {} allowed\n", 0, 1);
                    return self.get_dir_input();
                }
                let dir = dir.unwrap();
                if dir != 0 && dir != 1 {
                    println!("Invalid input {} or {} allowed\n", 0, 1);
                    return self.get_dir_input();
                }
                if dir == 0 {
                    return Some(Move::HEAD);
                }
                return Some(Move::TAIL);
            }
            Err(error) => {
                println!("error: {error}");
                return self.get_dir_input();
            }
        }
    }

    fn play_machine(&mut self, turn: u8) {
        let machine_deck = match turn {
            2 => Some(&self.machine1_deck),
            3 => Some(&self.machine2_deck),
            4 => Some(&self.machine3_deck),
            _ => None,
        };

        if !machine_deck.is_some() {
            println!("{}", format!("Invalid Turn").red());
            return;
        }

        let machine_deck = machine_deck.unwrap();
        if !self.check_deck_for_available_move(&machine_deck) {
            println!("{}", format!("Machine {} pass", turn).red());
            return;
        }
        println!("MACHINE PLAYS {} ", turn);
        // Add timeout of machine Play
        let random_secs = time::Duration::from_secs(thread_rng().gen_range(2..5));
        thread::sleep(random_secs);

        // 1. find available tiles to pay
        let available_moves = self.get_deck_for_available_move(&machine_deck);
        let mut available_idx: usize = 0;

        println!("All Available moves {}", available_moves.len());

        // find the higher Tile (sum of his dots)
        if available_moves.len() > 1 {
            let mut higher_value = 0;
            for (idx, x) in available_moves.iter().enumerate() {
                let deck_index = x.0;
                let tile_on_deck = machine_deck.get(deck_index).unwrap();

                let sum_dots_on_tile = tile_on_deck.0 + tile_on_deck.1;
                println!("Evaluation, {}, {}, {:?}", higher_value, sum_dots_on_tile, tile_on_deck);
                if higher_value < sum_dots_on_tile {
                    higher_value = sum_dots_on_tile;
                    available_idx = idx;
                }
            }
        }

        println!("Available index {}", available_idx);

        let candidate = available_moves.get(available_idx).unwrap();
        let deck_index = candidate.0;
        let mut move_dir = candidate.1;
        let machine_deck_mut = match turn {
            2 => Some(&mut self.machine1_deck),
            3 => Some(&mut self.machine2_deck),
            4 => Some(&mut self.machine3_deck),
            _ => None,
        };
        let tile_move = machine_deck_mut.unwrap().remove(deck_index);

        // when both play randomly select HEAD or TAIL
        if matches!(move_dir, Move::BOTH) {
            move_dir = [Move::HEAD, Move::TAIL][thread_rng().gen_range(0..2)]
        }

        self.move_tile(&move_dir, tile_move)
    }

    fn get_deck_for_available_move(&self, t_tiles: &Vec<Tile>) -> Vec<(usize, Move)> {
        let mut available_moves: Vec<(usize, Move)> = vec![];
        for (idx, t) in t_tiles.iter().enumerate() {
            let res_move = self.check_tile_move(t);
            if !matches!(res_move, Move::PASS) {
                available_moves.push((idx, res_move));
            }
        }
        available_moves
    }

    fn move_tile(&mut self, move_dir: &Move, tile_move: Tile) {
        if matches!(move_dir, Move::INIT) || matches!(move_dir, Move::TAIL) {
            self.board.push(tile_move);
        } else if matches!(move_dir, Move::HEAD) {
            self.board.insert(0, tile_move);
        } else {
            panic!("Invalid Move when moving tile")
        }
        self.arrange_board_tiles(&move_dir);
    }

    /*
     * Arrange first and last tiles with the correct direction
     * depending on the head and tail values
     */
    fn arrange_board_tiles(&mut self, move_dir: &Move) {
        if matches!(move_dir, Move::HEAD) {
            let first_tile = self.board.first().unwrap();
            let second_tile = self.board.get(1).unwrap();
            // swap values of the first tile to match with the second_tile
            if first_tile.1 != second_tile.0 {
                let tmp = self.board[0].0;
                self.board[0].0 = self.board[0].1;
                self.board[0].1 = tmp;
            }
        }
        if matches!(move_dir, Move::TAIL) {
            let last_index = self.board.len() - 1;
            let second_last_index = self.board.len() - 2;
            println!("last index {} - {}", last_index, second_last_index);

            let last_tile = self.board.last().unwrap();
            let second_last_tile = self.board.get(second_last_index).unwrap();

            if last_tile.0 != second_last_tile.1 {
                let tmp = self.board[last_index].0;
                self.board[last_index].0 = self.board[last_index].1;
                self.board[last_index].1 = tmp;
            }
        }
    }

    fn check_deck_for_available_move(&self, t_tiles: &Vec<Tile>) -> bool {
        for t in t_tiles {
            let res = self.check_tile_move(t);
            if !matches!(res, Move::PASS) {
                return true;
            }
        }
        false
    }

    fn check_tile_move(&self, tile: &Tile) -> Move {
        if self.board.is_empty() {
            return Move::INIT;
        }

        let head_value = self.board.first().unwrap().0;
        let tail_value = self.board.last().unwrap().1;

        let valid_head = tile.1 == head_value || tile.0 == head_value;
        let valid_tail = tile.1 == tail_value || tile.0 == tail_value;

        if valid_head && valid_tail {
            Move::BOTH
        } else if valid_head {
            Move::HEAD
        } else if valid_tail {
            Move::TAIL
        } else {
            Move::PASS
        }
    }

    /* PRINT */
    pub fn print(&self) {
        self.print_players_deck();
        self.print_board_deck();
    }

    fn print_players_deck(&self) {
        println!("{:_^32}", "Player Deck".bold().red());
        self.print_tiles(&self.player_deck, Some(true));
        println!("{:_^32}", "Machine 1 Deck");
        self.print_tiles(&self.machine1_deck, None);
        println!("{:_^32}", "Machine 2 Deck");
        self.print_tiles(&self.machine2_deck, None);
        println!("{:_^32}", "Machine 3 Deck");
        self.print_tiles(&self.machine3_deck, None);
    }

    fn print_board_deck(&self) {
        println!("Board");
        self.print_tiles(&self.board, None);
    }

    fn print_tiles(&self, tiles: &Vec<Tile>, with_number: Option<bool>) {
        println!("Length {}", tiles.len());
        for (ind, t) in tiles.iter().enumerate() {
            if with_number.unwrap_or(false) == true {
                print!("{}-{} ", format!("#{}", ind).bold().on_green().white(), t);
            } else {
                print!("{} ", t);
            }
        }
        println!("\n--------------------------");
        println!();
    }
    /* END PRINT */

    pub fn check_finished(&self) -> bool {
        // check  users with empty deck
        // check closed game - All pass
        false
    }

    pub fn get_winner(&self) {
        // find by users with empty hands
        // find user with less dots on his deck
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
