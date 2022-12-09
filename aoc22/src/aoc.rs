use std::fs;
use regex::Regex;
use std::str::FromStr;
use itertools::{Itertools, Unique};
use indextree::{Arena, NodeEdge, NodeId};
use std::fmt;

extern crate array_tool;

use array_tool::vec::Intersect;


pub fn day1() {
    let contents = fs::read_to_string("input/01-p1").expect("problem with the file");

    let re = Regex::new(r"\n{2,}").unwrap();
    let loads= re.split(&contents).collect::<Vec<&str>>();
   
    
    //Part 1: max calories holder

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

pub fn day2() {
    let contents = fs::read_to_string("input/02-input").expect("problem with the file");

    let games = contents.split("\n").map( |s| 
        s.split_whitespace().collect::<Vec<&str>>()
    );  

    let mut scores: Vec<usize> = Vec::new(); 
    for g in games {

        let score = game_p2(g);
        scores.push(score);
    }

    let total_score: usize = scores.iter().sum();

    println!("Total score is {}",total_score); //8933

}

/*
        A - Rock:       1 - X 
        B - Paper:      2 - Y
        C - Scissors:   3 - Z

        Win + 6 -> Z
        Tie + 3 -> Y
        Lose + 0 -> X

    */
fn game_p2( picks: Vec<&str> ) -> usize { 

        //   A B C
let strategy_matrix: [[usize;3];3] = [  [2,0,1],  // X
            [0,1,2],  // Y
            [1,2,0]]; // Z

let game_matrix: [[usize;3];3] = [  [3,0,6],
        [6,3,0],
        [0,6,3]]; 


let strategy = match picks[1] {
"X" => 0,
"Y" => 1,
_ => 2,
};                                    

let opponent = match picks[0]{
"A" => 0,
"B" => 1,
_ => 2, 
};

let me: usize = strategy_matrix[strategy][opponent];

let points = game_matrix[me][opponent];
let score = points + me + 1; // cause cause matrix offset
score 

}

/*
A - Rock:       1 - X 
B - Paper:      2 - Y
C - Scissors:   3 - Z

Win + 6
Tie + 3
Lose + 0

*/
fn game_p1 ( picks: Vec<&str> ) -> usize {

let game_matrix: [[usize;3];3] = [  [3,0,6],
        [6,3,0],
        [0,6,3]];

let me = match picks[1] {
"X" => 0,
"Y" => 1,
_ => 2,
};

let opponent = match picks[0]{
"A" => 0,
"B" => 1,
_ => 2, 
};

let points = game_matrix[me][opponent];
let score = points + me + 1; // cause cause matrix offset
score

}


pub fn day3() {

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
        


}


pub fn day4(){
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

pub fn day5() {

    let contents = fs::read_to_string("input/05-input").expect("problem with the file");

    let re = Regex::new(r"\n{2,}").unwrap();
    let loads= re.split(&contents).collect::<Vec<&str>>();

    let num_stacks = loads.first().unwrap().trim_end().chars().last().unwrap().to_digit(10).unwrap();

    println!("we have {} stacks", num_stacks);

    //create num_Stacks

    let mut stacks: Vec<Vec<char>> = std::iter::repeat(Vec::new()).take(num_stacks as usize).collect::<Vec<Vec<char>>>();
    let mut initial_load = loads.first().unwrap().split("\n").collect::<Vec<&str>>();
    let instructions = loads.last().unwrap().split("\n").collect::<Vec<&str>>();

    initial_load.pop();
    initial_load.reverse();
    //build the initial stacks
    for line in initial_load {

        let mut i = 0;
        let mut j = 1;

        let load: char = line.chars().nth(j).unwrap_or_default();
        if load.is_ascii_alphabetic(){
            stacks[0].push(load);
        }

        while j < line.len() {
            i = i +1;  // stacks index
            j = j+ 4;  // jumper for charsi

            let load: char = line.chars().nth(j).unwrap_or_default();
            if load.is_ascii_alphabetic(){
                stacks[i].push(load);
            } 
        }
    }

    //move things around
    // move 1 from 2 to 1
    let inst_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap(); 
    for inst in instructions {

        let mut count: usize = 0;
        let mut from: usize = 0;
        let mut to: usize = 0;
        for cap in inst_re.captures_iter(inst){
            count =  FromStr::from_str(&cap[1]).unwrap() ;
            from = FromStr::from_str(&cap[2]).unwrap();
            to= FromStr::from_str(&cap[3]).unwrap();

        }

       
        let cut: usize = stacks[from-1].len() - count;
        let mut movers = stacks[from-1].split_off(cut);
        // movers.reverse();  //flip this one off for part 2
        stacks[to-1].append(&mut movers);

    }

    let message:String = stacks.iter().map(|s| s.last().unwrap()).collect();   
    println!("{}",message);
    

}

pub fn day6(){
    let contents = fs::read_to_string("input/06-input").expect("problem with the file");
    let jump: usize = 14; // flip to 4 for part 1 
    let mut start: usize = 0;
    let mut end: usize = jump;
    while end < contents.len(){
        start = start +1;
        end = start + jump;
        let bloc = contents.get(start..end).unwrap();
        let bloc_uniques = bloc.to_string().chars().unique().collect_vec().len();
        println!("bloc {} uniques {}",bloc, bloc_uniques);

        if bloc.len() == bloc_uniques {
            println!("found sequence after {} characters", end);
            break;
        } 
    }

}


#[derive(Copy, Clone)]
struct NodeInfo<'a> {
    name: &'a str, 
    size: usize}

impl fmt::Display for NodeInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node: {}, size: {}", self.name, self.size)
    }
}
impl PartialEq for NodeInfo<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name) && self.size == other.size
    }
}

