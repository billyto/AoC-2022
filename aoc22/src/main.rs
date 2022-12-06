use std::fs;


// pub mod aoc;

fn main() {

    let contents = fs::read_to_string("input/04-input").expect("problem with the file");
    let sections: Vec<&str> = contents.split("\n").collect();
}


