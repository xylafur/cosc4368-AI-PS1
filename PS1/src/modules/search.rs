use std::collections::HashMap;
use crate::modules::nodes::*;

/*
 *  (a) indicate which goal state is reached if any
 *  (b) list, in order, the states expanded
 *  (c) show the final contents of the OPEN and CLOSED lists
 */

fn print_vec(vec_name: &str, vec: Vec<&str>) {
    println!("Showing contents of {}", vec_name);
    for each in vec {
        print!("{}, ", each);
    }
    println!("");
}

fn do_insert(search_func: &'static str, name : &'static str,
             open: Vec<&'static str>) -> Vec<&'static str> {
    match search_func {
        "bfs" => {
            //for breadth first search, this newly added node should be the
            //last thing that we expand.  We add it to the beginning because we
            //pop from the end
            let mut open_cpy = open.clone();
            open_cpy.insert(0, name);
            return open_cpy
        },
        //in dfs, this new node should be the next thing we expand, so we add
        //it to the end of the list since thats what we expand next
        "dfs" => {
            let mut open_cpy = open.clone();
            open_cpy.push(name);
            return open_cpy
        },
        _ => return Vec::new(),
    }
}

fn generic_search(start_node: &'static str, goal_states: Vec<&'static str>,
                  search_func: &'static str,
                  node_map: HashMap<&'static str, Node<'static>>) -> bool {

    let mut expanded: Vec<&'static str> = Vec::new();
    let mut closed: Vec<&'static str> = Vec::new();
    let mut open = node_map.keys().cloned().filter(|x| *x == start_node)
                 .collect::<Vec<&'static str>>();

    let mut ret: bool = true;

    while open.len() > 0 {
        //Get the next element from the open list
        let top = open.pop();
        //If there are no elements, then we didn't find a goal!
        if top == None{
            ret = false;
            break;
        }

        //Is this node a goal state?  If so say so and return true
        let current: &'static str = top.unwrap_or("");
        println!("Current: {}", current);
        if goal_states.contains(&current) {
            println!("Encountered Goal state {}", current);
            ret = true;
            break;
        }

        //It wasn't a goal state, lets expand
        expanded.push(current.clone());
        println!("Expanding {}", current);

        closed.push(current.clone());

        for (value, name) in node_map[current].children_info.clone() {
            if ! closed.contains(&name) && ! open.contains(&name) {
                open = do_insert(search_func, name, open);
            }
        }
    }
    print_vec("open", open);
    print_vec("closed", closed);
    return ret;
}

pub fn bfs(start_node: &'static str, goal_states: Vec<&'static str>,
           node_map: HashMap<&'static str, Node<'static>>) -> bool {
    println!("Running Breadth First Search!");

    return generic_search(start_node, goal_states, "bfs", node_map);
}

pub fn dfs(start_node: &'static str, goal_states: Vec<&'static str>,
           node_map: HashMap<&'static str, Node<'static>>) -> bool {
    println!("Running Depth First Search!");

    return generic_search(start_node, goal_states, "dfs", node_map);
}
