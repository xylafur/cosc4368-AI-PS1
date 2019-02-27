extern crate priority_queue;
use priority_queue::PriorityQueue;

use std::collections::HashMap;
use crate::modules::nodes::*;

fn get_path_from_origin_map(goal: &'static str, start: &'static str,
                            origin: &HashMap<&'static str, &'static str>)
                                -> Vec<&'static str> {
    let mut current = goal;
    let mut ret = Vec::new();
    while current != start {
        ret.push(current.clone());
        current = origin[current];
    }
    ret.push(current.clone());
    return ret;
}

fn queue_empty(queue: & PriorityQueue<&'static str, i64>) -> bool {
    if let Some(ref top) = queue.peek() {
        return false;
    } else {
        return true;
    }
}

pub fn a_star(start: &'static str, end: &'static str,
              node_map: &HashMap<&'static str, Node<'static>>) -> Vec<&'static str> {

    let mut queue: PriorityQueue<&'static str, i64> =  PriorityQueue::new();
    let mut cost: HashMap<&'static str, u32> = HashMap::new();
    let mut origin: HashMap<&'static str, &'static str> = HashMap::new();

    let mut visited = Vec::new();

    queue.push(start, 0);
    cost.insert(start, 0);
    origin.insert(start, "");

    while ! queue_empty(& queue){
        //print_queue(&queue);

        let current_tuple = queue.pop().unwrap();
        let current: &'static str = current_tuple.0;

        visited.push(current);

        if current == end {
            break;
        }

        for (ii, neighbor_name) in node_map[current].neighbors_names.clone().iter().enumerate() {
            let neighbor_dist = node_map[current].neighbors_dist[ii];

            let this_cost = cost[current] + neighbor_dist +
                            node_map[neighbor_name].heuristic(&node_map[end]);
            // If we haven't visited this guy
            // or if we found a better route to this guy
            if ! visited.contains(neighbor_name) || this_cost < cost[neighbor_name]{
                origin.insert(neighbor_name, current);
                cost.insert(neighbor_name, this_cost);
                queue.push(neighbor_name, -(this_cost as i64) );
            }
        }
    }

    println!("Cost: {}", cost[end]);

    return get_path_from_origin_map(end, &start, &origin);
}
