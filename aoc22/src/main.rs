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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position{x: isize, y:isize}

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

    let contents = fs::read_to_string("input/09-sample").expect("problem with the file");    

    let moves = contents.lines()
                .map(|line| Move::from_str(line).unwrap() );

    let mut visited:HashSet<Position> = HashSet::new();

    let mut tail_p = Position{x:0,y:0};
    let mut head_p= Position{x:0, y:0}; 

    visited.insert(tail_p.clone());

    for m in moves {


        match m.dir {
            
            Direction::Up => {
                                println!("Going Up {}",m.dist);
                                // let init_y = tail_p.y;
                                // head_p.y = head_p.y + m.dist;
                                // tail_p.y = head_p.y - 1; // I'm right below

                                // for n in init_y as isize.. head_p.y as isize {
                                //     visited.insert(Position{x:tail_p.x, y: n as usize});
                                // }

                                let init_h_y = head_p.y;
                                head_p.y = head_p.y + m.dist;
                                tail_p.y = head_p.y - 1; // I'm right below
                                for n in (init_h_y + 1) .. tail_p.y { // from the first place where head will move to the tail.
                                    visited.insert(Position{x:tail_p.x,y:n as usize});
                                }

                            },
            Direction::Down => {
                                println!("Going Down {}",m.dist);
                                // let init_y = tail_p.y;
                                // head_p.y = head_p.y - m.dist;
                                // tail_p.y = head_p.y + 1; //I'm right above
                                
                                // for n in head_p.y as isize..tail_p.y as isize {
                                //     visited.insert(Position{x:tail_p.x, y: n as usize});
                                // }
                                let init_h_y = head_p.y; 
                                head_p.y = head_p.y - m.dist;
                                tail_p.y = head_p.y + 1; //I'm right above
                                for n in (init_h_y -1) .. tail_p.y {
                                    visited.insert(Position{x:tail_p.x,y:n as usize});
                                }


                            },
            Direction::Left => {
                                println!("Going Left {}",m.dist);
                                // head_p.x = head_p.x - m.dist;
                                // tail_p.x = head_p.x + 1;
                                
                                // for n in head_p.x as isize..tail_p.x as isize {
                                //     visited.insert(Position{x:n as usize, y: tail_p.y as usize});
                                // }
                                let init_h_x = head_p.x;
                                head_p.x = head_p.x - m.dist;
                                tail_p.x = head_p.x + 1;
                                for n in (init_h_x-1) .. tail_p.x {
                                    visited.insert(Position{x:n as usize,y:tail_p.y as usize});
                                }

                            },
            Direction::Right => {
                                println!("Going Right {}",m.dist);
                                // let init_x = tail_p.x;
                                // head_p.x = head_p.x + m.dist;
                                // tail_p.x = head_p.x - 1; 

                                // for n in  tail_p.x as isize.. head_p.x as isize{
                                //     visited.insert(Position{x:n as usize,y:tail_p.y});
                                // }
                                let init_h_x = head_p.x;
                                head_p.x = head_p.x - m.dist;
                                tail_p.x = head_p.x + 1;                             
                                for n in (init_h_x+1) .. tail_p.x{
                                    visited.insert(Position{x:n,y:tail_p.y});
                                }
                            },
        }
    }

    println!("Positions covered by tail {}", visited.len());


}

