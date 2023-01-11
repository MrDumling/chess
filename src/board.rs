use crate::{piece::{Piece, PieceType}, Side};

pub struct Board {
    held_pieces: [Option<Piece>; 64],
    active_side: Side,
    caslting_capabilities: CastlingCapablities,
    en_passent_target: Option<usize>
}

struct BoardPosition {
    rank: u8,
    file: u8,
}

struct CastlingCapablities {
    white_caslting_ability: CasltingAbilities,
    black_caslting_ability: CasltingAbilities,
}

struct CasltingAbilities {
    king_side: bool,
    queen_side: bool,
}

impl From<&str> for CastlingCapablities {
    fn from(x: &str) -> Self {
        CastlingCapablities {
            white_caslting_ability: CasltingAbilities {
                king_side: x.contains('k'),
                queen_side: x.contains('q'),
            },
            black_caslting_ability: CasltingAbilities {
                king_side: x.contains('K'),
                queen_side: x.contains('Q'),
            },
        }
    }
}

impl Board {
    pub fn default_position() -> Self {
        Board::position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn position_from_fen(input_fen: &str) -> Self {
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

        let [piece_positions, active_side, castling_capabilities, en_passant_target_pos, _, _]: [&str; 6] = input_fen
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
            en_passent_target: if en_passant_target_pos == "-" {None} else {Some(Self::position_to_index(en_passant_target_pos))}
        }
    }

    fn position_to_index(pos: &str) -> usize {
        if pos.len() != 2 {
            panic!("Expected position of length 2, instead got {pos}");
        }

        let mut chars = pos.chars();

        let rank = chars.next().unwrap() as usize - 'a' as usize;
        let file = chars.next().unwrap().to_digit(8).unwrap() as usize;

        rank * 8 + file
    }

    pub fn display_board(&self) {
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