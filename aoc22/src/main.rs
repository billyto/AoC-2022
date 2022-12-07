use std::fs;
use indextree::Arena;
use std::str::FromStr;
use std::fmt;

// pub mod aoc;


struct NodeInfo<'a> {
    name: &'a str, 
    size: usize}

impl fmt::Display for NodeInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node: {}, size: {}", self.name, self.size)
    }
}

fn main() {

    let contents = fs::read_to_string("input/07-sample").expect("problem with the file");
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

        }


    }
    println!("dir tree is\n {}", root.debug_pretty_print(&arena));

}


