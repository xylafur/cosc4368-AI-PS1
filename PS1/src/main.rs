//use nodes;

use std::collections::HashMap;

mod modules;
use modules::nodes::*;

fn main(){
    let node_map: HashMap<&str, modules::nodes::Node> = modules::nodes::obtain_nodes();
    modules::nodes::print_node_map(node_map);
}
