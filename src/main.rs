use std::io::{self};
mod board;
mod piece;

use crate::board::*;

fn main() {
    let board = select_mode();
    board.display_board();
}

fn select_mode() -> Board {
    println!("Select a mode:");
    loop {
        println!("1: Normal Board");
        println!("2: Board Position");

        let mut provided_request = String::new();

        io::stdin()
            .read_line(&mut provided_request)
            .expect("Failed to read request");

        if provided_request.starts_with('1')
            || provided_request.to_lowercase().starts_with("normal")
        {
            return Board::default_position();
        } else if provided_request.starts_with('2')
            || provided_request.to_lowercase().starts_with("position")
        {
            println!("Please provide a FEN string");
            let mut provided_request = String::new();
            io::stdin().read_line(&mut provided_request).expect("Failed to read FEN string");
            return Board::position_from_fen(&provided_request[..]);
        } else {
            println!("Invalid request, please choose from valid requests:");
        }
    }
}

pub enum Side {
    WHITE,
    BLACK,
}
