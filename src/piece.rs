use crate::Side;

pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
}

impl Piece {
    pub fn get_display(&self) -> char {
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