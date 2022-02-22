use colored::Colorize;
use crossterm::{ event::{ read, Event, KeyCode } };

#[derive(Clone, Copy)]
struct Dimension {
    width: usize,
    height: usize,
}

struct Game {
    size: Dimension,
    field: Vec<Vec<usize>>,
    players: usize,
    current_player: usize,
    selector: usize,
    colors: Vec<String>,
}

impl Game {
    fn introduction(&self) {
        println!("hi, here you can gaem :sunglussesswk:");
    }
    fn reset_field(&mut self) {
        self.field = vec![ vec![ 0; self.size.height ]; self.size.width ];
    }
    fn current_player(&self) {
        println!("Current player: {}", self.colors[self.current_player]);
    }
    fn display_winner(&self, winner: usize) {
        if winner == 0 {
            println!("This is a draw!");
        } else {
            println!("Winner player: {}", self.colors[winner]);
        }
    }
    fn check_field_full(&self) -> bool {
        for i in 0..self.size.width {
            if self.field[i][self.size.height-1] == 0 { return false }
        }
        return true;
    }
    fn player_move(&mut self) {
        loop {
            let mut move_done = false;

            if let Event::Key(event) = read().unwrap() {
                let previous_selector = self.selector;
                match event.code {
                    KeyCode::Left => {
                        if self.selector > 0 { self.selector -= 1 }
                    },
                    KeyCode::Right => {
                        if self.selector < self.size.width - 1 { self.selector += 1 }
                    },
                    KeyCode::Enter | KeyCode::Up | KeyCode::Down => {
                        for y in 0..self.size.height {
                            if self.field[self.selector][y] == 0 {
                                self.field[self.selector][y] = self.current_player;
                                move_done = true;
                                break;
                            }
                        }
                    },
                    _ => (),
                }

                if previous_selector != self.selector || move_done {
                    self.clear_terminal();
                    self.draw_field();
                    self.draw_selector();
                    self.current_player();
                    if move_done { break }
                }
            }
        }
        self.next_player();
    }
    fn next_player(&mut self) {
        self.current_player = self.current_player % self.players + 1
    }
    fn who_won(&self) -> usize {
        // inspired by:
        // https://stackoverflow.com/questions/32770321/connect-4-check-for-a-win-algorithm

        for x in 0..self.size.width {
            for y in 0..self.size.height {
                let current = self.field[x][y];

                if current == 0 { continue }

                // horizontalCheck
                if x < self.size.width - 3 {
                    for i in 0..=4 {
                        if i == 4 { return current }
                        if current != self.field[x+i][y] { break }
                    }
                }

                //verticalCheck
                if y < self.size.height - 3 {
                    for i in 0..=4 {
                        if i == 4 { return current }
                        if current != self.field[x][y+i] { break }
                    }
                }

                // ascendingDiagonalCheck
                if x < self.size.width - 3 && y < self.size.height - 3 {
                    for i in 0..=4 {
                        if i == 4 { return current }
                        if current != self.field[x+i][y+i] { break }
                    }
                }

                // descendingDiagonalCheck
                if x >= 4 && y >= 4 {
                    for i in 0..=4 {
                        if i == 4 { return current }
                        if current != self.field[x-i][y-i] { break }
                    }
                }
            }
        }

        return 0;
    }
    fn draw_field(&self) {
        for y in (0..self.size.height).rev() {
            for x in 0..self.size.width {
                print!("{}", self.colors[self.field[x][y]]);
            }
            println!("{}", (160 as char).to_string().bright_black().to_string());
        }
    }
    fn draw_selector(&self) {
        for i in 0..self.selector {
            print!("{}", (i+1).to_string().bright_black());
        }
        print!("{}", "^".on_bright_white().black().to_string());
        for i in self.selector+1..self.size.width {
            print!("{}", (i+1).to_string().bright_black());
        }
        println!();
    }
    fn clear_terminal(&self) {
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn main() {
    let field_size = Dimension {
        width: 7,
        height: 6,
    };

    let mut game = Game {
        size: field_size,
        players: 2,
        current_player: 1,
        field: vec![vec![]],
        selector: 0,
        colors: vec![
            (160 as char).to_string().on_bright_black().to_string(),
            "X".on_bright_yellow().black().to_string(),
            "O".on_bright_red().black().to_string(),
            "I".on_bright_green().black().to_string(),
            "V".on_blue().black().to_string(),
            "L".on_bright_cyan().black().to_string(),
            "H".on_magenta().black().to_string(),
        ]
    };

    game.reset_field();

    game.clear_terminal();
    game.introduction();
    game.draw_field();
    game.draw_selector();
    game.current_player();
    game.player_move();

    let winner_player = loop {
        let winner = game.who_won();
        if winner != 0 { break winner; }
        
        game.clear_terminal();
        game.draw_field();
        game.draw_selector();

        if game.check_field_full() {
            break 0;
        } else {
            game.current_player();
            game.player_move();
        }
    };

    game.clear_terminal();
    game.draw_field();
    game.display_winner(winner_player);
}
