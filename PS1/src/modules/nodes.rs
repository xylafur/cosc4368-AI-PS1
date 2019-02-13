use std::collections::HashMap;

fn get_node_defs() -> Vec<&'static str> {
    let node_defs: Vec<&'static str> = vec!["F 9 1 D 4",
                                            "S 10 3 A 5 B 8 F 9",
                                            "B 8 2 E 4 G2 9",
                                            "G2 0 1 B 8",
                                            "H 2 1 G2 1",
                                            "E 4 2 H 1 G2 5",
                                            "A 5 2 D 6 C 3",
                                            "C 3 2 S 2 D 4",
                                            "D 2 2 G1 6 B 8",
                                            "G1 0 1 C 2"
                                           ];
    return node_defs;
}

pub fn get_goal_states() -> Vec<&'static str> {
    return vec!["G1", "G2"]
}

pub struct Node <'a> {
    pub value: u32,
    pub name: &'a str,
    pub num_children: u32,
    pub children_info: Vec<(u32, &'a str)>,
}

/*  This function grabs all of the pieces from the string describing a node and
 *  then creates a node from that information
 */
fn create_node(node_str:&'static str) -> Node {
    let mut children_info: Vec<(u32, &'static str)> = Vec::new();

    let split = node_str.split(" ").collect::<Vec<&str>>();

    assert!(split.len() >= 3);

    let name: &str = split[0];
    let value: u32 = split[1].parse::<u32>().unwrap();
    let num_children: u32 = split[2].parse::<u32>().unwrap();

    //make sure that we have the exact number of parts we say we do
    assert!(split.len() == (3 + num_children * 2) as usize);

    for ii in (0..num_children).rev() {
        let name: &str = split[(ii * 2 + 3) as usize];
        let distance: u32 = split[(ii * 2 + 4) as usize].parse::<u32>().unwrap();
        children_info.push((distance, name));
    }

    return Node {value: value, name: &name, num_children: num_children,
                 children_info: children_info};
}

fn create_nodes(defs: Vec<&'static str>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for node_str in defs {
        nodes.push(create_node(node_str));
    }
    return nodes;
}

fn get_node_map(nodes: Vec<Node>) -> HashMap<&str, Node> {
    let mut node_map = HashMap::new();
    for node in nodes {
        node_map.insert(node.name, node);
    }
    return node_map;
}

fn print_node(node: & Node){
    println!("Name: {}", node.name);
    println!("Value: {}", node.value);
    println!("Showing {} children", node.num_children);
    for ii in (0..node.num_children).rev() {
        println!("    Name {}, Distance {}",
                 node.children_info[ii as usize].1,
                 node.children_info[ii as usize].0);
    }
    println!("");
}

fn print_nodes(nodes: Vec<Node>){
    for node in nodes {
        print_node(&node);
    }
}

pub fn print_node_map(node_map: HashMap<&str, Node>){
    for (name, node_info) in &node_map {
        print_node(&node_info);
    }
}

pub fn obtain_nodes() -> HashMap<&'static str, Node<'static>> {
    let node_defs: Vec<&'static str> = get_node_defs();
    let _nodes: Vec<Node> = create_nodes(node_defs);
    return get_node_map(_nodes);
}
