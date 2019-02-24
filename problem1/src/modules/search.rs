extern crate priority_queue;
use priority_queue::PriorityQueue;

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
        "dfs" | "bestfs" => {
            let mut open_cpy = open.clone();
            open_cpy.push(name);
            return open_cpy
        },
        _ => return Vec::new(),
    }
}

/*  In this function we find the next node that we want to explore based on the
 *  particular search function that we are using.  We return the name of that
 *  node and remove it from the open list if we find it.
 *
 *  If we do not find a valid node (because the open list is empty) then we
 *  return an empty string.  In this case, the open list is left unchanged
 */
fn choose_node(search_func: &'static str, open_list: &mut Vec<&'static str>,
               node_map: &HashMap<&'static str, Node<'static>>)
                    -> &'static str {
    match search_func {
        "bfs" | "dfs" => {
        //Get the next element from the open list
            let top = open_list.pop();
            //If there are no elements, then we didn't find a goal!
            if top == None{
                return "";
            }

            //Is this node a goal state?  If so say so and return true
            return top.unwrap_or("");
        },
        "bestfs" => {
            if open_list.len() == 0 {
                //println!("Found empty open list!");
                return "";
            }
            let mut min_name: &'static str = node_map[open_list[0]].name;
            let mut min_val: u32 = node_map[open_list[0]].value;
            let mut min_index: u32 = 0;
            let mut ii = 0;

            for (ii, each) in open_list.clone().iter().enumerate() {
                if node_map[each].value < min_val {
                    min_index = ii as u32;
                    min_val = node_map[each].value;
                    min_name = each;
                }
            }
            //println!("Found min: {}", min_name);

            return min_name;

        },
        _ => return "",
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
        //chose the next node based on our search function
        let current = choose_node(search_func, &mut open, &node_map);
        if current == "" {
            ret = false;
            break;
        }

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

fn remove_from_vec(vector: &mut Vec<&'static str>, which: & &'static str){
    for (ii, node) in vector.clone().iter().enumerate() {
        if node == which {
            vector.remove(ii);
            break;
        }
    }
}

fn queue_empty(queue: & PriorityQueue<&'static str, i32>) -> bool {
    if let Some(ref top) = queue.peek() {
        return false;
    } else {
        return true;
    }
}

fn print_queue(queue: &PriorityQueue<&'static str, i32>){
    for each in queue.clone().into_sorted_iter() {
        println!("{}: {}", each.1, each.0);
    }
}

fn A_star_search(start_node: &'static str, goal_states: Vec<&'static str>,
                 search_func: &'static str,
                 node_map: HashMap<&'static str, Node<'static>>) -> bool {

    /*  Put the starting node into our priority queue
     *  keep track of who each node came from, and the total cost up to this node
     *
     *  while our prioirity queue is not empty
     *      get the next from the prioirity queue
     *
     *      if its a goal state return
     *
     *      iterate through each of this current node's neighbors
     *          if we haven't added it to our cost so far dict or we have and
     *          this distance is less, add it to the queue
     *              the cost for the priority queue is the cost to that node
     *              plus the supposed cost to the goal node
     */

    let mut expanded: Vec<&'static str> = Vec::new();
    let mut closed: Vec<&'static str> = Vec::new();
    let mut open = node_map.keys().cloned().filter(|x| *x == start_node)
                 .collect::<Vec<&'static str>>();


    let mut queue: PriorityQueue<&'static str, i32> =  PriorityQueue::new();
    let mut cost: HashMap<&'static str, u32> = HashMap::new();
    let mut origin: HashMap<&'static str, &'static str> = HashMap::new();
    let mut ret = false;

    //I am pushing the negative of the number, just because the priority queue
    //that I'm using pops off the maximum (I guess as a priority queue should)
    //and can't be switched.  To get around that just make all the values
    //negative and now the lowest number (the one we want) is actually the greatest!
    queue.push(start_node, -(node_map[start_node].value as i32));
    cost.insert(start_node, 0);
    origin.insert(start_node, "");

    while ! queue_empty(& queue){
        //print_queue(&queue);

        let current = queue.pop().unwrap();
        let current_node: &'static str = current.0;

        //remove our selected node (highest priority) from the open list
        remove_from_vec(&mut open, & current_node);

        if goal_states.contains(&current_node) {
            println!("Encountered Goal state {}", current_node);
            ret = true;
            break;
        }

        //I still need this guy.. so lets just clone him
        closed.push(current_node.clone());

        println!("Expanding node {}", current_node);

        //There must be a more rustian solution, but all I know is this works
        for child_info in node_map[current_node].children_info.clone() {
            let child_cost: u32 = child_info.0;
            let child_name: &'static str =  child_info.1;

            // if the child isn't in the open and closed sets
            if ! open.contains(& child_name) && ! closed.contains(& child_name) {
                queue.push(child_name,
                           -((cost[current_node] + child_cost +
                              node_map[child_name].value) as i32));
                open.push(child_name);
                cost.insert(child_name, cost[current_node] + child_cost);
                origin.insert(child_name, current_node);
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

pub fn bestfs(start_node: &'static str, goal_states: Vec<&'static str>,
           node_map: HashMap<&'static str, Node<'static>>) -> bool {
    println!("Running Best First Search!");

    return generic_search(start_node, goal_states, "bestfs", node_map);
}

pub fn A_star(start_node: &'static str, goal_states: Vec<&'static str>,
           node_map: HashMap<&'static str, Node<'static>>) -> bool {
    println!("Running A* search");
    return A_star_search(start_node, goal_states, "A*", node_map);
}

pub fn SMA_star(start_node: &'static str, goal_states: Vec<&'static str>,
           node_map: HashMap<&'static str, Node<'static>>) -> bool {
    println!("Running SMA* search");

    return false;
}
