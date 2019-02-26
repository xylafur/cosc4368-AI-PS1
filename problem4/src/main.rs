//use nodes;

use std::collections::HashMap;

mod modules;
use modules::search::*;
use modules::nodes::*;

/*
 *  1. Minneapolis to Houston
 *  2. San Francisco to Chicago
 *  3. New York City to Los Angeles
 */

fn print_result(result: Vec<&'static str>){
    for each in result {
        println!("{}", each);
    }
    println!("");
}

fn main(){
    let node_map = construct_graph();
    //print_node_map(node_map);
    print_result(a_star("Minneapolis", "Houston", &node_map));
    print_result(a_star("San Francisco", "Chicago", &node_map));
    print_result(a_star("New York", "Los Angeles", &node_map));
}
