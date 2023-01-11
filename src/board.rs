use crate::{piece::Piece, Side};

pub struct Board {
    pub held_pieces: [Option<Piece>; 64],
    pub active_side: Side,
    pub caslting_capabilities: CastlingCapablities,
}

pub struct CastlingCapablities {
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
