use std::fs;
extern crate array_tool;

use array_tool::vec::Intersect;
//use crate::aoc::{day1,day2};

pub mod aoc;

fn main() {

    //day1();
    //day2();
    
    let contents = fs::read_to_string("input/03-input").expect("problem with the file");

    //A-> 65 Z->90  27-52
    //a->97 z->122  1-26

    let rucksacks = contents.split("\n").map( |rs| 
                rs.chars().map(
                    |c| 
                    // c as u32 
                        if c.is_ascii_lowercase() {
                            (c as i32).checked_add(-96).unwrap() as i32         
                        } else {
                            (c as i32).checked_add(-38).unwrap() as i32
                        }
                    
                    ));

    // println!("some {}", rucksacks.collect().len());
    let mut ps: Vec<i32> = Vec::new();
    for r in rucksacks {
        let t: Vec<i32> = r.collect();
        let bags_holder = t.len()/2;
        let mut bags_chunks = t.chunks(bags_holder);
    
        let bag1 = bags_chunks.next().unwrap().to_vec();
        let bag2 = bags_chunks.next().unwrap().to_vec();
        let p = bag1.intersect(bag2).pop().unwrap();

      
        println!("repeated is {} ",p);
        ps.push(p);
        
    }
     let suma: i32 = ps.iter().sum();

     println!("sum of priorities is {}", suma)


        
        

    /*
    let a = "29";
    for c in a.chars() {
    println!("{}", c as u32);
}
    */

    // let priorities = rucksacks.map(|s|
    //     s.chars().map( |c| c as u32).collect()
    // );

}

