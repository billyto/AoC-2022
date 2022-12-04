use std::fs;
extern crate array_tool;

use array_tool::vec::Intersect;
//use crate::aoc::{day1,day2};

pub mod aoc;

fn main() {

    //day1();
    //day2();
    
    let contents = fs::read_to_string("input/03-input").expect("problem with the file");

    let contents_p2: String = contents.clone();

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

    // let mut runsacks_2 = rucksacks.clone();

    let mut ps: Vec<i32> = Vec::new();
    for r in rucksacks {
        let t: Vec<i32> = r.collect();
        let bags_holder = t.len()/2;
        let mut bags_chunks = t.chunks(bags_holder);
    
        let bag1 = bags_chunks.next().unwrap().to_vec();
        let bag2 = bags_chunks.next().unwrap().to_vec();
        let p = bag1.intersect(bag2).pop().unwrap();

        // println!("repeated is {} ",p);
        ps.push(p);
        
    }
    let suma: i32 = ps.iter().sum();

     println!("sum of priorities is {}", suma);


    // part 2
    //runsacks_2
    
    let rucksacks_2 = contents_p2.split("\n").map( |rs| 
        rs.chars().map(
            |c| 
            // c as u32 
                if c.is_ascii_lowercase() {
                    (c as i32).checked_add(-96).unwrap() as i32         
                } else {
                    (c as i32).checked_add(-38).unwrap() as i32
                }
            
            ).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    
    print!("r2 is {}",rucksacks_2.len());

    let size_t = 3;
    let trios = rucksacks_2.chunks(size_t);
    print!("how many trios? {}",trios.len()); 

    let mut badges: Vec<i32> = Vec::new();

    for trio in trios {

        let t1 = &trio[0];
        let t2: &Vec<i32> = &trio[1];
        let t3: &Vec<i32> = &trio[2];

        let badge = t1.intersect(t2.to_vec()).intersect(t3.to_vec()).pop().unwrap();

        badges.push(badge); 
        println!("common badge is {}", badge);

    }
    let suma_badges: i32 = badges.iter().sum();

    println!("sum of badges is {}", suma_badges); 
        

        // let t: Vec<i32> = r.collect();
        // let bags_holder = t.len()/2;
        // let mut bags_chunks = t.chunks(bags_holder);
    
        // let bag1 = bags_chunks.next().unwrap().to_vec();
        // let bag2 = bags_chunks.next().unwrap().to_vec();
        // let p = bag1.intersect(bag2).pop().unwrap();

        // // println!("repeated is {} ",p);
        // ps.push(p);
        
    }
    //  let suma: i32 = ps.iter().sum();

    // for chunko in chunks {
    //     println!("chunko size {}",chunko.len())
    // }

        




