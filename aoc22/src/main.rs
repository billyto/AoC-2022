use std::{fs, ops::Index};

fn main() {

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

fn day8_p1(){
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
