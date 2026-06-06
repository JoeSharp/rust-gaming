use super::{GoBoard, GoCell, GoPlayer, LastMove};
use std::fmt;

impl fmt::Display for GoPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GoPlayer::White => GoBoard::WHITE,
                GoPlayer::Black => GoBoard::BLACK,
            }
        )
    }
}

impl fmt::Display for LastMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LastMove::Ok => write!(f, "ok"),
            LastMove::FirstMove => write!(f, "first_move"),
            LastMove::IllegalKo => write!(f, "illegal_ko"),
            LastMove::IllegalSuicidal => write!(f, "illegal_suicidal"),
        }
    }
}

impl fmt::Display for GoCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GoCell::White => GoBoard::WHITE,
                GoCell::WhitePending => GoBoard::WHITE_PENDING,
                GoCell::Black => GoBoard::BLACK,
                GoCell::BlackPending => GoBoard::BLACK_PENDING,
                GoCell::Empty => GoBoard::EMPTY,
            }
        )
    }
}

impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "turn={}\nlast_move={}\ncapturesW={}\ncapturesB={}\n{}",
            self.whos_turn,
            self.last_move,
            self.captures.get(&GoPlayer::White).unwrap(),
            self.captures.get(&GoPlayer::Black).unwrap(),
            self.board
        )
    }
}