pub fn day7(){

    let contents = fs::read_to_string("input/07-input").expect("problem with the file");
    let instructions: Vec<&str> = contents.split("\n").collect();
    let arena = &mut Arena::new();    
    let mut pointer = arena.new_node(NodeInfo{name:"/",size:0}); // root
    let root = pointer.clone();

    for inst in instructions {

        if inst.starts_with("$ cd"){
                // got to dir
            let dir_dest = inst.strip_prefix("$ cd ").unwrap().trim();

            match dir_dest {
                ".." =>  { 
                        pointer = arena.get(pointer).unwrap().parent().unwrap();
                }, //move one up
                "/" => {}, // go to root
                _ => { // got to dir_dest
                    pointer = pointer.children(&arena).filter(|child| 
                        arena.get(*child).unwrap().get().name.eq(dir_dest)).next().unwrap();
                }, 
            }

        } else if inst.starts_with("dir") {
            let dir_name = inst.strip_prefix("dir ").unwrap().trim();
                let dir = arena.new_node(NodeInfo{name: dir_name, size:0});
                pointer.append(dir, arena)//create a dir
        } else if inst.starts_with("$ ls") {
                // following lines are in current node
        } else {
            // this is a file ie. 324234 b.txt
            let temp_vec = inst.split(" ").collect::<Vec<&str>>();
            let file_size:usize =  FromStr::from_str(temp_vec.first().unwrap()).unwrap();
            let file_name = temp_vec.last().unwrap();
            let file = arena.new_node(NodeInfo{name: file_name, size: file_size});
            pointer.append(file, arena);

            let oldies = pointer.ancestors(&arena).collect::<Vec<NodeId>>();
            for ancestor in oldies {
                let node_info = arena.get_mut(ancestor).unwrap().get_mut();
                node_info.size = node_info.size + file_size;
            }
        }
    }
    println!("dir tree is\n {}", root.debug_pretty_print(&arena));
    let traverser = root.traverse(arena);
    
    let mut dirs: Vec<NodeInfo> = Vec::new();
    for node_e in traverser {
        let n =  match node_e {
            NodeEdge::Start(node_info) => node_info,
            NodeEdge::End(node_info) => node_info,
        };
        let node_info = arena.get(n).unwrap().get();
        let node_name = node_info.name;
        let node_size = node_info.size;
        
        if !dirs.contains(node_info) && n.children(arena).count() > 0 { // &&  node_size <=100000{  // enable for part 1
            println!("adding {}", node_info);
            dirs.push(*node_info);
        }
    }

    let all_smalls = dirs.iter()
                .fold(0,|accum, node| accum + node.size);

    println!("Big number is {}", all_smalls);

    let space_taken = arena.get(root).unwrap().get().size;
    let free_space: usize = 70000000 - space_taken;
    let space_needed: usize = 30000000 - free_space;
    println!("Free space {}, space needed {}", free_space, space_needed); 

    let candidates = dirs.iter().filter(|node| node.size > space_needed);
    let min = candidates.min_by(|x,t|  x.size.cmp(&t.size)).unwrap(); 
    println!("best option is {}", min);

}

