use std::fs;
use indextree::{Arena, NodeEdge, NodeId};
use std::str::FromStr;
use std::fmt;

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

fn main() {

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
