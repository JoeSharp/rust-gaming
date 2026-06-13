use super::{GoBoard, GoBoardError, GoCell, GoPlayer, LastMove};
use arr2d::Arr2d;
use arr2d::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};

impl GoPlayer {
    pub fn other(&self) -> GoPlayer {
        match *self {
            GoPlayer::White => GoPlayer::Black,
            GoPlayer::Black => GoPlayer::White,
        }
    }

    fn into_pending(self) -> GoCell {
        match self {
            GoPlayer::White => GoCell::WhitePending,
            GoPlayer::Black => GoCell::BlackPending,
        }
    }
}

impl Into<GoCell> for GoPlayer {
    fn into(self) -> GoCell {
        match self {
            GoPlayer::White => GoCell::White,
            GoPlayer::Black => GoCell::Black,
        }
    }
}
impl TryInto<GoPlayer> for GoCell {
    type Error = GoBoardError;

    fn try_into(self) -> Result<GoPlayer, GoBoardError> {
        match self {
            GoCell::White => Ok(GoPlayer::White),
            GoCell::WhitePending => Ok(GoPlayer::White),
            GoCell::Black => Ok(GoPlayer::Black),
            GoCell::BlackPending => Ok(GoPlayer::Black),
            _ => Err(GoBoardError::InvalidPlayer),
        }
    }
}

impl GoBoard {
    pub const WHITE: char = 'W';
    pub const WHITE_PENDING: char = 'w';
    pub const BLACK: char = 'B';
    pub const BLACK_PENDING: char = 'b';
    pub const EMPTY: char = '-';

    pub fn with_size(rows: usize, columns: usize) -> GoBoard {
        GoBoard {
            whos_turn: GoPlayer::Black,
            last_move: LastMove::FirstMove,
            captures: HashMap::from([(GoPlayer::Black, 0), (GoPlayer::White, 0)]),
            last_captures: VecDeque::new(),
            board: Arr2d::with_size(rows, columns, GoCell::Empty),
        }
    }

    fn locate_pending(&self) -> Option<&Cell<GoCell>> {
        self.board
            .all_cells()
            .filter(|c| match c.value() {
                GoCell::WhitePending | GoCell::BlackPending => true,
                _ => false,
            })
            .next()
    }

    fn calculate_captures(
        &self,
        from: &Cell<GoCell>,
        opponent: GoPlayer,
    ) -> impl Iterator<Item = &Cell<GoCell>> {
        let opponent_cell: GoCell = opponent.into();
        let mut seen_ids = HashSet::new();
        self.board
            .get_neighbours(from.row(), from.column())
            .filter(move |neighbour| neighbour.value() == opponent_cell)
            .filter(|neighbour| {
                !self
                    .has_liberties(neighbour.row(), neighbour.column())
                    .unwrap()
            })
            .flat_map(|captured_neighbour| {
                self.board
                    .flood_fill(captured_neighbour.row(), captured_neighbour.column())
                    .unwrap()
            })
            .filter(move |c| seen_ids.insert(c.id()))
    }

    pub fn make_move(&mut self, row: usize, column: usize) -> Result<(), GoBoardError> {
        self.board.set(row, column, self.whos_turn.into_pending());
        self.iterate()
    }

    fn check_suicidal(
        &self,
        row: usize,
        column: usize,
        captures: &Vec<(usize, usize)>,
    ) -> Result<(), GoBoardError> {
        // Zero captures, and zero empty neighbours, indicates suicidal move
        match captures.len() {
            0 => match self
                .board
                .get_neighbours(row, column)
                .filter(|c| c.value() == GoCell::Empty)
                .count()
            {
                0 => Err(GoBoardError::IllegalMove),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    fn check_ko(&mut self, captures: &Vec<(usize, usize)>) -> Result<(), GoBoardError> {
        if captures.len() == 0 {
            return Ok(());
        }

        let this_str = captures
            .iter()
            .map(|(a, b)| format!("{},{}", a, b))
            .collect::<Vec<String>>()
            .join("-");

        if self.last_captures.contains(&this_str) {
            return Err(GoBoardError::IllegalMove);
        }
        self.last_captures.push_back(this_str);

        while self.last_captures.len() > 2 {
            self.last_captures.pop_front();
        }

        Ok(())
    }

    pub fn iterate(&mut self) -> Result<(), GoBoardError> {
        let cell = match self.locate_pending() {
            Some(c) => c,
            None => return Err(GoBoardError::NoPendingFound),
        };
        let who: GoPlayer = match cell.value().try_into() {
            Ok(w) => w,
            Err(e) => return Err(e),
        };

        if who != self.whos_turn {
            return Err(GoBoardError::WrongPlayerTurn);
        }

        let row = cell.row();
        let column = cell.column();

        let opponent = who.other();
        let captures = self
            .calculate_captures(cell, opponent)
            .map(|c| (c.row(), c.column()))
            .collect::<Vec<_>>();

        if let Err(e) = self.check_ko(&captures) {
            self.last_move = LastMove::IllegalKo;
            self.board.set(row, column, GoCell::Empty);
            return Err(e);
        }

        if let Err(e) = self.check_suicidal(row, column, &captures) {
            self.last_move = LastMove::IllegalSuicidal;
            self.board.set(row, column, GoCell::Empty);
            return Err(e);
        }

        for (row, column) in captures {
            self.board.set(row, column, GoCell::Empty);
            self.captures.entry(who).and_modify(|e| *e += 1);
        }

        let played_cell: GoCell = who.into();
        self.board.set(row, column, played_cell);

        self.whos_turn = opponent;

        Ok(())
    }

    fn has_liberties(&self, row: usize, column: usize) -> Result<bool, &str> {
        match self.get_liberties(row, column) {
            Ok(c) => Ok(c.count() > 0),
            Err(e) => Err(e),
        }
    }

    fn get_liberties(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<GoCell>>, &str> {
        match self.board.get_perimeter(row, column) {
            Ok(p) => Ok(p.filter(|c| c.value() == GoCell::Empty)),
            _ => Err("Could not retrieve perimeter of {row}, {column}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_size() {
        // When
        let state = GoBoard::with_size(5, 3);

        use GoCell::*;

        // Then
        assert_eq!(
            state.board,
            Arr2d::from_2d_array(vec![
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
            ])
        );
    }

    #[test]
    fn create_with_size_then_display() {
        // Given
        let state = GoBoard::with_size(5, 3);

        // When, Then
        let _ = state.to_string();
    }

    #[test]
    fn test_has_liberties_true() {
        // Given
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W-W-
WBW--
WBBW-
WBBbW
WWWW-
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        // When
        let result = state.has_liberties(2, 1).unwrap();

        // Then
        assert!(!result);
    }

    #[test]
    fn test_has_liberties_false() {
        // Given
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W-W-
W-W--
WBBW-
WB-bW
WWWW-
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        // When
        let result = state.has_liberties(2, 1).unwrap();

        // Then
        assert!(result);
    }

    #[test]
    fn test_get_liberties() {
        // Given
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W-W-
W-W--
WBBW-
WB-bW
WWWW-
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        // When
        let result: Vec<&Cell<GoCell>> = match state.get_liberties(2, 1) {
            Ok(p) => p.collect(),
            _ => panic!("Could not get liberties"),
        };

        // Then
        for (exp_row, exp_column) in [(1, 1), (3, 2)] {
            let exp_cell: Cell<GoCell> = Cell::from(exp_row, exp_column, GoCell::Empty);

            assert!(
                result.contains(&&exp_cell),
                "Result {result:?} does not contain {exp_cell:?}"
            );
        }
    }
}
