use std::collections::HashMap;

pub struct Node <'a> {
    pub name: &'a str,
    pub lat: f32,
    pub long: f32,
    pub neighbors_names: Vec<&'a str>,
    pub neighbors_dist: Vec<u32>,
}

impl Node <'static>{
    pub fn add_neighbor (&mut self, neighbor: &Node<'static>, dist: u32){
        self.neighbors_names.push(neighbor.name);
        self.neighbors_dist.push(dist);
    }

    pub fn get_neighbors (&self) -> Vec<&'static str> {
        return self.neighbors_names.clone();
    }

    pub fn get_dist_to(&self, who: &'static str) -> u32{
        for (ii, _who) in self.neighbors_names.clone().iter().enumerate() {
            if *_who == who {
                return self.neighbors_dist[ii];
            }
        }
        return 0;
    }

    pub fn new(name: &'static str, lat: f32, long: f32) -> Node<'static> {
        return Node {name: name, lat: lat, long: long,
                     neighbors_names: Vec::new(), neighbors_dist: Vec::new() };
    }
}

pub fn construct_graph() -> HashMap<&'static str,  Node<'static>> {
    let mut ny = Node::new("New York", 40.730610, 73.935242);
    let mut la = Node::new("Los Angeles", 34.052235, 118.243683);
    let mut chicago = Node::new("Chicago", 41.881832, 87.623177);
    let mut minneapolis = Node::new("Minneapolis", 44.986656, 93.258133);
    let mut denver = Node::new("Denver", 39.742043, 104.991531);;
    let mut dallas = Node::new("Dallas", 32.897480, 97.040443);
    let mut seattle = Node::new("Seattle", 47.608013, 122.335167);
    let mut boston = Node::new("Boston", 42.361145, 71.057083 );
    let mut sf = Node::new("San Francisco",37.7749, 122.4194  );
    let mut sl = Node::new("St. Louis", 38.627003, 90.199402);
    let mut houston = Node::new("Houston", 29.7604, 95.3698);
    let mut phoenix = Node::new("Phoenix", 33.448376, 112.074036);
    let mut slc = Node::new("Salt Lake City", 40.7608, 111.8910);


    ny.add_neighbor(&chicago, 713);
    ny.add_neighbor(&minneapolis, 1018);
    ny.add_neighbor(&dallas, 1374);
    ny.add_neighbor(&boston, 213);
    ny.add_neighbor(&sl, 875);

    la.add_neighbor(&denver, 831);
    la.add_neighbor(&dallas, 1240);
    la.add_neighbor(&seattle, 959);
    la.add_neighbor(&sf, 403);
    la.add_neighbor(&houston, 1374);
    la.add_neighbor(&phoenix, 357);
    la.add_neighbor(&slc, 579);

    chicago.add_neighbor(&ny, 713);
    chicago.add_neighbor(&minneapolis, 355);
    chicago.add_neighbor(&denver, 920);
    chicago.add_neighbor(&dallas, 803);
    chicago.add_neighbor(&boston, 851);
    chicago.add_neighbor(&sl, 262);
    chicago.add_neighbor(&houston, 940);

    minneapolis.add_neighbor(&ny, 1018);
    minneapolis.add_neighbor(&chicago, 355);
    minneapolis.add_neighbor(&denver, 700);
    minneapolis.add_neighbor(&seattle, 1395);
    minneapolis.add_neighbor(&boston, 1123);
    minneapolis.add_neighbor(&sl, 466);
    minneapolis.add_neighbor(&slc, 987);

    denver.add_neighbor(&la, 831);
    denver.add_neighbor(&chicago, 920);
    denver.add_neighbor(&minneapolis, 700);
    denver.add_neighbor(&dallas, 663);
    denver.add_neighbor(&seattle, 1021);
    denver.add_neighbor(&sf, 949);
    denver.add_neighbor(&sl, 796);
    denver.add_neighbor(&houston, 879);
    denver.add_neighbor(&phoenix, 586);
    denver.add_neighbor(&slc, 371);

    dallas.add_neighbor(&ny, 1374);
    dallas.add_neighbor(&la, 1240);
    dallas.add_neighbor(&chicago, 803);
    dallas.add_neighbor(&denver, 663);
    dallas.add_neighbor(&sl, 547);
    dallas.add_neighbor(&houston, 225);
    dallas.add_neighbor(&phoenix, 887);
    dallas.add_neighbor(&slc, 999);

    seattle.add_neighbor(&la, 959);
    seattle.add_neighbor(&minneapolis, 1395);
    seattle.add_neighbor(&denver, 1021);
    seattle.add_neighbor(&sf, 678);
    seattle.add_neighbor(&phoenix, 1114);
    seattle.add_neighbor(&slc, 701);


    boston.add_neighbor(&ny, 213);
    boston.add_neighbor(&chicago, 851);
    boston.add_neighbor(&minneapolis, 1123);
    boston.add_neighbor(&sl, 1038);

    sf.add_neighbor(&la, 403);
    sf.add_neighbor(&denver, 949);
    sf.add_neighbor(&seattle, 678);
    sf.add_neighbor(&houston, 1645);
    sf.add_neighbor(&phoenix, 653);
    sf.add_neighbor(&slc, 600);

    sl.add_neighbor(&ny, 875);
    sl.add_neighbor(&chicago, 262);
    sl.add_neighbor(&minneapolis, 466);
    sl.add_neighbor(&denver, 796);
    sl.add_neighbor(&dallas, 547);
    sl.add_neighbor(&boston, 1038);
    sl.add_neighbor(&houston, 679);
    sl.add_neighbor(&phoenix, 1272);
    sl.add_neighbor(&slc , 1162);

    houston.add_neighbor(&la, 1374);
    houston.add_neighbor(&chicago, 940);
    houston.add_neighbor(&denver, 879);
    houston.add_neighbor(&dallas, 225);
    houston.add_neighbor(&sf, 1645);
    houston.add_neighbor(&sl, 679);
    houston.add_neighbor(&slc, 1200);

    phoenix.add_neighbor(&la, 357);
    phoenix.add_neighbor(&denver, 586);
    phoenix.add_neighbor(&dallas, 887);
    phoenix.add_neighbor(&seattle, 1114);
    phoenix.add_neighbor(&sf, 653);
    phoenix.add_neighbor(&sl, 1272);
    phoenix.add_neighbor(&slc, 504);

    slc.add_neighbor(&la, 579);
    slc.add_neighbor(&minneapolis, 987);
    slc.add_neighbor(&denver, 371);
    slc.add_neighbor(&dallas, 999);
    slc.add_neighbor(&seattle, 701);
    slc.add_neighbor(&sf, 600);
    slc.add_neighbor(&sl, 1162);
    slc.add_neighbor(&houston, 1200);
    slc.add_neighbor(&phoenix, 504);

    let mut node_map: HashMap<&'static str, Node> = HashMap::new();

    node_map.insert(ny.name, ny);
    node_map.insert(la.name, la);
    node_map.insert(chicago.name, chicago);
    node_map.insert(minneapolis.name, minneapolis);
    node_map.insert(denver.name, denver);
    node_map.insert(dallas.name, dallas);
    node_map.insert(seattle.name, seattle);
    node_map.insert(boston.name, boston);
    node_map.insert(sf.name, sf);
    node_map.insert(sl.name, sl);
    node_map.insert(houston.name, houston);
    node_map.insert(phoenix.name, phoenix);
    node_map.insert(slc.name, slc);

    return node_map;
}

fn print_node(node: & Node){
    println!("{}: {} N, {} W", node.name, node.lat, node.long);
    for (ii, name) in node.neighbors_names.clone().iter().enumerate() {
        println!("{} - {}", name, node.neighbors_dist[ii]);
    }
}

pub fn print_node_map(node_map: HashMap<&str, Node>){
    for (name, node) in &node_map {
        print_node(node);
        println!();
    }
}
