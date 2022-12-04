use std::fs;
use std::str::FromStr;
//use crate::aoc::{day1,day2};

pub mod aoc;

fn main() {

    //day1();
    //day2();

    let contents = fs::read_to_string("input/04-input").expect("problem with the file");

    let sections: Vec<&str> = contents.split("\n").collect();

    let mut overlaps: u32 = 0;
    let mut patial_overlap: u32 = 0; 

    for section in sections {

        println!("working on {section}");
        let assigns: Vec<&str> = section.split(",").collect();

        let mut ps: Vec<std::ops::Range<i32>> = Vec::new();

        for ass in assigns{

            let mut elf_assign: Vec<&str> = ass.split("-").collect();
            let start_s = elf_assign.pop().unwrap();
            let start: i32 =  FromStr::from_str(start_s).unwrap();
            let end_s = elf_assign.pop().unwrap();
            let end = FromStr::from_str(end_s).unwrap();

            let r1 = std::ops::Range { start: end, end: start+1 }; //yolo

            ps.push(r1);
        }
        //we have to ranges
        let r1 = ps.pop().unwrap();
        let r2 = ps.pop().unwrap();

        let r1_end = r1.end - 1;
        let r2_end = r2.end - 1;

        if (r1.contains(&r2.start) && r1.contains(&r2_end) ) || 
            r2.contains(&r1.start) && r2.contains(&r1_end){
                overlaps = overlaps +1;
        } 

        if (r1.contains(&r2.start) || r1.contains(&r2_end) ) || 
        r2.contains(&r1.start) || r2.contains(&r1_end){
            patial_overlap = patial_overlap +1;
    }  

    }
    println!("overlaps {}", overlaps);
    println!("partial overlaps {}", patial_overlap);
        

}


