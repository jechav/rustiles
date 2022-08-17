pub enum GameStatus {
    INIT,
    ONPROGRESS,
    OVER,
}
pub struct Game {
    status: GameStatus,
    username: String,
}

impl Game {
    pub fn new(status: GameStatus) -> Game {
        Game {
            status,
            username: "".to_string(),
        }
    }


    fn ask_user_name() {
    }
    pub fn start() {
    }

    pub fn deal_tiles() {
    }

    pub fn print() {
        Self::print_players_deck();
        Self::print_board_deck();
    }

    fn print_players_deck() {
    }

    fn print_board_deck() {
    }

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

