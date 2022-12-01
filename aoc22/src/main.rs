use std::fs;
use regex::Regex;

fn main() {

    let contents = fs::read_to_string("input/01-p1").expect("problem with the file");

    let re = Regex::new(r"\n{2,}").unwrap();
    let loads= re.split(&contents).collect::<Vec<&str>>();
    
    let max_load = loads.iter().map( |l| 
        l.split("\n").map(|s| 
                           s.parse::<i32>().unwrap()
        ).sum::<i32>()).max().unwrap();

        println!("Max Calories carried {}", max_load);

}