pub fn day8_p1(){
    let contents = fs::read_to_string("input/08-sample").expect("problem with the file");    

    let forest = contents.lines()
                .map(|line| line.chars()
                        .map(|c|  c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    // println!("forest is {}", forest.len());

    let mut visible_trees: usize = 0; //forest.len() + forest[0].len(); // start with the perimeter
    println!("Perimeter trees (visible) {visible_trees}");

    //Part 1 
    let mut scores :Vec<u32> = Vec::new();
    for (i, tree_row) in forest.iter().enumerate(){

        if i ==0 || i == forest.len()-1 {
       //     println!("Skipping checking treeline {}, all trees are visible", i);
            visible_trees = visible_trees + tree_row.len();
            continue;
        }

       
        for (j,tree) in tree_row.iter().enumerate() {

            let mut north_score = 0;
            if j == 0 || j == tree_row.len()-1 {
       //         println!("Skipping check for tree {i},{j} w height {tree}, always visible");
                visible_trees = visible_trees + 1;
            } else{
                println!("Checking tree {i},{j} w height {tree}");

                //check north view
                let north_view  = forest[..i].iter()
                       .map(|v_row| v_row.get(j).unwrap())
                               .filter(|v_tree| *v_tree  >= tree ).count() == 0;

                println!("indexes i:{i} j:{j}");
                let north_score_view = forest[..i].iter()

                .map(|v_row| v_row.get(j).unwrap());


                if i == 1 {
                    north_score = 1; //  only have one tree to see no matter the heigh 
                }
                else{
                   for (index, x) in north_score_view.enumerate().rev() {
                        println!("{}:vec x is {}",index, x);
                        
                        north_score = north_score +1;
                        if x>= tree {
                            break;
                        }
                        
                    }
                }
                scores.push(north_score);

                if north_view { 
          //          println!(" -> visible from north");
                    visible_trees = visible_trees + 1;
                    continue;
                }

                let south_view = forest[(
                    i+1)..].iter()
                        .map(|v_row| v_row.get(j).unwrap())
                            .filter(|v_tree| *v_tree  >= tree ).count() == 0 ;

                if south_view {
        //            println!(" -> visible from south"); 
                    visible_trees = visible_trees + 1;
                    continue;
                }

                let _temp_west = tree_row[..j].to_vec();
                let west_view = tree_row[..j].iter().filter(|t| *t >= tree).count() == 0;

                if west_view {
       //             println!(" -> visible from west");
                    visible_trees = visible_trees + 1;
                    continue;
                }

                let _temp = tree_row[(j+1)..].to_vec();
                let east_view = tree_row[(j+1)..].iter().filter(|t| *t >= tree).count() == 0 ;

                if east_view {
        //            println!(" -> visible from east");
                    visible_trees = visible_trees + 1;
                    continue;
                }
            }
        }
    }
    println!("All Visible trees {visible_trees}");
}

pub fn day8_p2(){
    let contents = fs::read_to_string("input/08-input").expect("problem with the file");    

    let forest = contents.lines()
                .map(|line| line.chars()
                        .map(|c|  c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();



    //Part 2
    let mut scores :Vec<u32> = Vec::new();
    for (i, tree_row) in forest.iter().enumerate(){
       
        let mut tree_score: u32 = 0;
        for (j,tree) in tree_row.iter().enumerate() {

            println!("indexes i:{i} j:{j}");

            //check the north
            let mut north_score = 0;
            let north_score_view = forest[..i].iter()
                .map(|v_row| v_row.get(j).unwrap());

            if i == 1 || i == 0{
                north_score = 1; //  only have one tree to see no matter the heigh 
            } else{
                for (index, x) in north_score_view.enumerate().rev() {
                    println!("{}:vec north x is {}",index, x);    
                    north_score = north_score +1;
                    if x>= tree {
                        break;
                    }
                }
            }

            //check the south
            let mut south_score: u32 = 0;
            let south_score_view = forest[i+1..].iter()
            .map(|v_row| v_row.get(j).unwrap());
            
            if i == forest.len()-1 || i == forest.len() -2 {
                south_score = 1;
            } else{
                for (index, x) in south_score_view.enumerate() {
                    println!("{}:vec south x is {}",index, x);    
                    south_score = south_score +1;
                    if x>= tree {
                        break;
                    }
                }
            }

            //check the west
            let west_score_view = tree_row[..j].iter();
            let mut west_score: u32 = 0;
            if j == 0 {
                west_score = 1;
            }else {
                for (index, x) in west_score_view.rev().enumerate(){
                    println!("{}:vec west x is {}",index, x);    
                    west_score = west_score +1;
                    if x>= tree {
                        break;
                    }  
                }
            }

            //check the east
            let east_score_view = tree_row[j+1..].iter();
            let mut east_score: u32 = 0;
            if j == tree_row.len()-1 {
                east_score = 1;
            } else {
                for (index, x) in east_score_view.enumerate(){
                    println!("{}:vec east x is {}",index, x);    
                    east_score = east_score +1;
                    if x>= tree {
                        break;
                    } 
                }
            }

            tree_score = north_score * south_score * west_score * east_score;
            scores.push(tree_score);
 
        }
    }
    println!("Max scores is {}", scores.iter().max().unwrap());
}