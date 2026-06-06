use super::{GoBoard, GoCell, GoPlayer, LastMove};
use arr2d::Arr2d;
use arr2d::ParseError;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

impl TryFrom<char> for GoPlayer {
    type Error = ParseError;

    fn try_from(c: char) -> Result<GoPlayer, ParseError> {
        match c {
            GoBoard::WHITE => Ok(GoPlayer::White),
            GoBoard::BLACK => Ok(GoPlayer::Black),
            _ => Err(ParseError::InvalidCharacter),
        }
    }
}

impl FromStr for LastMove {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "ok" => Result::Ok(LastMove::Ok),
            "illegal_ko" => Result::Ok(LastMove::IllegalKo),
            "illegal_suicidal" => Result::Ok(LastMove::IllegalSuicidal),
            _ => Err(ParseError::InvalidValue),
        }
    }
}

impl TryFrom<char> for GoCell {
    type Error = ParseError;

    fn try_from(c: char) -> Result<GoCell, ParseError> {
        match c {
            GoBoard::WHITE => Ok(GoCell::White),
            GoBoard::WHITE_PENDING => Ok(GoCell::WhitePending),
            GoBoard::BLACK => Ok(GoCell::Black),
            GoBoard::BLACK_PENDING => Ok(GoCell::BlackPending),
            GoBoard::EMPTY => Ok(GoCell::Empty),
            _ => Err(ParseError::InvalidCharacter),
        }
    }
}

impl GoBoard {
    /**
     * It would probably be nicer if it allowed reading of K/V pairs in whatever order, put them in
     * a map and then we would look for specific keys
     */
    fn read_kv<'a, 'b>(input: &'a str, name: &'b str) -> Result<&'a str, ParseError> {
        let parts: Vec<&str> = input.split("=").collect();
        match parts.len() {
            2 => {
                if parts[0] == name {
                    Ok(parts[1])
                } else {
                    Err(ParseError::InvalidValue)
                }
            }
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn from_str(as_str: &str) -> Result<GoBoard, ParseError> {
        let lines: Vec<&str> = as_str
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();

        if lines.len() <= 4 {
            return Err(ParseError::NotEnoughLines);
        }

        let whos_turn = Self::read_kv(&lines[0], "turn")?;
        let whos_turn: GoPlayer = match whos_turn.chars().nth(0) {
            Some(c) => match GoPlayer::try_from(c) {
                Ok(gp) => gp,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError::NotEnoughChars),
        };

        let last_move = Self::read_kv(lines[1], "last_move")?;
        let last_move: LastMove = match last_move.parse() {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        let white_captures = Self::read_kv(lines[2], "capturesW")?;
        let white_captures: u16 = match white_captures.parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseError::InvalidValue),
        };

        let black_captures = Self::read_kv(lines[3], "capturesB")?;
        let black_captures: u16 = match black_captures.parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseError::InvalidValue),
        };

        let slice = &lines[4..];
        let board: Arr2d<GoCell> = match Arr2d::from_lines(slice.iter().copied()) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        let mut captures: HashMap<_, _> = HashMap::new();
        captures.insert(GoPlayer::White, white_captures);
        captures.insert(GoPlayer::Black, black_captures);

        Ok(GoBoard {
            whos_turn,
            last_move,
            captures,
            last_captures: VecDeque::new(),
            board,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str() {
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W---
--W--
-B---
-B-b-
-----
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

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
                board: Arr2d::from_contents(vec![
                    vec![Empty, White, Empty, Empty, Empty,],
                    vec![Empty, Empty, White, Empty, Empty],
                    vec![Empty, Black, Empty, Empty, Empty],
                    vec![Empty, Black, Empty, BlackPending, Empty],
                    vec![Empty, Empty, Empty, Empty, Empty],
                ])
            }
        );
    }
}
