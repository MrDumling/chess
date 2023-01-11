use std::io;
mod piece;
mod board;

use crate::{piece::*, board::*};

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

        if provided_request.starts_with("1")
            || provided_request.to_lowercase().starts_with("normal")
        {
            return Board::default_position();
        } else if provided_request.starts_with("2")
            || provided_request.to_lowercase().starts_with("position")
        {
            return Board::position_from_fen(todo!());
        } else {
            println!("Invalid request, please choose from valid requests:");
        }
    }
}

impl Board {
    fn default_position() -> Self {
        Board::position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    fn position_from_fen(input_position: &str) -> Self {
        fn get_piece_from_char(c: char) -> Piece {
            Piece {
                side: if c.is_uppercase() {
                    Side::WHITE
                } else {
                    Side::BLACK
                },
                piece_type: match c.to_ascii_lowercase() {
                    'r' => PieceType::Rook,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'k' => PieceType::King,
                    'q' => PieceType::Queen,
                    'p' => PieceType::Pawn,
                    x => {
                        panic!("Unknown letter '{x}'")
                    }
                },
            }
        }

        let [piece_positions, active_side, castling_capabilities, en_passant_target, _, _]: [&str; 6] = input_position
            .split_whitespace()
            .collect::<Vec<&str>>()
            .try_into()
            .expect("Invalid FEN string {input_position}");

        let active_side = match active_side {
            "w" => Side::WHITE,
            "b" => Side::BLACK,
            _ => {
                panic!("Invalid side in FEN string {active_side}")
            }
        };

        const EMPTY: Option<Piece> = None;
        let mut held_pieces: [Option<Piece>; 64] = [EMPTY; 64];

        let mut rank = 7;
        let mut file = 0;
        for current_letter in piece_positions.chars() {
            if current_letter == '/' {
                rank -= 1;
                file = 0;
            } else if current_letter.is_numeric() {
                file += current_letter as usize - '0' as usize;
            } else {
                held_pieces[(rank) * 8 + file] = Some(get_piece_from_char(current_letter));
                file += 1;
            }
        }

        Board {
            held_pieces,
            active_side,
            caslting_capabilities: castling_capabilities.into(),
        }
    }

    fn display_board(&self) {
        for (index, piece) in self.held_pieces.iter().enumerate() {
            if index % 8 == 0 {
                println!();
            }

            print!("{}", match piece {
                Some(x) => {
                    x.get_display()
                },
                None => {
                    if ((index / 8) + (index % 8)) % 2 == 0 {
                        ' '
                    } else {
                        '█'
                    }
                }
            });
        }
    }
}

pub enum Side {
    WHITE,
    BLACK,
}