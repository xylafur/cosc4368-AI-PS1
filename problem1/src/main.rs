//use nodes;

use std::collections::HashMap;

mod modules;
use modules::search::*;
use modules::nodes::*;

fn main(){
    let node_map: HashMap<&str, Node> = obtain_nodes();
    let goals = get_goal_states();
    bfs("S", goals, node_map);
    println!("");

    let node_map: HashMap<&str, Node> = obtain_nodes();
    let goals = get_goal_states();
    dfs("S", goals, node_map);
    println!("");

    let node_map: HashMap<&str, Node> = obtain_nodes();
    let goals = get_goal_states();
    bestfs("S", goals, node_map);
    println!("");

    let node_map: HashMap<&str, Node> = obtain_nodes();
    let goals = get_goal_states();
    A_star("S", goals, node_map);
    //SMA_start("S", goals, node_map);

}
