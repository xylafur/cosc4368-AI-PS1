//use nodes;

use std::collections::HashMap;

mod modules;
use modules::search::*;
use modules::nodes::*;

fn main(){
    let node_map: HashMap<&str, Node> = obtain_nodes();
    let goals = get_goal_states();
    //modules::nodes::print_node_map(node_map);

    //bfs("S", goals, node_map);
    //dfs("S", goals, node_map);
    bestfs("S", goals, node_map);

}
