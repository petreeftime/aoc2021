use aoc2021::util::parse_file;
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(Debug)]
enum MoveParseError {
    Int(std::num::ParseIntError),
    DirectionError(String),
    FormatError,
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.split_whitespace();
        let direction = l.next().ok_or(MoveParseError::FormatError)?;
        let value = l.next().ok_or(MoveParseError::FormatError)?;
        if l.next().is_some() {
            return Err(MoveParseError::FormatError);
        }
        let value = value.parse().map_err(MoveParseError::Int)?;
        match direction {
            "forward" => Ok(Move::Forward(value)),
            "down" => Ok(Move::Down(value)),
            "up" => Ok(Move::Up(value)),
            d => Err(MoveParseError::DirectionError(d.to_string())),
        }
    }
}

fn day02_part01(moves: &[Move]) -> i64 {
    let (h, v) = moves
        .into_iter()
        .fold((0, 0), |pos, movement| match movement {
            Move::Forward(v) => (pos.0 + *v, pos.1),
            Move::Up(v) => (pos.0, pos.1 - *v),
            Move::Down(v) => (pos.0, pos.1 + *v),
        });
    h * v
}

fn day02_part02(moves: &[Move]) -> i64 {
    let (h, v, _) = moves
        .into_iter()
        .fold((0, 0, 0), |pos, movement| match movement {
            Move::Forward(v) => (pos.0 + *v, pos.1 + v * pos.2, pos.2),
            Move::Up(v) => (pos.0, pos.1, pos.2 - *v),
            Move::Down(v) => (pos.0, pos.1, pos.2 + *v),
        });
    h * v
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/day02".to_string());
    let moves: Vec<Move> = parse_file(path).unwrap();
    let p1 = day02_part01(&moves);
    println!("{}", p1);
    let p2 = day02_part02(&moves);
    println!("{}", p2);
}
