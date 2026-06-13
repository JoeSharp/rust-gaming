mod core;
mod display;
mod parse;

use arr2d::Arr2d;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Hash, Eq, Debug, PartialEq, Copy, Clone)]
pub enum GoPlayer {
    White,
    Black,
}

#[derive(Hash, Eq, Debug, PartialEq)]
pub enum LastMove {
    FirstMove,
    Ok,
    IllegalKo,
    IllegalSuicidal,
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum GoCell {
    White,
    WhitePending,
    Black,
    BlackPending,
    Empty,
}

#[derive(Debug)]
pub enum GoBoardError {
    IllegalMove,
    InvalidPlayer,
    NoPendingFound,
    WrongPlayerTurn,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GoBoard {
    whos_turn: GoPlayer,
    last_move: LastMove,
    captures: HashMap<GoPlayer, u16>,
    last_captures: VecDeque<String>,
    board: Arr2d<GoCell>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use arr2d::ParseError;
    use std::fs;
    use test_case::test_case;

    #[test_case(GoPlayer::White, GoPlayer::Black)]
    fn test_other_player(player: GoPlayer, expected: GoPlayer) {
        let result = player.other();

        assert_eq!(result, expected)
    }

    fn test_file_raw_contents(basefile: &str) -> String {
        let filename = format!("resources/tests/go/{}", basefile);
        fs::read_to_string(&filename).expect(&format!(
            "Expected to find hardcoded test resource at {}",
            filename
        ))
    }

    fn create_go_from_test_file(basefile: &str) -> Result<GoBoard, ParseError> {
        let file_contents = test_file_raw_contents(basefile);
        GoBoard::from_str(&file_contents)
    }

    fn extract_coord<'a>(mut parts: impl Iterator<Item = &'a str>) -> Result<usize, ParseError> {
        match parts.next() {
            Some(e) => match e.trim().parse() {
                Ok(r) => Ok(r),
                _ => return Err(ParseError::InvalidValue),
            },
            None => return Err(ParseError::InvalidValue),
        }
    }

    fn create_move_from_test_file(basefile: &str) -> Result<(usize, usize), ParseError> {
        let file_contents = test_file_raw_contents(basefile);

        let mut parts = file_contents.split(",");

        let row = match extract_coord(&mut parts) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let column = match extract_coord(&mut parts) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        Ok((row, column))
    }

    #[test]
    fn test_ko_rule() {
        let mut state = create_go_from_test_file("ko/simple_1/1_before.txt").unwrap();
        let state_1_execute = create_go_from_test_file("ko/simple_1/1_execute.txt").unwrap();
        let move_2 = create_move_from_test_file("ko/simple_1/2_move.txt").unwrap();
        let state_2_execute = create_go_from_test_file("ko/simple_1/2_execute.txt").unwrap();
        let move_3 = create_move_from_test_file("ko/simple_1/3_move.txt").unwrap();
        let state_3_execute = create_go_from_test_file("ko/simple_1/3_execute.txt").unwrap();
        let move_4 = create_move_from_test_file("ko/simple_1/4_move.txt").unwrap();
        let state_4_execute = create_go_from_test_file("ko/simple_1/4_execute.txt").unwrap();

        let _ = state.iterate().unwrap();
        assert_board_equal(&state_1_execute, &state);

        let _ = state.make_move(move_2.0, move_2.1).unwrap();
        assert_board_equal(&state_2_execute, &state);

        let _ = state.make_move(move_3.0, move_3.1).unwrap();
        assert_board_equal(&state_3_execute, &state);

        let result4 = state.make_move(move_4.0, move_4.1);

        assert!(result4.is_err(), "Expected move 4 to generate an error");
        assert_board_equal(&state_4_execute, &state);
    }

    #[test]
    fn test_parse() {
        let state = create_go_from_test_file("parse/1.txt").unwrap();

        use GoCell::*;

        assert_eq!(
            state,
            GoBoard {
                whos_turn: GoPlayer::White,
                last_move: LastMove::Ok,
                last_captures: VecDeque::new(),
                captures: [(GoPlayer::White, 16), (GoPlayer::Black, 23)]
                    .iter()
                    .cloned()
                    .collect(),
                board: Arr2d::from_2d_array(vec![
                    vec![Empty, White, Empty, Empty, Empty,],
                    vec![Empty, Empty, White, Empty, Empty],
                    vec![Empty, Black, Empty, Empty, Empty],
                    vec![Empty, Black, Empty, BlackPending, Empty],
                    vec![Empty, Empty, Empty, Empty, Empty],
                ])
            }
        );
    }

    #[test_case("captures/simple_1")]
    #[test_case("captures/simple_2")]
    #[test_case("captures/corner_1")]
    #[test_case("captures/corner_2")]
    #[test_case("suicidal_move/simple_1")]
    #[test_case("suicidal_move/simple_2")]
    fn test_iterate(name: &str) {
        let file_before = format!("{}/1_before.txt", name);
        let file_execute = format!("{}/1_execute.txt", name);
        let mut state_before = create_go_from_test_file(&file_before).unwrap();
        let _ = state_before.iterate();
        let state_execute = create_go_from_test_file(&file_execute).unwrap();
        assert_board_equal(&state_execute, &state_before);
    }

    fn assert_board_equal(expected: &GoBoard, result: &GoBoard) {
        assert_eq!(
            expected.whos_turn, result.whos_turn,
            "Incorrect turn \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.captures, result.captures,
            "Incorrect captures \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.last_move, result.last_move,
            "Incorrect Last Move \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.board, result.board,
            "Board states do not match \n{expected}\n\n{result}"
        );
    }
}
