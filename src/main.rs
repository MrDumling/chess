use std::{collections::HashMap, io};

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

        if provided_request.starts_with("1") || provided_request.to_lowercase().starts_with("normal") {
            return Board::default_position();
        } else if provided_request.starts_with("2") || provided_request.to_lowercase().starts_with("position") {
            return Board::position_from_fen("r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1");
            // return Board::position_from_FEN(todo!());
        } else {
            println!("Invalid request, please choose from valid requests:");
        }
    }
}

pub struct Board {
    held_pieces: HashMap<BoardPosition, Piece>,
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
                    x => {panic!("Unknown letter '{x}'")},
                },
            }
        }

        let mut held_pieces = HashMap::<BoardPosition, Piece>::new();

        let mut rank = 8;
        let mut file = 0;
        for current_letter in input_position.chars().collect::<Vec<char>>() {
            if current_letter == '/' {
                rank -= 1;
                file = 0;
            } else if current_letter.is_numeric() {
                file += current_letter as u8 - '0' as u8;
            } else {
                file += 1;
                let position = BoardPosition { rank, file };
                held_pieces.insert(position, get_piece_from_char(current_letter));
            }

            if rank == 1 && file == 8 {
                break;
            }
        }

        Board {
            held_pieces
        }
    }

    fn display_board(&self) {
        for rank in (1..=8).rev() {
            for file in 1..=8 {
                print!(
                    "{}",
                    match self.held_pieces.get(&BoardPosition { rank, file }) {
                        Some(x) => {
                            x.get_display()
                        }
                        None => {
                            if (rank + file) % 2 == 0 {
                                ' '
                            } else {
                                '█'
                            }
                        }
                    }
                )
            }

            println!();
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct BoardPosition {
    rank: u8,
    file: u8,
}

enum Side {
    WHITE,
    BLACK,
}

enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

struct Piece {
    piece_type: PieceType,
    side: Side,
}

impl Piece {
    fn get_display(&self) -> char {
        match self.piece_type {
            PieceType::Rook => match self.side {
                Side::WHITE => '♖',
                Side::BLACK => '♜',
            },
            PieceType::Knight => match self.side {
                Side::WHITE => '♘',
                Side::BLACK => '♞',
            },
            PieceType::Bishop => match self.side {
                Side::WHITE => '♗',
                Side::BLACK => '♝',
            },
            PieceType::Queen => match self.side {
                Side::WHITE => '♕',
                Side::BLACK => '♛',
            },
            PieceType::King => match self.side {
                Side::WHITE => '♔',
                Side::BLACK => '♚',
            },
            PieceType::Pawn => match self.side {
                Side::WHITE => '♙',
                Side::BLACK => '♟',
            },
        }
    }
}
