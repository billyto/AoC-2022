use std::hash::Hash;
use std::{fs, ops::Index};
use std::str::FromStr;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl FromStr for Direction{

    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U"  => Ok(Direction::Up),
            "D"  => Ok(Direction::Down),
            "R"  => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _      => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position{x: usize, y:usize}

struct Move {
    dir: Direction, 
    dist: usize}

impl FromStr for Move{
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {

        let mut pair = input.split(" ");
        let dir_st = pair.next().unwrap();
        let dir = Direction::from_str(dir_st).unwrap();
        let dist_st = pair.next().unwrap();
        let dist:usize = dist_st.parse().unwrap();
        Ok(Move { dir: dir, dist: dist })

    }
}

fn main() {

    let contents = fs::read_to_string("input/08-input").expect("problem with the file");    

    let moves = contents.lines()
                .map(|line| Move::from_str(line).unwrap() );

    let mut visited:HashSet<Position> = HashSet::new();

    visited.insert(Position{x:0,y:0});
    





}

