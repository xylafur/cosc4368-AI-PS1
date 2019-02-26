//use nodes;

use std::collections::HashMap;

mod modules;
use modules::search::*;
use modules::nodes::*;

fn main(){
    let node_map = construct_graph();
    print_node_map(node_map);
}
