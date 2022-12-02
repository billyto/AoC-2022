use std::fs;
use regex::Regex;


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
      //  println!("points per game {} ", score);

        scores.push(score);

    }

    let total_score: usize = scores.iter().sum();

    print!("Total score is {}",total_score); //8933

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

