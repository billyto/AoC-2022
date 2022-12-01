use std::fs;
use regex::Regex;

fn main() {

    let contents = fs::read_to_string("input/01-p1").expect("problem with the file");

    let re = Regex::new(r"\n{2,}").unwrap();
    let loads= re.split(&contents).collect::<Vec<&str>>();
   
    
    // Part 1: max calories holder

    let max_load = loads.iter().map( |l| 
        l.split("\n").map(|s| 
                           s.parse::<i32>().unwrap()
        ).sum::<i32>()).max().unwrap();

        println!("Max Calories carried {}", max_load);


    // Part 2: three max calories holders (Sum of)

    let mut loads = loads.iter().map( |l| 
        l.split("\n").map(|s| 
                           s.parse::<i32>().unwrap()
        ).sum::<i32>()).collect::<Vec<i32>>();

    loads.sort();
    loads.reverse();
    let top_three  = loads[0..3].iter().sum::<i32>();

    println!("Calories carried by top 3 elfs {}", top_three);

}
